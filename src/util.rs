use std::mem;

pub const USIZE_BYTES: usize = mem::size_of::<usize>();

pub fn read_usize(data: &[u8]) -> usize {
    usize::from_le_bytes((data[..USIZE_BYTES]).try_into().unwrap())
}