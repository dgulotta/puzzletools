extern crate puzzletools;

use puzzletools::search::{print_result, sort_results};
use puzzletools::wordlist::{load_wordlist_iter, pairs, Wordlist};

// The list names_female.txt can be generated by
// https://github.com/dgulotta/puzzle-tools
// (note the dash; this is a different repository, written in Python)

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    let wl = Wordlist::load("names_female.txt").unwrap();
    sort_results(pairs(it, &wl, |w| w.slug()[1..].to_owned()).take(2000))
        .take(50)
        .for_each(print_result);
}
