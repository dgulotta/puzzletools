extern crate puzzletools;

use puzzletools::letter::is_news_letter;
use puzzletools::search::print_result;
use puzzletools::word::{Text, special_letter_block};
use puzzletools::wordlist::load_wordlist_iter;

fn is_xvi(c: u8) -> bool {
    match c {
        b'X' | b'V' | b'I' => true,
        _ => false
    }
}

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| {
        let s = w.slug();
        if let Some(bl) = special_letter_block(&s, is_xvi) {
            if &s[bl] != "XIV" { return false; }
            s.bytes().filter(|&w| is_news_letter(w)).eq("SEEN".bytes())
        }
        else { false }
    }).take(50).for_each(print_result);
}
