lazy_static! {
    pub static ref NATO_ALPHABET: Vec<&'static str> =
        include_str!("../data/nato_phonetic_alphabet.txt").lines().collect();
}
