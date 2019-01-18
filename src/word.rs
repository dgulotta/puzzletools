use std::borrow::Cow;
use regex::Regex;

lazy_static! {
    static ref SLUG_RE: Regex = Regex::new("[^A-Z]").unwrap(); 
}

/**
 * A trait for types that could be interpreted as an ASCII string.
 * Common types that implement `AsRef<str>` or `AsRef<[u8]>` should
 * also implement this.
 */
pub trait Text {
    fn text_str(&self) -> &str;
    fn text_bytes(&self) -> &[u8] { self.text_str().as_ref() }
    fn byte(&self, idx: usize) -> u8 { self.text_bytes()[idx] }
}

macro_rules! text_impl_str {
    () => (
        fn text_str(&self) -> &str { self.as_ref() }
    )
}

macro_rules! text_impl_bytes {
    () => (
        fn text_str(&self) -> &str { ::std::str::from_utf8(self).unwrap() }
        fn text_bytes(&self) -> &[u8] { self }
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
    SLUG_RE.replace_all(s.text_str(), "")
}

/// Returns the number of (uppercase) letters in an ASCII string.
/// ```
/// use puzzletools::word::slug_len;
/// assert_eq!(slug_len("ASCII STRING"),11);
/// ```
pub fn slug_len<S: Text>(s: S) -> usize {
    s.text_bytes().iter().filter(|c| c.is_ascii_alphabetic()).count()
}

/// Returns the letters of the word in sorted order, so that two words
/// will have the same alphagram if and only if they are anagrams.
/// ```
/// use puzzletools::word::alphagram;
/// assert_eq!(alphagram("SPOON"),alphagram("SNOOP"));
/// assert_ne!(alphagram("SPOON"),alphagram("SPONN"));
/// ```
pub fn alphagram<S: Text>(s: S) -> String {
    let mut copy = s.text_bytes().to_owned();
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
    for &c in s.text_bytes() {
        let idx = lett_to_num_0(c);
        if (seen[idx] as i8) < 0 {
            seen[idx] = count;
            count += 1;
        }
    }
    let v = s.text_bytes()
        .iter()
        .map(|&c| b'A' + seen[lett_to_num_0(c)])
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
    for c in s.text_bytes() {
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
    let mut it1 = s.text_bytes().iter().peekable();
    for c2 in t.text_bytes() {
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
    let v = s.text_bytes().windows(2).filter_map(|x|
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
    for b in s.text_bytes().windows(2) {
        let idx = 26 * lett_to_num_0(b[0]) + lett_to_num_0(b[1]);
        if seen[idx] {
            repeated.push([b[0], b[1]]);
        }
        seen[idx] = true;
    }
    repeated
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
