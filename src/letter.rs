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
    fn byte(self) -> u8 { self }
}

impl<'a> Letter for &'a u8 {
    fn byte(self) -> u8 { *self }
}

impl Letter for char {
    fn byte(self) -> u8 { self as u8 }
}

impl<'a> Letter for &'a char {
    fn byte(self) -> u8 { *self as u8 }
}

/// Converts an uppercase letter into a number.  This is zero-offset,
/// so A becomes 0.
/// ```
/// use puzzletools::letter::lett_to_num_0;
/// assert_eq!(lett_to_num_0(b'E'),4);
/// ```
pub fn lett_to_num_0(c: u8) -> usize {
    (c - b'A') as usize
}

pub fn is_dna_letter<L: Letter>(c: L) -> bool {
    match c.byte() {
        b'A' | b'C' | b'T' | b'G' => true,
        _ => false
    }
}

pub fn is_rna_letter<L: Letter>(c: L) -> bool {
    match c.byte() {
        b'A' | b'C' | b'U' | b'G' => true,
        _ => false
    }
}

/// Returns `true` if the character is a vowel, including Y.
pub fn is_vowel_y<L: Letter>(c: L) -> bool {
    match c.byte() {
        b'A' | b'E' | b'I' | b'O' | b'U' | b'Y' => true,
        _ => false
    }
}

/// Retuns `true` if the character is a vowel, not including Y.
pub fn is_vowel_no_y<L: Letter>(c: L) -> bool {
    match c.byte() {
        b'A' | b'E' | b'I' | b'O' | b'U' => true,
        _ => false
    }
}

pub fn is_news_letter<L: Letter>(c: L) -> bool {
    match c.byte() {
        b'E' | b'N' | b'S' | b'W' => true,
        _ => false
    }
}

pub fn is_roman_numeral_letter<L: Letter>(c: L) -> bool {
    match c.byte() {
        b'I' | b'V' | b'X' | b'L' | b'C' | b'D' | b'M' => true,
        _ => false
    }
}

pub fn is_ascender<L: Letter>(c: L) -> bool {
    match c.byte() {
        b'B' | b'D' | b'F' | b'H' | b'K' | b'L' | b'T' => true,
        _ => false
    }
}

pub fn is_descender<L: Letter>(c: L) -> bool {
    match c.byte() {
        b'G' | b'J' | b'P' | b'Q' | b'Y' => true,
        _ => false
    }
}

