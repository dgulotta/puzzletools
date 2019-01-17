extern crate puzzletools;

use puzzletools::wordlist::load_wordlist_iter;
use puzzletools::search::print_result;

fn boston_letter(s: &[u8]) -> Option<u8> {
    if s.len() < 5 {
        return None;
    }
    for i in 0..(s.len() - 4) {
        if s[i] == s[i + 4] && s[i + 1] == s[i + 3] {
            return Some(s[i + 2]);
        }
    }
    None
}

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| {
        w.len() == 6 && boston_letter(w.slug().as_bytes()) == Some(b'I')
    }).take(50).for_each(print_result);
}
