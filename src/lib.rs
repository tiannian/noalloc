#![no_std]
#![feature(const_generics)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_extra)]

mod error;
pub use error::Result;

mod vector;
pub use vector::Vec;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
