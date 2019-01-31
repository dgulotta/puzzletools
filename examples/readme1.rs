extern crate puzzletools;
use puzzletools::word::{num_unique_letters, Text};
use puzzletools::wordlist::load_wordlist_iter;
use puzzletools::search::print_result;

fn main()
{
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| {
        let slug = w.slug();
        let bytes = slug.as_bytes();
        let l = slug.len();
        bytes[0]==b'X' && bytes[l-1]==b'X' && num_unique_letters(&slug) == l-1
    }).take(50).for_each(print_result);
}
