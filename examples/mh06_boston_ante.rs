extern crate puzzletools;

use puzzletools::word::is_addition;
use puzzletools::wordlist::load_wordlist_iter;
use puzzletools::search::print_result;

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| {
        let s = w.slug();
        is_addition("BRIG", &s, 5)
    }).take(50).for_each(print_result);
}
