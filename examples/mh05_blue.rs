extern crate puzzletools;

use puzzletools::search::print_result;
use puzzletools::word::Text;
use puzzletools::wordlist::load_wordlist_iter;

fn binary<S: Text>(v: S) -> Option<u8>
{
    let mut ans: u64 = 0;
    for c in v.text_bytes() {
        match c {
            b'O' => ans <<= 1,
            b'I' => { ans <<= 1; ans += 1; }
            b'A' | b'E' | b'U' | b'Y' => return None,
            _ => {}
        };
    }
    if ans >=1 && ans <= 26 {
        Some((ans as u8) + b'@')
    } else {
        None
    }
}

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| {
        binary(w.slug()) == Some(b'W')
    }).take(50).for_each(print_result);
}
