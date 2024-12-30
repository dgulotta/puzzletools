pub mod code;
pub mod data;
pub mod io;
pub mod iter;
pub mod letter;
pub mod search;
pub mod word;
pub mod wordlist;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

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
