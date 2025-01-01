use puzzletools::search::print_result;
use puzzletools::word::{alphagram, Text};
use puzzletools::wordlist::load_wordlist_iter;

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    let wd = "BATTLEDROIDS";
    it.filter(|w| {
        let s = w.slug();
        if s.len() != wd.len() {
            return false;
        }
        let matches: Vec<_> = wd
            .bytes()
            .zip(s.bytes())
            .filter_map(|(a, b)| if a == b { Some(a) } else { None })
            .collect();
        matches.len() == 4 && alphagram(matches) == "ADIT"
    })
    .take(50)
    .for_each(print_result);
}
