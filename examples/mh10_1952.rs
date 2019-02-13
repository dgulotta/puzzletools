extern crate puzzletools;

use puzzletools::word::Text;
use puzzletools::wordlist::{Wordlist, WordlistEntry, load_wordlist_iter, pairs_iter};
use puzzletools::search::{print_result, sort_results};

fn red_words(w: &&WordlistEntry) -> impl Iterator<Item=String>
{
    let s = w.slug.clone();
    (0..(s.len())).filter_map(move |i| {
        if s.byte(i) == b'O' {
            Some(format!("{}RED{}", &s[..i], &s[(i+1)..]))
        } else { None }
    })
}

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    let wl: Wordlist = it.filter(|w| w.freq >= 10000 && w.len() >= 3).collect();
    sort_results(pairs_iter(wl.iter(), &wl, red_words).take(2000)).take(50).for_each(print_result);
}
