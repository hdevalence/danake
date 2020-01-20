#[macro_use]
extern crate zkp;

mod epoch;
mod tag;

pub(crate) mod constants;
pub(crate) use tag::Tag;

pub use epoch::*;
pub mod wallet;
