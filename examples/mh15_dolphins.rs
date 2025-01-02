use puzzletools::search::print_result;
use puzzletools::word::{num_unique_letters, Text};
use puzzletools::wordlist::load_wordlist_iter;

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| {
        let s = w.slug();
        let o = 'T';
        s.len() == 9
            && s.char_1(5) == 'G'
            && num_unique_letters(&s) == 8
            && (1..=4).any(|n| s.char_1(n) == o && s.char_1(10 - n) == o)
    })
    .take(50)
    .for_each(print_result);
}
