use crate::bx;
use crate::memory::MemoryManager;
use crate::processing::blocks::{BlockHandler, BlockType, StackSizes};
use crate::processing::instructions::stack_create_0::StackCreateInstruction;
use crate::processing::instructions::stack_down_4::StackDownInstruction;
use crate::processing::instructions::stack_up_1::StackUpInstruction;
use crate::processing::reference_manager::ReferenceStack;
use crate::processing::symbols::Symbol;

pub struct BaseBlock {
    stack_create_instruction: Option<StackCreateInstruction>,
}

impl BaseBlock {
    pub fn new_block() -> Box<dyn BlockHandler> {
        bx!(Self {
            stack_create_instruction: None,
        })
    }
}

impl BlockHandler for BaseBlock {
    fn get_block_type(&self) -> BlockType {
        BlockType::Base
    }

    fn on_entry(
        &mut self,
        program_memory: &mut MemoryManager,
        _reference_stack: &mut ReferenceStack,
        stack_sizes: &mut StackSizes,
        _symbol_line: &[Symbol],
    ) -> Result<(), String> {
        self.stack_create_instruction = Some(StackCreateInstruction::new_alloc(program_memory, 0));
        StackUpInstruction::new_alloc(program_memory);
        stack_sizes.add_stack();
        Ok(())
    }

    fn on_forced_exit(
        &mut self,
        program_memory: &mut MemoryManager,
        _reference_stack: &mut ReferenceStack,
        stack_sizes: &mut StackSizes,
    ) -> Result<(), String> {
        StackDownInstruction::new_alloc(program_memory);
        self.stack_create_instruction
            .as_mut()
            .expect("No stack create instruction")
            .set_stack_size(stack_sizes.get_stack_size(), program_memory);
        stack_sizes.remove_stack();
        Ok(())
    }
}
