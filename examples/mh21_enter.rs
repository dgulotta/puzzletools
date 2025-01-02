use puzzletools::letter::lett_to_num_0;
use puzzletools::search::{print_result, sort_results};
use puzzletools::word::{deleted_letter_iter, Text};
use puzzletools::wordlist::{pairs_iter, Wordlist};

const FLIPPED: &[u8] = b"EB@PA@@YIR@TWUOD@JSLN@MXHZ";

fn transform_one(c: char) -> char {
    FLIPPED[lett_to_num_0(c)] as char
}

fn transform(s: String) -> String {
    s.chars().rev().map(transform_one).collect()
}

fn main() {
    let wl = Wordlist::load("combined.freq.txt").unwrap();
    let goal = transform_one('W');
    let it = pairs_iter(wl.iter().filter(|w| w.len() >= 5), &wl, |w| {
        deleted_letter_iter(&w.slug).filter_map(|i| {
            if i.deleted_char() == goal {
                Some(transform(i.text()))
            } else {
                None
            }
        })
    })
    .take(100000);
    sort_results(it).take(50).for_each(print_result);
}
