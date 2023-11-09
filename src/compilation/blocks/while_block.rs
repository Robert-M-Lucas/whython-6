use crate::bx;
use crate::memory::MemoryManager;
use crate::processing::arithmetic::evaluate_arithmetic_to_types;
use crate::processing::blocks::{BlockHandler, BlockType, StackSizes};
use crate::processing::instructions::jump_if_not_9::JumpIfNotInstruction;
use crate::processing::instructions::jump_instruction_10::JumpInstruction;
use crate::processing::reference_manager::ReferenceStack;
use crate::processing::symbols::{Symbol, TypeSymbol};

pub struct WhileBlock {
    jump_end_instruction: Option<JumpIfNotInstruction>,
    jump_end_instructions: Vec<JumpInstruction>,
    jump_start_instructions: Vec<JumpInstruction>,
    start_position: Option<usize>,
}

impl WhileBlock {
    pub fn new_block() -> Box<dyn BlockHandler> {
        bx!(Self {
            jump_end_instruction: None,
            jump_end_instructions: Vec::new(),
            jump_start_instructions: Vec::new(),
            start_position: None,
        })
    }
}

impl BlockHandler for WhileBlock {
    fn get_block_type(&self) -> BlockType {
        BlockType::While
    }

    fn on_entry(
        &mut self,
        program_memory: &mut MemoryManager,
        reference_stack: &mut ReferenceStack,
        stack_sizes: &mut StackSizes,
        symbol_line: &[Symbol],
    ) -> Result<(), String> {
        //? Save position before boolean evaluation
        self.start_position = Some(program_memory.get_position());

        //? Extract boolean
        let result = evaluate_arithmetic_to_types(
            &symbol_line[1..],
            &[TypeSymbol::Boolean],
            program_memory,
            reference_stack,
            stack_sizes,
        )?;
        let condition_boolean = result.as_ref();

        //? Create instruction to leave while if condition is false
        self.jump_end_instruction = Some(JumpIfNotInstruction::new_alloc(
            program_memory,
            condition_boolean.get_address(),
            0,
        ));

        Ok(())
    }

    fn on_exit(
        &mut self,
        program_memory: &mut MemoryManager,
        reference_stack: &mut ReferenceStack,
        stack_sizes: &mut StackSizes,
        _symbol_line: &[Symbol],
    ) -> Result<bool, String> {
        self.on_forced_exit(program_memory, reference_stack, stack_sizes)?;
        Ok(true)
    }

    fn on_forced_exit(
        &mut self,
        program_memory: &mut MemoryManager,
        _reference_stack: &mut ReferenceStack,
        _stack_sizes: &mut StackSizes,
    ) -> Result<(), String> {
        //? Insert looping instruction
        JumpInstruction::new_alloc(program_memory, self.start_position.unwrap());

        //? Set all instructions to jump to correct locations
        self.jump_end_instruction
            .as_mut()
            .unwrap()
            .set_destination(program_memory.get_position(), program_memory);
        for i in self.jump_end_instructions.iter_mut() {
            i.set_destination(program_memory.get_position(), program_memory);
        }
        for i in self.jump_start_instructions.iter_mut() {
            i.set_destination(self.start_position.unwrap(), program_memory);
        }
        Ok(())
    }

    fn on_break(&mut self, program_memory: &mut MemoryManager) -> Result<bool, String> {
        // Go to end of while
        self.jump_end_instructions
            .push(JumpInstruction::new_alloc(program_memory, 0));
        Ok(true)
    }

    fn on_continue(&mut self, program_memory: &mut MemoryManager) -> Result<bool, String> {
        // Go to start of while
        self.jump_start_instructions
            .push(JumpInstruction::new_alloc(program_memory, 0));
        Ok(true)
    }
}
