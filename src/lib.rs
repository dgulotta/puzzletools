pub mod code;
pub mod data;
pub mod letter;
pub mod io;
pub mod search;
pub mod word;
pub mod wordlist;

extern crate csv;
extern crate dotenv;
extern crate failure;
extern crate fnv;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod error {
    pub type Result<T> = ::std::result::Result<T, ::failure::Error>;
}

mod util {

    pub unsafe fn prolong<'a, 'b, T: ?Sized>(r: &'a T) -> &'b T {
        &*(r as *const T)
    }

}
