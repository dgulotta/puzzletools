use std::borrow::Cow;
use regex::Regex;

lazy_static! {
    static ref SLUG_RE: Regex = Regex::new("[^A-Z]").unwrap(); 
}

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

pub fn slugify<S: Text + ?Sized>(s: &S) -> Cow<str> {
    SLUG_RE.replace_all(s.text_str(), "")
}

pub fn slug_len<S: Text>(s: S) -> usize {
    s.text_bytes().iter().filter(|c| c.is_ascii_alphabetic()).count()
}

pub fn alphagram<S: Text>(s: S) -> String {
    let mut copy = s.text_bytes().to_owned();
    copy.sort_unstable();
    unsafe { String::from_utf8_unchecked(copy) }
}

pub fn lett_to_num_0(c: u8) -> usize {
    (c - b'A') as usize
}

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

pub fn double_letters<S: Text>(s: S) -> String
{
    let v = s.text_bytes().windows(2).filter_map(|x|
        if x[0] == x[1] { Some(x[0]) } else { None }).collect();
    unsafe { String::from_utf8_unchecked(v) }
}

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
