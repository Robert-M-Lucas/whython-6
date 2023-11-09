use std::fs;
use std::io::Write;

pub struct HeapMemory {
    data: Vec<u8>,
    usage: Vec<u8>,
    capacity: usize,
}

impl HeapMemory {
    pub fn new(capacity: usize) -> HeapMemory {
        let capacity = capacity + (capacity % 8);

        HeapMemory {
            data: vec![0; capacity],
            usage: vec![0; (capacity / 8)],
            capacity
        }
    }

    pub fn dump_usage(&self) {
        let mut output_string = String::with_capacity(self.capacity);

        for byte in 0..(self.capacity / 8) {
            for bit in 0..8u8 {
                if self.usage[byte] & (1 << bit) != 0 {
                    output_string.push('1');
                }
                else {
                    output_string.push('0');
                }
            }
        }

        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open("heap_usage.txt").unwrap().write_all(output_string.as_bytes());
    }

    fn find_free(&self, size: usize) -> usize {
        // Look for unallocated area of size + 2 (last bit is to indicate end of allocated area - prev and cur)
        let mut free: usize = 0;
        for byte in 0..(self.capacity / 8) {
            for bit in 0..8u8 {
                if self.usage[byte] & (1 << bit) == 0 {
                    free += 1;

                    if free == size + 2 {
                        return ((byte * 8) + bit as usize) - size;
                    }
                }
                else {
                    free = 0;
                }
            }
        }

        self.dump_usage();
        panic!("Could not find heap memory slot of size '{}'. Dumping usage", size);
    }

    pub fn allocate(&mut self, size: usize) -> usize {
        let address = self.find_free(size);

        for a in address..address + size {
            let byte = a / 8;
            let bit: u8 = (a % 8) as u8;

            println!("{:#b}", 1 << bit);
            self.usage[byte] |= 1 << bit;
            println!("{:#b}", self.usage[byte]);
            println!("-");
        }

        address
    }

    pub fn free(&mut self, address: usize) {
        let mut i = 0;
        loop {
            let byte = (address + i) / 8;
            let bit: u8 = ((address + i) % 8) as u8;

            if self.usage[byte] & 1 << bit == 0 { break; }
            self.usage[byte] &= !(1 << bit);

            i += 1;
        }
    }

    pub fn get_location(&self, location: usize) -> &[u8] {
        todo!()
    }
}