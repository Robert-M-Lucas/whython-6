mod stack_memory;
pub mod heap_memory;
pub mod program_cursor;

use crate::memory::runtime_memory::heap_memory::HeapMemory;
use crate::memory::runtime_memory::program_cursor::ProgramCursor;
use crate::memory::runtime_memory::stack_memory::StackMemory;

pub struct RuntimeMemory {
    program: ProgramCursor,
    stack: StackMemory,
    heap: HeapMemory
}

impl RuntimeMemory {
    pub fn program(&self) -> &ProgramCursor { &self.program }
    pub fn program_mut(&mut self) -> &mut ProgramCursor { &mut self.program }
    pub fn stack(&self) -> &StackMemory { &self.stack }
    pub fn stack_mut(&mut self) -> &mut StackMemory { &mut self.stack }
    pub fn heap(&self) -> &HeapMemory { &self.heap }
    pub fn heap_mut(&mut self) -> &mut HeapMemory { &mut self.heap }
}