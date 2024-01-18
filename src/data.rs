//! Data sets (NATO alphabet, chemical elements)

use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct ChemicalElement {
    pub number: usize,
    pub symbol: String,
    pub name: String,
}

/// ```
/// use puzzletools::data::{CHEMICAL_ELEMENTS, map_by};
/// let elements_by_symbol = map_by(&*CHEMICAL_ELEMENTS, |e| e.symbol.as_str());
/// assert_eq!(elements_by_symbol["Be"].number, 4);
/// ```
pub fn map_by<F, K, I>(it: I, mut key: F) -> HashMap<K, I::Item>
where
    I: IntoIterator,
    K: Eq + std::hash::Hash,
    F: FnMut(&I::Item) -> K,
{
    it.into_iter().map(|i| (key(&i), i)).collect()
}

lazy_static! {
    /// A list of the words in the NATO phonetic alphabet.
    /// ```
    /// use puzzletools::data::NATO_ALPHABET;
    /// assert_eq!(NATO_ALPHABET[16], "QUEBEC");
    /// ```
    pub static ref NATO_ALPHABET: Vec<&'static str> =
        include_str!("../data/nato_phonetic_alphabet.txt").lines().collect();


    /// A list chemical elements.
    /// ```
    /// use puzzletools::data::{CHEMICAL_ELEMENTS, map_by};
    /// let elements_by_symbol = map_by(&*CHEMICAL_ELEMENTS, |e| e.symbol.as_str());
    /// assert_eq!(elements_by_symbol["Be"].number, 4);
    /// ```
    pub static ref CHEMICAL_ELEMENTS: Vec<ChemicalElement> = {
        let data = std::io::Cursor::new(include_str!("../data/elements.tsv"));
        let rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .from_reader(data);
        let r: Result<Vec<ChemicalElement>,_> = rdr.into_deserialize().collect();
        r.unwrap()
    };

    static ref CHEMICAL_ELEMENTS_BY_SYMBOL: HashMap<String, &'static ChemicalElement> = map_by(&*CHEMICAL_ELEMENTS, |e| e.symbol.to_ascii_uppercase());
}

/// ```
/// use puzzletools::data::parse_as_element_symbols;
/// let (freq, symbs) = parse_as_element_symbols("THESOUTH");
/// assert_eq!(freq, 1);
/// assert!(symbs.is_some());
/// let symb_strs: Vec<_> = symbs.unwrap().iter().map(|e| e.symbol.as_str()).collect();
/// assert_eq!(symb_strs, vec!["Th", "Es", "O", "U", "Th"]);
/// ```
pub fn parse_as_element_symbols(s: &str) -> (u64, Option<Vec<&'static ChemicalElement>>) {
    let mut partial: Vec<(u64, usize)> = Vec::with_capacity(s.len() + 1);
    partial.push((1, 0));
    for idx in 1..(s.len() + 1) {
        let mut n = 0;
        let mut off = 0;
        for sz in (1..3).rev() {
            if idx >= sz {
                let k = &s[s.len() - idx..s.len() - idx + sz];
                if CHEMICAL_ELEMENTS_BY_SYMBOL.contains_key(k) {
                    n += partial[idx - sz].0;
                    if off == 0 {
                        off = sz;
                    }
                }
            }
        }
        partial.push((n, off))
    }
    let tot = partial[s.len()].0;
    if tot == 0 {
        return (0, None);
    }
    let mut v: Vec<&'static ChemicalElement> = Vec::with_capacity(s.len());
    let mut idx = s.len();
    while idx > 0 {
        let sz = partial[idx].1;
        let k = &s[s.len() - idx..s.len() - idx + sz];
        v.push(CHEMICAL_ELEMENTS_BY_SYMBOL[k]);
        idx -= sz;
    }
    (tot, Some(v))
}
