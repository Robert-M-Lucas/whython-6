use crate::memory::runtime_memory::heap_memory::HeapMemory;

mod memory;
mod error;
mod execution;
mod compilation;
mod util;
mod instructions;
mod lexical_parsing;
mod file_util;

fn main() {
    let mut heap = HeapMemory::new(32);

    heap.allocate(2);

    heap.allocate(10);

    heap.free(1);

    heap.allocate(2);

    heap.dump_usage();
}
