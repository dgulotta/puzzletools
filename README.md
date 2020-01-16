# puzzletools

A Rust library for puzzle solving.

## Setup

Puzzletools is not on crates.io yet, but you can use it by adding the
following line to your `Cargo.toml`:
```toml
[dependencies]
puzzletools = { git = "https://github.com/dgulotta/puzzletools" }
```
Or clone the repository and specify the path to your local copy:
```toml
[dependencies]
puzzletools = { path = "/path/to/puzzletools" }
```

Puzzletools needs to be told where your wordlist(s) are stored.
Puzzletools does not come with any wordlists.  I recommend using the
wordlist `combined.freq.txt` from
[rspeer/solvertools](https://github.com/rspeer/solvertools).
To specify your wordlist directory, either set the environment variable
`WORDLIST_DIR` or create a file called `.env` in the crate root
directory with the text
```
WORDLIST_DIR=/path/to/wordlists
```
where `/path/to/wordlists` is the directory where your wordlists are
stored.

## Features
The main purpose of puzzletools is to make it easy to search for words
satisfying constraints that cannot easily be described by regular
expressions.

To get a list of words such that all letters are different except that
the starting and ending letters are both X:
```rust
extern crate puzzletools;
use puzzletools::word::{num_unique_letters, Text};
use puzzletools::wordlist::load_wordlist_iter;
use puzzletools::search::print_result;

fn main()
{
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    it.filter(|w| {
        let slug = w.slug();
        let l = slug.len();
        slug.char(0)=='X' && slug.char(l-1)=='X' && num_unique_letters(&slug) == l-1
    }).take(50).for_each(print_result);
}
```

To get a list of words of length at least 4 whose reverse is also a
word, sorted by combined frequency of the two words:
```rust
extern crate puzzletools;
use puzzletools::word::Text;
use puzzletools::wordlist::{Wordlist, WordFreq, load_wordlist_iter, pairs};
use puzzletools::search::{sort_results, print_result};

fn rev(s: &WordFreq) -> Vec<u8>
{
    let mut v = s.slug().to_byte_vec();
    v.reverse();
    v
}

fn main()
{
    let it = load_wordlist_iter("combined.freq.txt").unwrap();
    let wl = Wordlist::load("combined.freq.txt").unwrap();
    sort_results(pairs(it.filter(|w| w.len() >= 4), &wl, rev).take(2000)).take(50).for_each(print_result);
}
```

There are some more examples (that contain Mystery Hunt spoilers) in the
`examples` folder.

To build the documentation, run
```
cargo doc --no-deps
```

## License
Dual licensed under the [MIT License](LICENSE-MIT) and the
[Apache License, Version 2.0](LICENSE-APACHE).
