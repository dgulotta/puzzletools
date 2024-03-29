extern crate puzzletools;

use puzzletools::data::NATO_ALPHABET;
use puzzletools::search::print_result;
use puzzletools::wordlist::load_wordlist_iter;

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| NATO_ALPHABET.iter().any(|s| w.slug().contains(s)))
        .take(50)
        .for_each(print_result);
}
