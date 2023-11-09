use num_format::{Locale, ToFormattedString};
use std::fs;
use std::io::Write;
use crate::error::BoxedError;
use crate::memory::runtime_memory::program_cursor::ProgramCursor;

#[derive(Default)]
pub struct MemoryManager {
    pub memory: Vec<u8>,
}

impl MemoryManager {
    /// Creates an empty memory manager
    pub fn new() -> Self {
        Self { memory: Vec::new() }
    }

    /// Gets the position after the last piece of memory written
    pub fn position(&self) -> usize {
        self.memory.len()
    }

    /// Adds a byte to the memory
    pub fn append_byte(&mut self, data: u8) -> usize {
        let position = self.position();
        self.memory.push(data);
        position
    }

    /// Adds an array of bytes to the end
    pub fn append(&mut self, data: &[u8]) -> usize {
        let position = self.position();
        self.memory.extend(data);
        position
    }

    /// Overwrites a region of memory
    pub fn overwrite(&mut self, position: usize, data: &[u8]) {
        for (i, b) in data.iter().enumerate() {
            self.memory[position + i] = *b;
        }
    }

    /// Reserves a section of memory. Returns the position of this memory
    pub fn reserve(&mut self, amount: usize) -> usize {
        let position = self.position();
        self.memory.reserve(amount);
        for _ in 0..amount {
            self.memory.push(0);
        }
        position
    }

    /// Saves the bytes in a '`name.b`' file
    pub fn dump_bytes(&self, name: String) {
        let name = name + ".b";
        println!(
            "Dumping memory to file '{}' [{} bytes]",
            &name,
            self.memory.len().to_formatted_string(&Locale::en)
        );

        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(name);

        let Ok(mut file) = file else {
            println!("Failed to open file - {}", file.unwrap_err());
            return;
        };

        if let Err(e) = file.write_all(&self.memory) {
            println!("Failed to write to file - {}", e)
        }
    }

    /// Saves compiled data to a file with the specified name (excluding extension)
    //noinspection SpellCheckingInspection
    pub fn save_to_file(&self, name: String) {
        let name = name + format!(" - {}.cwhy", usize::BITS).as_str();

        println!(
            "Saving data '{}' [{} bytes]",
            &name,
            self.memory.len().to_formatted_string(&Locale::en)
        );

        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(name);

        let Ok(mut file) = file else {
            println!("Failed to open file - {}", file.unwrap_err());
            return;
        };

        if let Err(e) = file.write_all(&self.memory) {
            println!("Failed to write to file - {}", e)
        }
    }

    /// Loads data from a compiled file
    pub fn load_from_file(path: String) -> Result<Self, BoxedError> {
        println!("Loading precompiled data from file '{}'", &path);

        let data = fs::read(path)?;

        Ok(Self { memory: data })
    }
}

impl From<Vec<u8>> for MemoryManager {
    fn from(value: Vec<u8>) -> Self {
        MemoryManager { memory: value }
    }
}

impl From<MemoryManager> for Vec<u8> {
    fn from(value: MemoryManager) -> Self {
        value.memory
    }
}

impl From<MemoryManager> for ProgramCursor {
    fn from(value: MemoryManager) -> Self {
        ProgramCursor::new(value.memory)
    }
}