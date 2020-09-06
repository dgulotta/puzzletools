//! Codes (Morse, amino acids)

use std::collections::HashMap;
use crate::word::Text;

lazy_static! {
    static ref ALPHA_TO_MORSE: HashMap<char,String> = {
        let mut r = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .quoting(false)
            .from_reader(&include_bytes!("../data/morse.tsv")[..]);
        r.deserialize::<(char,String)>().map(csv::Result::unwrap).collect()
    };

    static ref MORSE_TO_ALPHA: HashMap<&'static str,char> =
        ALPHA_TO_MORSE.iter().map(|(&k,v)| (v.as_str(),k)).collect();

    static ref GENETIC_CODE_DNA: HashMap<String,char> = {
        let mut r = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .from_reader(&include_bytes!("../data/genetic_code.tsv")[..]);
        r.deserialize::<(String,char)>().map(csv::Result::unwrap).collect()
    };

    static ref GENETIC_CODE_RNA: HashMap<String,char> =
        GENETIC_CODE_DNA.iter().map(|(s,c)| (s.replace("T","U"), *c)).collect();
}

// Right now the Morse code functions recognize a non-ascii character
// (MULTIPLICATION SIGN), which clashes with the behavior of the rest of
// the library.  We should probably get rid of it.
/// Returns the Morse code representation of the given character.
/// ```
/// use puzzletools::code::to_morse;
/// assert_eq!(to_morse('m'), Some("--"));
/// ```
pub fn to_morse(c: char) -> Option<&'static str> {
    ALPHA_TO_MORSE.get(&c.to_ascii_uppercase()).map(
        String::as_str,
    )
}

/// Returns the character with the given Morse code representation, if
/// one exists.
/// ```
/// use puzzletools::code::from_morse;
/// assert_eq!(from_morse("--"), Some('M'));
/// ```
pub fn from_morse<S: Text>(s: S) -> Option<char> {
    MORSE_TO_ALPHA.get(s.as_str()).copied()
}

/// Returns the letter of the amino acid corresponding to the given
/// three-letter DNA sequence.
/// ```
/// use puzzletools::code::dna_letter;
/// assert_eq!(dna_letter("ATA"), Some('I'));
/// ```
pub fn dna_letter<S: Text>(s: S) -> Option<char> {
    GENETIC_CODE_DNA.get(s.as_str()).copied()
}

/// Returns the letter of the amino acid corresponding to the given
/// three-letter RNA sequence.
/// ```
/// use puzzletools::code::rna_letter;
/// assert_eq!(rna_letter("AUA"), Some('I'));
/// ```
pub fn rna_letter<S: Text>(s: S) -> Option<char> {
    GENETIC_CODE_RNA.get(s.as_str()).copied()
}
