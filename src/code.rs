use std::collections::HashMap;
use csv;

lazy_static! {
    static ref ALPHA_TO_MORSE: HashMap<char,String> = {
        let mut r = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .quoting(false)
            .from_reader(&include_bytes!("../data/morse.tsv")[..]);
        r.deserialize::<(char,String)>().map(csv::Result::unwrap).collect()
    };

    static ref MORSE_TO_ALPHA: HashMap<&'static str,char> =
        ALPHA_TO_MORSE.iter().map(|(&k,v)| (v.as_str(),k)).collect();

    static ref GENETIC_CODE_DNA: HashMap<String,char> = {
        let mut r = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .from_reader(&include_bytes!("../data/genetic_code.tsv")[..]);
        r.deserialize::<(String,char)>().map(csv::Result::unwrap).collect()
    };

    static ref GENETIC_CODE_RNA: HashMap<String,char> =
        GENETIC_CODE_DNA.iter().map(|(s,c)| (s.replace("T","U"), *c)).collect();
}

pub fn to_morse(c: char) -> Option<&'static str> {
    ALPHA_TO_MORSE.get(&c.to_ascii_uppercase()).map(
        String::as_str,
    )
}

pub fn from_morse(s: &str) -> Option<char> {
    MORSE_TO_ALPHA.get(s).map(|&c| c)
}

pub fn dna_letter(s: &str) -> Option<char> {
    GENETIC_CODE_DNA.get(s).map(|&c| c)
}

pub fn rna_letter(s: &str) -> Option<char> {
    GENETIC_CODE_RNA.get(s).map(|&c| c)
}

#[test]
fn to_morse_test() {
    assert_eq!(to_morse('m'), Some("--"));
}

#[test]
fn from_morse_test() {
    assert_eq!(from_morse("--"), Some('M'));
}
