use puzzletools::search::{print_result, sort_results};
use puzzletools::wordlist::{pairs_filter, Wordlist, WordlistEntry};
use regex::Regex;
use std::borrow::Cow;

fn main() {
    let reg = Regex::new("[B-E]").unwrap();
    let f = |w: &&WordlistEntry| {
        if w.len() < 4 || w.slug.contains('A') {
            return None;
        }
        match reg.replace_all(&w.slug, "A") {
            Cow::Owned(o) => Some(o),
            Cow::Borrowed(_) => None,
        }
    };
    let wl = Wordlist::load("combined.freq.txt").unwrap();
    sort_results(pairs_filter(wl.iter(), &wl, f).take(100000))
        .take(50)
        .for_each(print_result);
}
