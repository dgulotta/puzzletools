extern crate puzzletools;

use puzzletools::search::print_result;
use puzzletools::wordlist::load_wordlist_iter;
use puzzletools::word::Text;

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| {
        let s = w.slug();
        s.get_byte(7) == Some(b'Y') && s.contains("STAR")
    }).take(50).for_each(print_result);
}
