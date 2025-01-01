use puzzletools::search::print_result;
use puzzletools::word::Text;
use puzzletools::wordlist::load_wordlist_iter;

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| {
        let s = w.slug();
        s.starts_with('D') && s.ends_with('D') && s.get_char_1(6) == Some('G')
    })
    .take(100)
    .for_each(print_result);
}
