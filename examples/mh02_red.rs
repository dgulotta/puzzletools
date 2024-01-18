extern crate puzzletools;

use puzzletools::search::print_result;
use puzzletools::word::repeated_bigrams;
use puzzletools::wordlist::load_wordlist_iter;

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| repeated_bigrams(w.slug()) == vec![*b"PL"])
        .take(50)
        .for_each(print_result);
}
