use crate::bx;
use crate::memory::MemoryManager;
use crate::processing::arithmetic::evaluate_arithmetic_to_types;
use crate::processing::blocks::StackSizes;
use crate::processing::blocks::{BlockHandler, BlockType};
use crate::processing::instructions::jump_if_not_9::JumpIfNotInstruction;
use crate::processing::instructions::jump_instruction_10::JumpInstruction;
use crate::processing::reference_manager::ReferenceStack;
use crate::processing::symbols::{Block, Symbol, TypeSymbol};

pub struct IfBlock {
    jump_next_instruction: Option<JumpIfNotInstruction>,
    jump_end_instructions: Vec<JumpInstruction>,
}

impl IfBlock {
    pub fn new_block() -> Box<dyn BlockHandler> {
        bx!(Self {
            jump_next_instruction: None,
            jump_end_instructions: Vec::new(),
        })
    }
}

impl BlockHandler for IfBlock {
    fn get_block_type(&self) -> BlockType {
        BlockType::If
    }

    fn on_entry(
        &mut self,
        program_memory: &mut MemoryManager,
        reference_stack: &mut ReferenceStack,
        stack_sizes: &mut StackSizes,
        symbol_line: &[Symbol],
    ) -> Result<(), String> {
        //? Extract condition boolean
        let result = evaluate_arithmetic_to_types(
            &symbol_line[1..],
            &[TypeSymbol::Boolean],
            program_memory,
            reference_stack,
            stack_sizes,
        )?;

        let condition_boolean = result.as_ref();

        //? Insert instruction to skip this section if boolean is false
        self.jump_next_instruction = Some(JumpIfNotInstruction::new_alloc(
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
        symbol_line: &[Symbol],
    ) -> Result<bool, String> {
        fn exit_with_cleanup(
            this: &mut IfBlock,
            program_memory: &mut MemoryManager,
            reference_stack: &mut ReferenceStack,
            stack_sizes: &mut StackSizes,
        ) -> Result<bool, String> {
            this.on_forced_exit(program_memory, reference_stack, stack_sizes)?;
            Ok(true)
        }

        //? No elif or else
        if symbol_line.is_empty() {
            return exit_with_cleanup(self, program_memory, reference_stack, stack_sizes);
        }

        // Filter out non-blocks
        let block_type = match &symbol_line[0] {
            Symbol::Block(block) => block,
            _ => {
                return exit_with_cleanup(self, program_memory, reference_stack, stack_sizes);
            }
        };

        //? Handle elif or else
        match block_type {
            Block::Elif => {
                if self.jump_next_instruction.is_none() {
                    return Err(format!(
                        "{} cannot follow an {} block as it will never be reached",
                        Block::Elif,
                        Block::Else
                    ));
                }

                // Add instruction to skip to end if previous if/elif condition was met and executed
                self.jump_end_instructions
                    .push(JumpInstruction::new_alloc(program_memory, 0));
                // Set jump next instruction to jump to this section (check this block if previous was false)
                self.jump_next_instruction
                    .as_mut()
                    .unwrap()
                    .set_destination(program_memory.get_position(), program_memory);
                // Reuse if handling
                self.on_entry(program_memory, reference_stack, stack_sizes, symbol_line)?;
                // Create new scope
                reference_stack.remove_handler();
                reference_stack.add_handler();
                Ok(false)
            }
            Block::Else => {
                if symbol_line.len() > 1 {
                    return Err("Else cannot be followed by any other symbol".to_string());
                }
                if self.jump_next_instruction.is_none() {
                    return Err(
                        "'else' cannot follow an 'else' block as it will never be reached"
                            .to_string(),
                    );
                }
                // Add instruction to skip to end if previous if/elif condition was met and executed
                self.jump_end_instructions
                    .push(JumpInstruction::new_alloc(program_memory, 0));
                // Set jump next instruction to jump to this section (run this block if previous was false)
                self.jump_next_instruction
                    .as_mut()
                    .unwrap()
                    .set_destination(program_memory.get_position(), program_memory);
                // Else block cannot be skipped
                self.jump_next_instruction = None;
                // Create new scope
                reference_stack.remove_handler();
                reference_stack.add_handler();
                Ok(false)
            }
            _ => exit_with_cleanup(self, program_memory, reference_stack, stack_sizes),
        }
    }

    fn on_forced_exit(
        &mut self,
        program_memory: &mut MemoryManager,
        _reference_stack: &mut ReferenceStack,
        _stack_sizes: &mut StackSizes,
    ) -> Result<(), String> {
        /*
        If :: Jump to next if not
            content
             :: Jump to end

        ElIf :: Jump to next if not
            content
             :: Jump to end

         ElIf :: Jump to next if not
            content

         */

        // Set jump to next
        if let Some(instruction) = self.jump_next_instruction.as_mut() {
            instruction.set_destination(program_memory.get_position(), program_memory)
        }

        // Set all jump to end
        for j in self.jump_end_instructions.iter_mut() {
            j.set_destination(program_memory.get_position(), program_memory);
        }
        Ok(())
    }
}
