//! Tests for commonly used properties of individual letters.

/// A byte or a character.  Functions in the `puzzletools` crate use
/// this trait in order to accept either bytes or characters.
/// ```
/// use puzzletools::letter::is_dna_letter;
/// assert!(is_dna_letter('A'));
/// assert!(is_dna_letter(b'A'));
/// ```
pub trait Letter {
    fn byte(self) -> u8;
}

impl Letter for u8 {
    fn byte(self) -> u8 {
        self
    }
}

impl Letter for &u8 {
    fn byte(self) -> u8 {
        *self
    }
}

impl Letter for char {
    fn byte(self) -> u8 {
        self as u8
    }
}

impl Letter for &char {
    fn byte(self) -> u8 {
        *self as u8
    }
}

/// Converts an uppercase letter into a number.  This is zero-offset,
/// so A becomes 0.
/// ```
/// use puzzletools::letter::lett_to_num_0;
/// assert_eq!(lett_to_num_0(b'E'),4);
/// ```
pub const fn lett_to_num_0(c: u8) -> usize {
    (c - b'A') as usize
}

pub fn is_dna_letter<L: Letter>(c: L) -> bool {
    matches!(c.byte(), b'A' | b'C' | b'T' | b'G')
}

pub fn is_rna_letter<L: Letter>(c: L) -> bool {
    matches!(c.byte(), b'A' | b'C' | b'U' | b'G')
}

/// Returns `true` if the character is a vowel, including Y.
pub fn is_vowel_y<L: Letter>(c: L) -> bool {
    matches!(c.byte(), b'A' | b'E' | b'I' | b'O' | b'U' | b'Y')
}

/// Retuns `true` if the character is a vowel, not including Y.
pub fn is_vowel_no_y<L: Letter>(c: L) -> bool {
    matches!(c.byte(), b'A' | b'E' | b'I' | b'O' | b'U')
}

pub fn is_news_letter<L: Letter>(c: L) -> bool {
    matches!(c.byte(), b'E' | b'N' | b'S' | b'W')
}

pub fn is_roman_numeral_letter<L: Letter>(c: L) -> bool {
    matches!(c.byte(), b'I' | b'V' | b'X' | b'L' | b'C' | b'D' | b'M')
}

pub fn is_ascender<L: Letter>(c: L) -> bool {
    matches!(c.byte(), b'B' | b'D' | b'F' | b'H' | b'K' | b'L' | b'T')
}

pub fn is_descender<L: Letter>(c: L) -> bool {
    matches!(c.byte(), b'G' | b'J' | b'P' | b'Q' | b'Y')
}

pub fn scrabble_value<L: Letter>(c: L) -> u32 {
    match c.byte() {
        b'A' | b'E' | b'I' | b'O' | b'U' | b'L' | b'N' | b'S' | b'T' | b'R' => 1,
        b'D' | b'G' => 2,
        b'B' | b'C' | b'M' | b'P' => 3,
        b'F' | b'H' | b'V' | b'W' | b'Y' => 4,
        b'K' => 5,
        b'J' | b'X' => 8,
        b'Q' | b'Z' => 10,
        b'.' => 0,
        _ => panic!("invalid letter"),
    }
}
