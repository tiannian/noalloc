use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};

use crate::error::NoallocError;
use crate::Result;

pub struct Vec<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    length: usize,
}

impl<T, const N: usize> Vec<T, N> {
    pub fn new() -> Self {
        let data = MaybeUninit::uninit_array();
        Vec { data, length: 0 }
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
}

impl<T, const N: usize> Vec<T, N> {
    pub fn into_raw_parts(mut self) -> (*mut T, usize, usize) {
        let slice = &mut self.data[..];
        let ptr = MaybeUninit::first_ptr_mut(slice);
        (ptr, self.len(), self.capacity())
    }

    pub unsafe fn from_row_parts(ptr: *mut T, length: usize, capacity: usize) -> Vec<T, N> {}
}

impl<T, const N: usize> Vec<T, N> {
    pub fn append<const O: usize>(&mut self, other: &mut Vec<T, O>) -> Result<()> {
        let total = self.len() + other.len();
        if total > self.capacity() {
            Err(NoallocError::LengthExceed)
        } else {
            for i in 0..other.len() {
                let t = unsafe { other.data[i].read() };
                self.data[self.len() + i].write(t);
            }
            Ok(())
        }
    }

    pub fn split_off<const O: usize>(&mut self, at: usize) -> Vec<T, O> {
        assert!(at <= self.len());

        let mut v = Vec::new();
        if at < self.len() - 1 {
            for i in at..self.len() {
                let t = unsafe { self.data[i].read() };
                v.data[i - self.len()].write(t);
            }
            v.length = self.len() - at;
        }
        v
    }
}

impl<T, const N: usize> Vec<T, N>
where
    T: Clone,
{
    pub fn resize(&mut self, new_len: usize, item: T) -> Result<()> {
        assert!(new_len <= self.capacity());

        for i in self.len()..new_len {
            self.data[i].write(item.clone());
        }
        self.length = new_len;
        Ok(())
    }
}

impl<T, const N: usize> Vec<T, N>
where
    T: Default + Clone,
{
    pub fn resize_default(&mut self, new_len: usize) -> Result<()> {
        self.resize(new_len, T::default())
    }

    pub fn extend_from_slice(&mut self, other: &[T]) {
        let length = self.len() + other.len();
        assert!(length <= self.capacity());

        for i in self.len()..length {
            let t = unsafe {
                let r = other.get_unchecked(i - self.len()) as *const T;
                core::ptr::read(r)
            };
            self.data[i].write(t);
        }
    }
}

impl<T, const N: usize> Vec<T, N> {
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        let slice = &mut self.data[..];
        unsafe { MaybeUninit::slice_get_mut(slice) }
    }

    pub fn dedup_by<F>(&mut self, same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        let len = {
            let (dedup, _) = self.as_mut_slice().partition_dedup_by(same_bucket);
            dedup.len()
        };
        self.truncate(len);
    }

    pub fn truncate(&mut self, len: usize) {
        assert!(self.len() <= len);

        self.length = len;
    }

    pub fn dedup_by_key<F, K>(&mut self, mut key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq<K>,
    {
        self.dedup_by(|a, b| key(a) == key(b))
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len());

        unsafe {
            let ret;
            {
                let ptr = self.as_mut_ptr().add(index);
                ret = core::ptr::read(ptr);
                core::ptr::copy(ptr.offset(1), ptr, self.len() - index - 1);
            }
            self.length = self.len() - 1;
            ret
        }
    }
}

impl<T, const N: usize> Vec<T, N>
where
    T: PartialEq<T>,
{
    pub fn dedup(&mut self) {
        self.dedup_by(|a, b| a == b)
    }
}

impl<T, const N: usize> Vec<T, N> {
    pub fn remove_item<V>(&mut self, item: &V) -> Option<T>
    where
        T: PartialEq<V>,
    {
        let pos = self.iter().position(|x| *x == *item)?;
        Some(self.remove(pos))
    }
}

impl<T, const N: usize> Deref for Vec<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        let slice = &self.data[..];
        unsafe { MaybeUninit::slice_get_ref(slice) }
    }
}

impl<T, const N: usize> DerefMut for Vec<T, N> {
    fn deref_mut(&mut self) -> &mut [T] {
        let slice = &mut self.data[..];
        unsafe { MaybeUninit::slice_get_mut(slice) }
    }
}
