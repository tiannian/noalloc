#![no_std]
#![feature(const_generics)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_extra)]
#![feature(maybe_uninit_slice_assume_init)]
#![feature(slice_partition_dedup)]
#![feature(maybe_uninit_slice)]

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
