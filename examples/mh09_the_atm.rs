extern crate puzzletools;

use puzzletools::search::print_result;
use puzzletools::wordlist::load_wordlist_iter;
use puzzletools::word::is_addition;

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| is_addition("MES", &w.slug(), 1))
        .take(50)
        .for_each(print_result);
}
