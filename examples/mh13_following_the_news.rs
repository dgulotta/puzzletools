use puzzletools::letter::is_news_letter;
use puzzletools::search::print_result;
use puzzletools::word::Text;
use puzzletools::wordlist::load_wordlist_iter;

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| {
        let s = w.slug();
        s.get_byte_1(1) == Some(b'I')
            // unfortunatly, filter(is_news_letter) does not compile;
            // see Rust issue #36582
            && s.bytes().filter(|b| is_news_letter(b)).eq("NESE".bytes())
    })
    .take(50)
    .for_each(print_result);
}
