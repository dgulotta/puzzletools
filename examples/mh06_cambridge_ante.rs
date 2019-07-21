extern crate puzzletools;

use puzzletools::wordlist::load_wordlist_iter;
use puzzletools::search::print_result;

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| {
        let s = w.slug();
        s.len() == 10 && s.contains("DA") && s.contains("AN") && s.contains("NA") && s.contains("CU") && s.contains("UL")
    }).take(50).for_each(print_result);
}
