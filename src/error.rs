pub enum NoallocError {
    LengthExceed,
    IndexExceed,
}

pub type Result<T> = core::result::Result<T, NoallocError>;

