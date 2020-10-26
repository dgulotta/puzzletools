pub mod code;
pub mod data;
pub mod letter;
pub mod io;
pub mod iter;
pub mod search;
pub mod word;
pub mod wordlist;

extern crate csv;
extern crate dotenv;
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
        Dotenv(#[from] dotenv::Error),
        #[error("{0}")]
        Io(#[from] std::io::Error)
    }
    pub type Result<T> = ::std::result::Result<T, Error>;
}

mod util {

    pub unsafe fn prolong<'a, 'b, T: ?Sized>(r: &'a T) -> &'b T {
        &*(r as *const T)
    }

}
