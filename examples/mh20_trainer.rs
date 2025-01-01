use puzzletools::search::{print_result, sort_results};
use puzzletools::wordlist::{pairs_filter, Wordlist, WordlistEntry};
use regex::Regex;

fn main() {
    let rg = Regex::new("^..V..$").unwrap();
    let f = |w: &&WordlistEntry| {
        if !rg.is_match(&w.slug) {
            return None;
        }
        let s = format!("{}{}", &w.slug[0..2], &w.slug[3..5]);
        Some(s)
    };
    let wl = Wordlist::load("combined.freq.txt").unwrap();
    sort_results(pairs_filter(wl.iter(), &wl, f).take(100000))
        .take(500)
        .for_each(print_result);
}
