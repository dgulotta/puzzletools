//! Utilities for searching or iterating through a word list.
#![allow(clippy::len_without_is_empty)]
use crate::error::Result;
use crate::search::SearchResult;
use crate::word::{slug_len, slugify, Text};
use std::borrow::Cow;
use std::fs::File;
use std::hash::BuildHasher;
use std::io::{BufReader, Read};
use std::iter::FromIterator;
use std::path::PathBuf;

pub fn load_wordlist_file(name: &str) -> Result<BufReader<File>> {
    dotenvy::dotenv().ok();
    let mut path = dotenvy::var("WORDLIST_DIR").map_or_else(|_| PathBuf::new(), PathBuf::from);
    path.push(name);
    Ok(BufReader::new(File::open(path)?))
}

struct CsvIter<R: Read>(csv::DeserializeRecordsIntoIter<R, WordFreq>);

impl<R: Read> Iterator for CsvIter<R> {
    type Item = WordFreq;

    fn next(&mut self) -> Option<WordFreq> {
        self.0.next().map(::std::result::Result::unwrap)
    }
}

pub fn wordlist_iter<R: Read + 'static>(r: R) -> impl Iterator<Item = WordFreq> {
    let rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(r);
    CsvIter(rdr.into_deserialize())
}

/// Returns an iterator that iterates over all words in the given wordlist.
/// The iterator will panic if it fails to read or parse the file.
pub fn load_wordlist_iter(name: &str) -> Result<impl Iterator<Item = WordFreq>> {
    let file = load_wordlist_file(name)?;
    Ok(wordlist_iter(file))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct WordlistEntry {
    /// The word, including spaces and punctuation.
    pub word: String,
    /// The word, with spaces and punctuation removed.
    pub slug: String,
    /// The frequency of the word in the wordlist.
    pub freq: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    pub fn slug(&self) -> Cow<str> {
        slugify(&self.word)
    }
    /// The number of letters in the word (non-alphabetic characters are not counted).
    /// ```
    /// use puzzletools::wordlist::WordFreq;
    /// let wf = WordFreq { word: "ASCII STRING".to_owned(), freq: 1 };
    /// assert_eq!(wf.len(),11);
    /// ```
    pub fn len(&self) -> usize {
        slug_len(&self.word)
    }
}

impl SearchResult for WordFreq {
    type Data = String;
    type Freq = u64;
    fn data(&self) -> String {
        self.word.clone()
    }
    fn freq(&self) -> u64 {
        self.freq
    }
}

impl<'a> SearchResult for &'a WordFreq {
    type Data = &'a str;
    type Freq = u64;
    fn data(&self) -> &'a str {
        &self.word
    }
    fn freq(&self) -> u64 {
        self.freq
    }
}

impl From<WordFreq> for WordlistEntry {
    fn from(wf: WordFreq) -> Self {
        let slug = slugify(&wf.word).to_string();
        Self {
            word: wf.word,
            slug,
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
    /// The length of the word, not counting spaces and punctuation.
    pub fn len(&self) -> usize {
        self.slug.len()
    }
}

macro_rules! pair_search_result_impl {
    () => {
        type Data = String;
        type Freq = u128;
        fn data(&self) -> String {
            format!("{}, {}", self.0.word, self.1.word)
        }
        fn freq(&self) -> u128 {
            (self.0.freq as u128) * (self.1.freq as u128)
        }
    };
}

impl SearchResult for (&WordlistEntry, &WordlistEntry) {
    pair_search_result_impl!();
}

impl SearchResult for (WordFreq, &WordlistEntry) {
    pair_search_result_impl!();
}

impl SearchResult for (&WordFreq, &WordlistEntry) {
    pair_search_result_impl!();
}

/// A structure that can be used to iterate over all words in a
/// wordlist, or to look up the frequency of a particular word.
///
/// If you just want to iterate over the words in a wordlist and never
/// need to do lookups, it is faster to use `wordlist_iter`.
pub struct Wordlist {
    entries: Vec<WordlistEntry>,
    // this is essentially a manually implemented IndexSet,
    // but for some reason was about 30% faster in tests
    lookup: hashbrown::HashTable<usize>,
    hasher: hashbrown::DefaultHashBuilder,
}

impl Wordlist {
    pub fn get<S: Text>(&self, s: S) -> Option<&WordlistEntry> {
        let bytes = s.as_bytes();
        let hash = self.hasher.hash_one(bytes);
        self.lookup
            .find(hash, |&n| self.entries[n].slug.as_bytes() == bytes)
            .map(|&n| &self.entries[n])
    }

    /// Returns the frequency of the given slug, or zero if the slug
    /// does not appear in the wordlist.
    /// ```
    /// use std::io::Cursor;
    /// use puzzletools::wordlist::{Wordlist};
    /// let wltext = "TWO,2";
    /// let wl = Wordlist::load_from_reader(Cursor::new(wltext)).unwrap();
    /// assert_eq!(wl.freq("TWO"), 2);
    /// assert_eq!(wl.freq("MISSING"), 0);
    /// ```
    pub fn freq<S: Text>(&self, s: S) -> u64 {
        self.get(s).map_or(0, |e| e.freq)
    }

    pub fn load_from_reader<R: Read>(r: R) -> Result<Self> {
        let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(r);
        let res: ::std::result::Result<Self, csv::Error> = rdr.deserialize::<WordFreq>().collect();
        Ok(res?)
    }

    pub fn load(list_name: &str) -> Result<Self> {
        let r = load_wordlist_file(list_name)?;
        Self::load_from_reader(r)
    }

    pub fn iter(&self) -> ::std::slice::Iter<WordlistEntry> {
        self.into_iter()
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
        let entries: Vec<_> = iter.into_iter().collect();
        let hasher = hashbrown::DefaultHashBuilder::default();
        let mut lookup = hashbrown::HashTable::with_capacity(entries.len());
        for (n, item) in entries.iter().enumerate() {
            let hash = hasher.hash_one(item.slug.as_bytes());
            lookup.insert_unique(hash, n, |&n| hasher.hash_one(entries[n].slug.as_bytes()));
        }
        Self {
            entries,
            lookup,
            hasher,
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

/// Returns pairs of words satisfying certain properties.
///
/// More specifically, this function returns pairs `(word1, word2)`
/// such that the `word1` is in `list1`, `word2` is in `list2`, and
/// `word2 = trans(word1)`.
/// ```
/// use std::io::Cursor;
/// use puzzletools::wordlist::{Wordlist, pairs};
/// let wl = "\
/// AIRS,1
/// PAIRS,1";
/// let wl = Wordlist::load_from_reader(Cursor::new(wl)).unwrap();
/// let v: Vec<_> = pairs(wl.iter(), &wl, |w| &w.slug[1..]).collect();
/// assert_eq!(&v, &[(wl.get("PAIRS").unwrap(), wl.get("AIRS").unwrap())]);
/// ```
pub fn pairs<I, F, W>(
    list1: I,
    list2: &Wordlist,
    mut trans: F,
) -> impl Iterator<Item = (I::Item, &WordlistEntry)>
where
    I: IntoIterator,
    F: FnMut(&I::Item) -> W,
    W: Text,
{
    list1
        .into_iter()
        .filter_map(move |w1| list2.get(trans(&w1).as_bytes()).map(|w2| (w1, w2)))
}

/// Returns pairs of words satisfying certain properties.
///
/// More specifically, this function returns pairs `(word1, word2)`
/// such that the `word1` is in `list1`, `word2` is in `list2`, and
/// `trans(word1) == Some(word2)`.
/// ```
/// use std::io::Cursor;
/// use puzzletools::wordlist::{Wordlist, WordlistEntry, pairs_filter};
/// let wl = "\
/// AIRS,1
/// PAIRS,1";
/// let wl = Wordlist::load_from_reader(Cursor::new(wl)).unwrap();
/// fn pred<'a>(w: &&'a WordlistEntry) -> Option<&'a str> {
///     if w.len() > 0 { Some(&w.slug[1..]) }
///     else { None }
/// };
/// let v: Vec<_> = pairs_filter(wl.iter(), &wl, pred).collect();
/// assert_eq!(&v, &[(wl.get("PAIRS").unwrap(), wl.get("AIRS").unwrap())]);
/// ```
pub fn pairs_filter<I, F, W>(
    list1: I,
    list2: &Wordlist,
    mut trans: F,
) -> impl Iterator<Item = (I::Item, &WordlistEntry)>
where
    I: IntoIterator,
    F: FnMut(&I::Item) -> Option<W>,
    W: Text,
{
    list1
        .into_iter()
        .filter_map(move |w1| trans(&w1).and_then(|e| list2.get(e.as_bytes()).map(|w2| (w1, w2))))
}

/// Returns pairs of words satisfying certain properties.
///
/// More specifically, this function returns pairs `(word1, word2)`
/// such that the `word1` is in `list1`, `word2` is in `list2`, and
/// `word2` is one of the elements of the iterator `trans(word1)`.
/// ```
/// use std::io::Cursor;
/// use puzzletools::wordlist::{Wordlist, pairs_iter};
/// let wltext = "\
/// AIRS,1
/// PAIRS,1";
/// let wl = Wordlist::load_from_reader(Cursor::new(wltext)).unwrap();
/// let v: Vec<_> = pairs_iter(wl.iter(), &wl, move |w| {
///     let s = w.slug.clone();
///     (b'A'..b'Z').map(move |c|
///         format!("{}{}", c as char, &s)
///     )
/// }).collect();
/// assert_eq!(&v, &[(wl.get("AIRS").unwrap(), wl.get("PAIRS").unwrap())]);
/// ```
pub fn pairs_iter<I, F, J>(
    list1: I,
    list2: &Wordlist,
    mut trans: F,
) -> impl Iterator<Item = (<I::Item as ToOwned>::Owned, &WordlistEntry)>
where
    I: IntoIterator,
    I::Item: ToOwned,
    F: FnMut(&I::Item) -> J,
    J: IntoIterator,
    J::Item: Text,
{
    list1.into_iter().flat_map(move |w1| {
        trans(&w1)
            .into_iter()
            .filter_map(move |wt: J::Item| list2.get(wt).map(|w2| (w1.to_owned(), w2)))
    })
}
