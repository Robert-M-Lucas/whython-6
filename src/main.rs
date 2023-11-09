use crate::memory::runtime_memory::heap_memory::HeapMemory;

mod memory;
mod error;
mod execution;
mod compilation;
mod util;

fn main() {
    let mut heap = HeapMemory::new(12);
    let mut addresses = Vec::new();

    for i in 0..3 {
        addresses.push(heap.allocate(3));
    }

    heap.free(addresses[1]);

    addresses.push(heap.allocate(3));

    heap.dump_usage();
}
