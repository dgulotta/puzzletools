extern crate puzzletools;
use puzzletools::word::Text;
use puzzletools::wordlist::{Wordlist, WordFreq, load_wordlist_iter, pairs};
use puzzletools::search::{sort_results, print_result};

fn rev(s: &WordFreq) -> Vec<u8>
{
    let mut v = s.slug().to_byte_vec();
    v.reverse();
    v
}

fn main()
{
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    let wl = Wordlist::load("combined.freq.txt").unwrap();
    sort_results(pairs(it.filter(|w| w.len() >= 4), &wl, rev).take(2000)).take(50).for_each(print_result);
}
