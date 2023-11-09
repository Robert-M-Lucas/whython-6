use crate::memory::address::Address;
use crate::memory::MemoryManager;
use crate::util::{read_usize, USIZE_BYTES};

pub struct ProgramCursor {
    position: usize,
    program: Vec<u8>
}

impl ProgramCursor {
    pub fn new(program: Vec<u8>) -> ProgramCursor {
        ProgramCursor { position: 0, program }
    }

    pub fn get_byte(&mut self) -> u8 {
        let b = self.program[self.position];
        self.position += 1;
        b
    }

    pub fn get_usize(&mut self) -> usize {
        let r = read_usize(&self.program[self.position ..]);
        self.position += USIZE_BYTES;
        r
    }

    pub fn get_location(&self, location: usize) -> &[u8] {
        &self.program[location..]
    }

    pub fn get_slice(&mut self, length: usize) -> &[u8] {
        self.position += length;
        &self.program[self.position - length .. self.position]
    }

    pub fn get_address(&mut self, expected_len: Option<usize>) -> Address {
        let (address, len) = Address::get_address(&self.program[self.position ..], expected_len);
        self.position += len;
        address
    }
}