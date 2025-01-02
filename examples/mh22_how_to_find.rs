use puzzletools::search::print_result;
use puzzletools::wordlist::load_wordlist_iter;

fn num_seq(s: &[u8]) -> [u8; 9] {
    let mut a = [0; 9];
    for i in 0..9 {
        for c in s {
            if s[i] >= *c {
                a[i] += 1;
            }
        }
    }
    a
}

fn main() {
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| w.len() == 9 && num_seq(w.slug().as_bytes()) == [3, 7, 2, 9, 1, 5, 6, 8, 4])
        .take(50)
        .for_each(print_result);
}
