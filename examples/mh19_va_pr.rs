use puzzletools::search::print_result;
use puzzletools::word::Text;
use puzzletools::wordlist::load_wordlist_iter;
use regex::Regex;

fn main() {
    let r: Regex = Regex::new("^[A-Z]*E E[A-Z]*$").unwrap();
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| w.slug().get_char(5) == Some('Y') && r.is_match(&w.word))
        .take(50)
        .for_each(print_result);
}
