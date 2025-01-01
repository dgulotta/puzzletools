use puzzletools::search::print_result;
use puzzletools::word::Text;
use puzzletools::wordlist::load_wordlist_iter;

fn count<T: Text>(w: T, c: u8) -> usize {
    w.bytes().filter(|b| *b == c).count()
}

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| {
        let s = w.slug();
        s.char_1_eq(6, 'L')
            && count(&s, b'A') == 1
            && count(&s, b'E') == 2
            && count(&s, b'I') == 1
            && count(&s, b'O') == 1
            && count(&s, b'U') == 1
    })
    .take(50)
    .for_each(print_result);
}
