use std::iter::FromIterator;
use word::slugify;
use csv;
use std::borrow::Cow;
use std::io::{Read, BufReader};
use std::ops::DerefMut;
use std::fs::File;
use std::path::PathBuf;
use search::SearchResult;
use error::Result;
use word::{Text, slug_len};

pub fn load_wordlist_file(name: &str) -> Result<BufReader<File>>
{
    dotenv::dotenv().ok();
    let mut path = match dotenv::var("WORDLIST_DIR") {
        Ok(s) => PathBuf::from(s),
        Err(_) => PathBuf::new()
    };
    path.push(name);
    Ok(BufReader::new(File::open(path)?))
}

/*
 * Ideally, we would use a crate like owned_ref or rental here, but I
 * don't think either of them covers this use case.
 */
struct CsvIter<'a, R: Read + 'a>
{
    _rdr: Box<csv::Reader<R>>,
    iter: csv::DeserializeRecordsIter<'a, R, WordFreq>
}

impl<'a, R: Read + 'a> CsvIter<'a, R> {
    pub fn new(r: csv::Reader<R>) -> Self {
        let mut rdr = Box::new(r);
        let iter = unsafe { ::util::prolong_mut(rdr.deref_mut()).deserialize() };
        Self { _rdr: rdr, iter }
    }
}

impl<'a, R: Read + 'a> Iterator for CsvIter<'a, R> {
    type Item = WordFreq;

    fn next(&mut self) -> Option<WordFreq> {
        self.iter.next().map(::std::result::Result::unwrap)
    }
}

pub fn wordlist_iter<R: Read>(r: R) -> impl Iterator<Item=WordFreq>
{
    let rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(r);
    CsvIter::new(rdr)
}

/// Returns an iterator that iterates over all words in the given wordlist.
/// The iterator will panic if it fails to read or parse the file.
pub fn load_wordlist_iter(name: &str) -> Result<impl Iterator<Item=WordFreq>>
{
    let file = load_wordlist_file(name)?;
    Ok(wordlist_iter(file))
}

#[derive(Clone,Debug,Deserialize)]
pub struct WordlistEntry {
    pub word: String,
    pub slug: String,
    pub freq: u64,
}

#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct WordFreq {
    pub word: String,
    pub freq: u64,
}

impl WordFreq {
    /// The word with spaces and punctuation removed.
    /// ```
    /// use puzzletools::wordlist::WordFreq;
    /// let wf = WordFreq { word: "ASCII STRING".to_owned(), freq: 1 };
    /// assert_eq!(wf.slug(),"ASCIISTRING");
    /// ```
    pub fn slug(&self) -> Cow<str> { slugify(&self.word) }
    /// The number of letters in the word (non-alphabetic characters are not counted).
    /// ```
    /// use puzzletools::wordlist::WordFreq;
    /// let wf = WordFreq { word: "ASCII STRING".to_owned(), freq: 1 };
    /// assert_eq!(wf.len(),11);
    /// ```
    pub fn len(&self) -> usize { slug_len(&self.word) }
}

impl SearchResult for WordFreq {
    type Data = String;
    type Freq = u64;
    fn data(&self) -> String { self.word.clone() }
    fn freq(&self) -> u64 { self.freq }
}

impl<'a> SearchResult for &'a WordFreq {
    type Data = &'a str;
    type Freq = u64;
    fn data(&self) -> &'a str { &self.word }
    fn freq(&self) -> u64 { self.freq }
}

impl From<WordFreq> for WordlistEntry {
    fn from(wf: WordFreq) -> Self {
        let slug = slugify(&wf.word).to_string();
        WordlistEntry {
            word: wf.word,
            slug: slug,
            freq: wf.freq,
        }
    }
}

impl<'a> SearchResult for &'a WordlistEntry {
    type Data = &'a str;
    type Freq = u64;
    fn data(&self) -> &'a str {
        &self.word
    }
    fn freq(&self) -> u64 {
        self.freq
    }
}

impl WordlistEntry {
    pub fn len(&self) -> usize {
        self.slug.len()
    }
}

macro_rules! pair_search_result_impl {
    () => (
        type Data = String;
        type Freq = u128;
        fn data(&self) -> String {
            format!("{}, {}",self.0.word,self.1.word)
        }
        fn freq(&self) -> u128 { (self.0.freq as u128) * (self.1.freq as u128) }
    )
}

impl<'a, 'b> SearchResult for (&'a WordlistEntry, &'b WordlistEntry) {
    pair_search_result_impl!();
}

impl<'a> SearchResult for (WordFreq, &'a WordlistEntry) {
    pair_search_result_impl!();
}

impl<'a, 'b> SearchResult for (&'a WordFreq, &'b WordlistEntry) {
    pair_search_result_impl!();
}


pub struct Wordlist {
    entries: Vec<WordlistEntry>,
    lookup: fnv::FnvHashMap<&'static [u8], u32>,
}

impl Wordlist {
    pub fn get<S: Text>(&self, s: S) -> Option<&WordlistEntry>
    {
        self.lookup.get(s.as_bytes()).map(
            |&n| &self.entries[n as usize],
        )
    }

    pub fn freq<S: Text>(&self, s: S) -> u64
    {
        self.lookup.get(s.as_bytes()).map_or(0, |&n| {
            self.entries[n as usize].freq
        })
    }

    pub fn load_from_reader<R: Read>(r: R) -> Result<Self> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(r);
        let res: ::std::result::Result<Self, csv::Error> =
            rdr.deserialize::<WordFreq>().collect();
        Ok(res?)
    }

    pub fn load(list_name: &str) -> Result<Self> {
        let r = load_wordlist_file(list_name)?;
        Self::load_from_reader(r)
    }

}

impl<'a> IntoIterator for &'a Wordlist {
    type Item = &'a WordlistEntry;
    type IntoIter = ::std::slice::Iter<'a, WordlistEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter()
    }
}

impl FromIterator<WordlistEntry> for Wordlist {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = WordlistEntry>,
    {
        let ent: Vec<_> = iter.into_iter().collect();
        let lookup = ent.iter().enumerate().map(|(n,wf)| {
            let uent = unsafe { ::util::prolong(wf.slug.as_ref()) };
            (uent, n as u32)
        }).collect();
        Wordlist {
            entries: ent,
            lookup: lookup,
        }
    }
}

impl FromIterator<WordFreq> for Wordlist {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = WordFreq>,
    {
        iter.into_iter()
            .map::<WordlistEntry, _>(Into::into)
            .collect()
    }
}

pub fn pairs<'a,I,F,E,W>(list1: I, list2: &'a Wordlist, mut trans: F)
    -> impl Iterator<Item = (E, &'a WordlistEntry)>
where
    I: IntoIterator<Item = E>,
    F: FnMut(&E) -> W,
    W: Text,
{
    list1.into_iter().filter_map(move |w1| {
        list2.get(trans(&w1).as_bytes()).map(|w2| (w1,w2))
    })
}
