extern crate puzzletools;

use puzzletools::search::print_result;
use puzzletools::word::Text;
use puzzletools::wordlist::load_wordlist_iter;

fn annihilates<S: Text, T: Text>(s: S, t: T) -> bool
{
    let mut b: u32 = 0;
    for c in s.bytes() {
        b ^= 1 << (c - b'A');
    }
    for c in t.bytes() {
        b ^= 1 << (c - b'A');
    }
    b == 0
}

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| {
        let s = w.slug();
        annihilates(&s, "MEAN")
    }).take(50).for_each(print_result);
}
