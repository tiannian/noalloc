use core::mem::MaybeUninit;

use crate::Result;
use crate::error::NoallocError;

pub struct Vec<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    length: usize,
}

impl<T, const N: usize> Vec<T, N> {
    pub fn new() -> Self {
        let data = MaybeUninit::uninit_array();
        Vec {
            data, length: 0
        }
    }
    
    pub fn append<const O: usize>(&mut self, other: &mut Vec<T, O>) -> Result<()> {
        let total = self.len() + other.len();
        if total > self.capacity() {
            Err(NoallocError::LengthExceed)
        } else {
            for i in 0 .. other.len() {
                let t = unsafe { other.data[i].read() };
                self.data[self.len() + i].write(t);
            }
            Ok(())
        }
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        N
    }

    #[inline]
    pub fn clear(&mut self) {
        self.length = 0;
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn split_off<const O: usize>(&mut self, at: usize) -> Vec<T, O> {
        let mut v = Vec::new();
        if at < self.len() - 1 {
            for i in at .. self.len() {
                let t = unsafe { self.data[i].read() };
                v.data[i - self.len()].write(t);
            }
            v.length = self.len() - at;
        }
        v
    }

    pub fn resize(&mut self, new_len: usize, item: T) -> Result<()> where T: Clone {
        if new_len > self.capacity() {
            Err(NoallocError::LengthExceed)
        } else {
            for i in self.len() .. new_len {
                self.data[i].write(item.clone());
            }
            self.length = new_len;
            Ok(())
        }
    }

    pub fn resize_default(&mut self, new_len: usize) -> Result<()> where T: Default + Clone {
        self.resize(new_len, T::default())
    }

    pub fn resize_with<F>(&mut self, new_len: usize, f: F) where FnMut() -> T, T: Clone {
        self.resize(new_len, f())
    }
}
