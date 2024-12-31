extern crate puzzletools;

use puzzletools::search::print_result;
use puzzletools::word::Text;
use puzzletools::wordlist::load_wordlist_iter;

fn ws(letts: &[u8], pos: &[usize], word: &[u8]) -> bool {
    let mut s: Vec<_> = pos.iter().map(|p| word[*p]).collect();
    s.sort_unstable();
    s == letts
}

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| {
        let s = w.slug();
        if s.len() != 9 {
            return false;
        }
        ws(b"AMRVW", &[0, 3, 5, 6, 7], s.as_bytes())
    })
    .for_each(print_result);
}
