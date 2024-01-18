extern crate puzzletools;

use puzzletools::search::{print_result, sort_results};
use puzzletools::word::Text;
use puzzletools::wordlist::{load_wordlist_iter, WordFreq};

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    let itf = it.filter_map(|w| {
        let s = w.slug();
        if s.len() != 10 || s.char_1(3) != 'I' {
            return None;
        }
        let f = (0..10)
            .map(|i| {
                if s.byte(i) == "BLITHESOME".byte(i) || s.byte(i) == "FRIDAKAHLO".byte(i) {
                    8
                } else {
                    1
                }
            })
            .fold(w.freq, |p, q| p * q);
        Some(WordFreq {
            word: w.word,
            freq: f,
        })
    });
    sort_results(itf).take(50).for_each(print_result);
}
