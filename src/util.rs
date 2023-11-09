use std::{fs, mem};
use std::io::Write;

pub const USIZE_BYTES: usize = mem::size_of::<usize>();

pub fn read_usize(data: &[u8]) -> usize {
    usize::from_le_bytes((data[..USIZE_BYTES]).try_into().unwrap())
}

pub fn dump_bytes(file: &str, data: &[u8]) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file)
        .unwrap();

    file.write_all(data).unwrap();
}

// TODO: Replace all instances of this with proper path handling
pub fn join_file_name(name: &[String]) -> String {
    let mut joined_name = String::new();
    for (i, string) in name.iter().enumerate() {
        joined_name += string;

        if i < name.len() - 2 {
            joined_name.push('/');
        } else if i == name.len() - 2 {
            joined_name.push('.')
        }
    }
    joined_name
}
