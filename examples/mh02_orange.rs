extern crate puzzletools;

use puzzletools::search::print_result;
use puzzletools::word::double_letters;
use puzzletools::wordlist::load_wordlist_iter;

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| {
        double_letters(w.slug()) == "ES"
    }).take(50).for_each(print_result);
}
