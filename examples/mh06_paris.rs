use puzzletools::code::rna_letter;
use puzzletools::search::print_result;
use puzzletools::word::{special_letter_block, Text};
use puzzletools::wordlist::load_wordlist_iter;

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| {
        let s = w.slug();
        if s.contains('T') {
            return false;
        }
        if let Some(i) = special_letter_block(&s, |c| matches!(c, b'A' | b'C' | b'G' | b'U')) {
            rna_letter(&s.as_str()[i]) == Some('R')
        } else {
            false
        }
    })
    .take(100)
    .for_each(print_result);
}
