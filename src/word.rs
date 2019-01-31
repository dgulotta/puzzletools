use std::borrow::Cow;
use regex::Regex;

lazy_static! {
    static ref SLUG_RE: Regex = Regex::new("[^A-Z]").unwrap();
}

/// A trait for types that could be interpreted as an ASCII string.
/// Common types that implement `AsRef<str>` or `AsRef<[u8]>` should
/// also implement this.
pub trait Text {
    fn as_str(&self) -> &str;
    fn as_bytes(&self) -> &[u8] { self.as_str().as_ref() }
    fn bytes(&self) -> std::str::Bytes { self.as_str().bytes() }
    fn chars(&self) -> std::str::Chars { self.as_str().chars() }
    fn byte(&self, idx: usize) -> u8 { self.as_bytes()[idx] }
    fn char(&self, idx: usize) -> char { self.byte(idx) as char }
    fn get_byte(&self, idx: usize) -> Option<u8> {
        self.as_bytes().get(idx).map(|&c| c)
    }
    fn get_char(&self, idx: usize) -> Option<char> {
        self.get_byte(idx).map(|c| c as char)
    }
    fn to_byte_vec(&self) -> Vec<u8> { self.as_bytes().to_owned() }
}

macro_rules! text_impl_str {
    () => (
        fn as_str(&self) -> &str { self }
    )
}

macro_rules! text_impl_bytes {
    () => (
        fn as_str(&self) -> &str { ::std::str::from_utf8(self).unwrap() }
        fn as_bytes(&self) -> &[u8] { self }
    )
}

impl Text for str {
    text_impl_str!();
}

impl<'a> Text for &'a str {
    text_impl_str!();
}

impl Text for String {
    text_impl_str!();
}

impl<'a> Text for &'a String {
    text_impl_str!();
}

impl<'a> Text for Cow<'a,str> {
    text_impl_str!();
}

impl<'a, 'b> Text for &'a Cow<'b,str> {
    text_impl_str!();
}

impl Text for [u8] {
    text_impl_bytes!();
}

impl<'a> Text for &'a [u8] {
    text_impl_bytes!();
}

impl<'a> Text for Cow<'a,[u8]> {
    text_impl_bytes!();
}

impl<'a, 'b> Text for &'a Cow<'b,[u8]> {
    text_impl_bytes!();
}

impl Text for Vec<u8> {
    text_impl_bytes!();
}

impl<'a> Text for &'a Vec<u8> {
    text_impl_bytes!();
}

/// Removes spaces and punctuation from a string.  Warning: currently
/// also removes lowercase letters, so only use this with uppercase strings.
/// ```
/// use puzzletools::word::slugify;
/// assert_eq!(slugify("ONE 2 THREE"),"ONETHREE");
///
/// ```
pub fn slugify<S: Text + ?Sized>(s: &S) -> Cow<str> {
    SLUG_RE.replace_all(s.as_str(), "")
}

/// Returns the number of (uppercase) letters in an ASCII string.
/// ```
/// use puzzletools::word::slug_len;
/// assert_eq!(slug_len("ASCII STRING"),11);
/// ```
pub fn slug_len<S: Text>(s: S) -> usize {
    s.bytes().filter(|c| c.is_ascii_alphabetic()).count()
}

/// Returns the letters of the word in sorted order, so that two words
/// will have the same alphagram if and only if they are anagrams.
/// ```
/// use puzzletools::word::alphagram;
/// assert_eq!(alphagram("SPOON"),alphagram("SNOOP"));
/// assert_ne!(alphagram("SPOON"),alphagram("SPONN"));
/// ```
pub fn alphagram<S: Text>(s: S) -> String {
    let mut copy = s.to_byte_vec();
    copy.sort_unstable();
    unsafe { String::from_utf8_unchecked(copy) }
}

/// Converts an uppercase letter into a number.  This is zero-offset,
/// so A becomes 0.
/// ```
/// use puzzletools::word::lett_to_num_0;
/// assert_eq!(lett_to_num_0(b'E'),4);
/// ```
pub fn lett_to_num_0(c: u8) -> usize {
    (c - b'A') as usize
}

/// Applies a subsitution cipher so that the first letter of the word becomes A,
/// the second unique letter becomes B, etc.  Two words have the same ciphergram
/// if and only they can be obtained from each other via a substitution cipher.
/// ```
/// use puzzletools::word::ciphergram;
/// assert_eq!(ciphergram("POTATO"),ciphergram("UNEVEN"));
/// ```
pub fn ciphergram<S: Text>(s: S) -> String {
    let mut seen = [0xFFu8; 26];
    let mut count = 0u8;
    for c in s.bytes() {
        let idx = lett_to_num_0(c);
        if (seen[idx] as i8) < 0 {
            seen[idx] = count;
            count += 1;
        }
    }
    let v = s.bytes()
        .map(|c| b'A' + seen[lett_to_num_0(c)])
        .collect();
    unsafe { String::from_utf8_unchecked(v) }
}

/// Returns the number of unique letters in a word.
/// ```
/// use puzzletools::word::num_unique_letters;
/// assert_eq!(num_unique_letters("LETTERS"),5);
/// ```
pub fn num_unique_letters<S: Text>(s: S) -> usize {
    let mut seen = 0u32;
    let mut count = 0;
    for c in s.bytes() {
        let p = 1u32 << (c - b'A');
        if seen & p == 0 {
            seen |= p;
            count += 1;
        }
    }
    count
}

/// Returns `true` if the second word is obtained from the first word
/// by adding `additions` letters (at any place in the word)
/// ```
/// use puzzletools::word::is_addition;
/// assert!(is_addition("PORE","SPORE",1))
/// ```
pub fn is_addition<S, T>(s: S, t: T, mut additions: u8) -> bool
where
    S: Text,
    T: Text,
{
    let mut it1 = s.bytes().peekable();
    for c2 in t.bytes() {
        if it1.peek() == Some(&c2) {
            it1.next();
        } else {
            if additions == 0 {
                return false;
            }
            additions -= 1;
        }
    }
    return additions == 0 && it1.peek().is_none();
}

/// Returns a list of all letters that appear twice consecutively
/// in a word.
/// ```
/// use puzzletools::word::double_letters;
/// assert_eq!(double_letters("PUZZLETOOLS"),"ZO");
/// ```
pub fn double_letters<S: Text>(s: S) -> String
{
    let v = s.as_bytes().windows(2).filter_map(|x|
        if x[0] == x[1] { Some(x[0]) } else { None }).collect();
    unsafe { String::from_utf8_unchecked(v) }
}

/// Returns a list of bigrams that appear more than once in a word.
/// (If a bigram appears `n` times in the word, it will appear `n-1`
/// times in the list;
/// ```
/// use puzzletools::word::repeated_bigrams;
/// assert_eq!(repeated_bigrams("ONGOING"),vec![*b"NG"]);
/// ```
pub fn repeated_bigrams<S: Text>(s: S) -> Vec<[u8; 2]>
{
    let mut seen = [false; 676];
    let mut repeated = Vec::new();
    for b in s.as_bytes().windows(2) {
        let idx = 26 * lett_to_num_0(b[0]) + lett_to_num_0(b[1]);
        if seen[idx] {
            repeated.push([b[0], b[1]]);
        }
        seen[idx] = true;
    }
    repeated
}

/// If there is a single block of letters in `s` satisfying
/// the predicate `pred`, returns the location of that block.
/// Otherwise returns None.
/// ```
/// use std::ops::Range;
/// use puzzletools::word::{is_roman_numeral_letter, special_letter_block};
/// assert_eq!(special_letter_block("ARXIV", is_roman_numeral_letter), Some(2..5));
/// assert_eq!(special_letter_block("REFLEXIVE", is_roman_numeral_letter), None);
/// assert_eq!(special_letter_block("THROUGHOUT", is_roman_numeral_letter), None);
/// ```
pub fn special_letter_block<S: Text, F: FnMut(u8) -> bool>(s: S, mut pred: F) -> Option<std::ops::Range<usize>>
{
    let mut start = None;
    let mut end = None;
    for (n, c) in s.bytes().enumerate() {
        if pred(c) {
            if start.is_none() {
                start = Some(n);
            }
            else if end.is_some() {
                return None;
            }
        }
        else if start.is_some() && end.is_none() {
            end = Some(n);
        }
    }
    if let Some(st) = start {
        let en = end.unwrap_or(s.as_bytes().len());
        Some(st..en)
    }
    else { None }
}

pub fn is_dna_letter(c: u8) -> bool {
    match c {
        b'A' | b'C' | b'T' | b'G' => true,
        _ => false
    }
}

pub fn is_rna_letter(c: u8) -> bool {
    match c {
        b'A' | b'C' | b'U' | b'G' => true,
        _ => false
    }
}

/// Returns `true` if the character is a vowel, including Y.
pub fn is_vowel_y(c: u8) -> bool {
    match c {
        b'A' | b'E' | b'I' | b'O' | b'U' | b'Y' => true,
        _ => false
    }
}

/// Retuns `true` if the character is a vowel, not including Y.
pub fn is_vowel_no_y(c: u8) -> bool {
    match c {
        b'A' | b'E' | b'I' | b'O' | b'U' => true,
        _ => false
    }
}

pub fn is_news_letter(c: u8) -> bool {
    match c {
        b'E' | b'N' | b'S' | b'W' => true,
        _ => false
    }
}

pub fn is_roman_numeral_letter(c: u8) -> bool {
    match c {
        b'I' | b'V' | b'X' | b'L' | b'C' | b'D' | b'M' => true,
        _ => false
    }
}

#[test]
fn alphagram_test() {
    assert_eq!(alphagram("POTATO"), "AOOPTT");
}

#[test]
fn ciphergram_test() {
    assert_eq!(ciphergram("POTATO"), "ABCDCB");
}

#[test]
fn unique_test() {
    assert_eq!(num_unique_letters("POTATO"), 4);
}

#[test]
fn addition_test() {
    assert!(is_addition("POTATO", "POTATOS", 1));
    assert!(!is_addition("POTATO", "POTATO", 1));
    assert!(!is_addition("POTATO", "POTATOES", 1));
    assert!(!is_addition("MESSAGE", "MESO", 1));
}

#[test]
fn double_test() {
    assert_eq!(double_letters("NEEDLESS"), "ES");
}

#[test]
fn repeated_test() {
    assert_eq!(repeated_bigrams("APPLEDUMPLING"), vec![*b"PL"]);
}
