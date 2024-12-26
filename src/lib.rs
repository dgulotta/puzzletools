pub mod code;
pub mod data;
pub mod io;
pub mod iter;
pub mod letter;
pub mod search;
pub mod word;
pub mod wordlist;

extern crate csv;
extern crate dotenvy;
extern crate fnv;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate thiserror;

pub mod error {
    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("{0}")]
        Csv(#[from] csv::Error),
        #[error("{0}")]
        Dotenv(#[from] dotenvy::Error),
        #[error("{0}")]
        Io(#[from] std::io::Error),
    }
    pub type Result<T> = ::std::result::Result<T, Error>;
}

mod util {

    pub unsafe fn prolong<'b, T: ?Sized>(r: &T) -> &'b T {
        &*(r as *const T)
    }
}
