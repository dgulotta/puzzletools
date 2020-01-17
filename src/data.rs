//! Data sets (NATO alphabet, chemical elements)

use std::collections::HashMap;

#[derive(Deserialize)]
pub struct ChemicalElement {
    pub number: usize,
    pub symbol: String,
    pub name: String
}

/// Creates a hash map with the elements as the iterator `it` as values,
/// using the values of the function `key` as keys.
/// ```
/// use puzzletools::data::{CHEMICAL_ELEMENTS, map_by};
/// let elements_by_symbol = map_by(&*CHEMICAL_ELEMENTS, |e| e.symbol.clone());
/// assert_eq!(elements_by_symbol["Be"].number, 4);
/// ```
pub fn map_by<F, K, I>(it: I, mut key: F) -> HashMap<K, I::Item> where
    I: IntoIterator,
    K: Eq + std::hash::Hash,
    F: FnMut(&I::Item) -> K
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
    /// let elements_by_symbol = map_by(&*CHEMICAL_ELEMENTS, |e| e.symbol.clone());
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
}
