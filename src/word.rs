//! Utilities for working with individual words.

use std::borrow::Cow;
use regex::Regex;
use crate::letter::lett_to_num_0;

lazy_static! {
    static ref SLUG_RE: Regex = Regex::new("[^A-Z]").unwrap();
}

/// A trait for types that could be interpreted as an ASCII string.
/// Common types that implement `AsRef<str>` or `AsRef<[u8]>` should
/// also implement this.  Functions in the `puzzletools` crate use
/// this trait in order to accept any kind of text.
///
/// # Examples
/// Functions like `alphagram` accept a variety of different types:
/// ```
/// extern crate regex;
/// use regex::Regex;
/// use puzzletools::word::{Text, alphagram};
///
/// assert_eq!(&alphagram("TEXT"), "ETTX");
/// assert_eq!(&alphagram(&b"TEXT"[..]), "ETTX");
/// let s = Regex::new("..N").unwrap().replace_all("CONTEXT", "");
/// assert_eq!(&alphagram(s), "ETTX");
/// ```
///
/// For convenience, the `Text` trait offers both 0-indexing and
/// 1-indexing:
/// ```
/// use puzzletools::word::Text;
/// assert_eq!("TEXT".byte(2), b'X');
/// assert_eq!("TEXT".byte_1(3), b'X');
/// assert_eq!("TEXT".char(2), 'X');
/// assert_eq!("TEXT".char_1(3), 'X');
/// assert_eq!("TEXT".get_char_1(3), Some('X'));
/// assert_eq!("TEXT".get_char_1(5), None);
/// ```
///
pub trait Text {
    /// Returns representation of this text as a string slice.
    fn as_str(&self) -> &str;
    /// Returns representation of this text as a byte array slice.
    fn as_bytes(&self) -> &[u8] { self.as_str().as_ref() }
    /// Returns an iterator over the letters of this text, considered as bytes.
    fn bytes(&self) -> std::str::Bytes { self.as_str().bytes() }
    /// Returns an iterator over the letters of this text, considered as characters.
    fn chars(&self) -> std::str::Chars { self.as_str().chars() }
    /// Returns the byte at the index `idx`.
    fn byte(&self, idx: usize) -> u8 { self.as_bytes()[idx] }
    /// Returns the character at the index `idx`.
    fn char(&self, idx: usize) -> char { self.byte(idx) as char }
    /// Returns the byte at the index `idx`, or `None` if `idx` is out of bounds.
    fn get_byte(&self, idx: usize) -> Option<u8> {
        self.as_bytes().get(idx).copied()
    }
    /// Returns the character at the index `idx`, or `None` if `idx` is out of bounds.
    fn get_char(&self, idx: usize) -> Option<char> {
        self.get_byte(idx).map(|c| c as char)
    }
    /// Tests whether the byte at index `idx` equals `b`, returning false if
    /// `idx` is out of bounds.
    fn byte_eq(&self, idx: usize, b: u8) -> bool {
        self.get_byte(idx) == Some(b)
    }
    /// Tests whether the character at index `idx` equals `c`, returning false
    /// if `idx` is out of bounds.
    fn char_eq(&self, idx: usize, c: char) -> bool {
        self.get_char(idx) == Some(c)
    }
    /// Returns the byte at the index `idx - 1`.
    fn byte_1(&self, idx: usize) -> u8 { self.byte(idx - 1) }
    /// Returns the character at the index `idx - 1`.
    fn char_1(&self, idx: usize) -> char { self.char(idx - 1) }
    /// Returns the byte at the index `idx - 1`, or `None` if `idx - 1` is out of bounds.
    fn get_byte_1(&self, idx: usize) -> Option<u8> { self.get_byte(idx - 1) }
    /// Returns the character at the index `idx - 1`, or `None` if `idx - 1` is out of bounds.
    fn get_char_1(&self, idx: usize) -> Option<char> { self.get_char(idx - 1) }
    /// Tests whether the byte at index `idx - 1` equals `b`, returning false
    /// if `idx - 1` is out of bounds.
    fn byte_1_eq(&self, idx: usize, b: u8) -> bool {
        self.get_byte_1(idx) == Some(b)
    }
    /// Tests whether the character at index `idx - 1` equals `c`, returning
    /// false if `idx - 1` is out of bounds.
    fn char_1_eq(&self, idx: usize, c: char) -> bool {
        self.get_char_1(idx) == Some(c)
    }
    /// Creates a representation of this text as a byte vector.
    fn to_byte_vec(&self) -> Vec<u8> { self.as_bytes().to_owned() }
    /// Returns the length of this text in bytes.
    fn len(&self) -> usize { self.as_bytes().len() }
    fn is_empty(&self) -> bool { self.as_bytes().is_empty() }
}

macro_rules! text_impl_str {
    () => (
        fn as_str(&self) -> &str { self }
    )
}

macro_rules! text_impl_bytes {
    () => (
        fn as_str(&self) -> &str { unsafe { ::std::str::from_utf8_unchecked(self) } }
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

/// Returns true if all letters in a word are different.
/// ```
/// use puzzletools::word::all_unique_letters;
/// assert!(all_unique_letters("THUNDERCLAPS"));
/// assert!(!all_unique_letters("LETTERS"));
/// ```
pub fn all_unique_letters<S: Text>(s: S) -> bool {
    let mut seen = 0u32;
    for c in s.bytes() {
        let p = 1u32 << (c - b'A');
        if seen & p == 0 {
            seen |= p;
        }
        else {
            return false;
        }
    }
    true
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
    if s.bytes().len() + (additions as usize) != t.bytes().len() { return false; }
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
    additions == 0
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
/// use puzzletools::letter::is_roman_numeral_letter;
/// use puzzletools::word::special_letter_block;
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
