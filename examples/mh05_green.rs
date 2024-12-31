extern crate puzzletools;
extern crate regex;

use puzzletools::search::print_result;
use puzzletools::wordlist::load_wordlist_iter;
use puzzletools::word::Text;
use puzzletools::code::to_morse;
use regex::Regex;

fn morse_reg(c: char) -> Regex {
    let mut s = String::from("^I?");
    let mut first = true;
    for w in to_morse(c).unwrap().bytes() {
        if first { first = false; }
        else { s.push('I'); }
        s.push_str("[^AEIOUY]");
        if w == b'-' { s.push_str("{2}") }
    }
    s.push_str("I?$");
    Regex::new(&s).unwrap()
}

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    let re = morse_reg('R');
    it.filter(|w| {
        let s = w.slug();
        s.char(0)=='W' && re.is_match(&s)
    }).take(50).for_each(print_result);
}
