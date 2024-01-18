//! Codes (Morse, amino acids)

use crate::letter::{lett_to_num_0, Letter};
use crate::word::Text;
use std::collections::HashMap;

const BRAILLE_BITS: [u8; 26] = [
    0x1, 0x3, 0x9, 0x19, 0x11, 0xb, 0x1b, 0x13, 0xa, 0x1a, 0x5, 0x7, 0xd, 0x1d, 0x15, 0xf, 0x1f,
    0x17, 0xe, 0x1e, 0x25, 0x27, 0x3a, 0x2d, 0x3d, 0x35,
];

lazy_static! {
    static ref ALPHA_TO_MORSE: HashMap<char, String> = {
        let mut r = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .quoting(false)
            .from_reader(&include_bytes!("../data/morse.tsv")[..]);
        r.deserialize::<(char, String)>()
            .map(csv::Result::unwrap)
            .collect()
    };
    static ref MORSE_TO_ALPHA: HashMap<&'static str, char> = ALPHA_TO_MORSE
        .iter()
        .map(|(&k, v)| (v.as_str(), k))
        .collect();
    static ref GENETIC_CODE_DNA: HashMap<String, char> = {
        let mut r = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .from_reader(&include_bytes!("../data/genetic_code.tsv")[..]);
        r.deserialize::<(String, char)>()
            .map(csv::Result::unwrap)
            .collect()
    };
    static ref GENETIC_CODE_RNA: HashMap<String, char> = GENETIC_CODE_DNA
        .iter()
        .map(|(s, c)| (s.replace('T', "U"), *c))
        .collect();
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
    ALPHA_TO_MORSE
        .get(&c.to_ascii_uppercase())
        .map(String::as_str)
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

fn braille_bits<L: Letter>(l: L) -> u8 {
    BRAILLE_BITS[lett_to_num_0(l.byte())]
}

/// ```
/// use puzzletools::code::braille_distance;
/// assert_eq!(braille_distance('Q','W'),3);
/// assert_eq!(braille_distance('Q','R'),1);
/// assert_eq!(braille_distance('C','W'),4);
/// ```
pub fn braille_distance<L: Letter, M: Letter>(l1: L, l2: M) -> u32 {
    (braille_bits(l1) ^ braille_bits(l2)).count_ones()
}
