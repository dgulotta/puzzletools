use puzzletools::search::print_result;
use puzzletools::wordlist::load_wordlist_iter;

fn is_add(w: &str, s: &str) -> Option<u8> {
    let mut b = s.bytes().peekable();
    let mut seen = None;
    for c in w.bytes() {
        match c {
            b'A' | b'E' | b'I' | b'O' | b'U' | b'Y' => {}
            _ => {
                if b.peek() == Some(&c) {
                    b.next();
                } else if seen.is_some() {
                    return None;
                } else {
                    seen = Some(c)
                }
            }
        }
    }
    if b.peek().is_some() {
        None
    } else {
        seen
    }
}

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| is_add(&w.slug(), "NPTN") == Some(b'S'))
        .take(50)
        .for_each(print_result);
}
