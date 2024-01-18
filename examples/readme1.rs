extern crate puzzletools;
use puzzletools::search::print_result;
use puzzletools::word::{num_unique_letters, Text};
use puzzletools::wordlist::load_wordlist_iter;

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| {
        let slug = w.slug();
        let l = slug.len();
        slug.char(0) == 'X' && slug.char(l - 1) == 'X' && num_unique_letters(&slug) == l - 1
    })
    .take(50)
    .for_each(print_result);
}
