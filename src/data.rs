//! Data sets (currently just the NATO alphabet)

lazy_static! {
    /// A list of the words in the NATO phonetic alphabet.
    /// ```
    /// use puzzletools::data::NATO_ALPHABET;
    /// assert_eq!(NATO_ALPHABET[16], "QUEBEC");
    /// ```
    pub static ref NATO_ALPHABET: Vec<&'static str> =
        include_str!("../data/nato_phonetic_alphabet.txt").lines().collect();
}
