use puzzletools::search::print_result;
use puzzletools::wordlist::load_wordlist_iter;

fn is_crafty(w: &str) -> bool {
    let wds: Vec<_> = w.split_whitespace().collect();
    if wds.len() != 2 || wds[0].len() != 6 || wds[1].len() <= 1 {
        return false;
    }
    let mut bk = 0i8;
    let mut wt = 0i8;
    for c in wds[1].bytes() {
        match c {
            b'W' | b'H' | b'I' | b'T' | b'E' => wt += 1,
            b'B' | b'L' | b'A' | b'C' | b'K' => bk += 1,
            _ => return false,
        }
    }
    for (c1, c2) in wds[0].bytes().zip("PEGMAN".bytes()) {
        if c1 == c2 {
            bk -= 1;
        } else if wds[0].as_bytes().contains(&c2) {
            wt -= 1;
        }
    }
    bk == 0 && wt == 0
}

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| is_crafty(&w.word))
        .take(50)
        .for_each(print_result);
}
