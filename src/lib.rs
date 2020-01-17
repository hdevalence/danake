#[macro_use]
extern crate zkp;

mod epoch;

pub use epoch::*;

pub mod wallet;

mod tag;
pub(crate) use tag::Tag;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
