use super::LineHandler;
use crate::memory::MemoryManager;
use crate::processing::arithmetic::evaluate_arithmetic_into_type;
use crate::processing::blocks::{BlockCoordinator, BlockType, StackSizes};
use crate::processing::processor::ProcessingResult;
use crate::processing::reference_manager::{Reference, ReferenceStack};

use crate::processing::symbols::{Assigner, Symbol};
use crate::processing::types::TypeFactory;
use crate::q;

pub struct VariableInitialisationLine {}

impl VariableInitialisationLine {
    pub fn handle_initialisation(
        line: &[Symbol],
        program_memory: &mut MemoryManager,
        reference_stack: &mut ReferenceStack,
        stack_sizes: &mut StackSizes,
        has_value: bool,
    ) -> Result<(), String> {
        if has_value && line.len() < 4 {
            return Err(
                "Type must be followed by a Name, '=' and value to initialise a variable"
                    .to_string(),
            );
        } else if !has_value && line.len() != 2 {
            return Err("Variable must be formatted [Type] [Name]".to_string());
        }

        let name = match &line[1] {
            Symbol::Name(name) => name,
            _ => return Err("Type must be followed by a Name to initialise a variable".to_string()),
        };

        if has_value {
            match &line[2] {
                Symbol::Assigner(Assigner::Setter) => {}
                _ => {
                    return Err(
                        "Type must be followed by a Name, '=' and value to initialise a variable"
                            .to_string(),
                    )
                }
            };
        }

        let mut object = match &line[0] {
            Symbol::Type(type_symbol) => TypeFactory::get_unallocated_type(type_symbol)?,
            _ => return Err(format!("Type expected, recieved {}", &line[0])),
        };

        object.allocate_variable(stack_sizes, program_memory)?;

        if has_value {
            evaluate_arithmetic_into_type(
                &line[3..],
                object.as_ref(),
                program_memory,
                reference_stack,
                stack_sizes,
            )?;
        }

        reference_stack.register_reference(Reference::Variable(object), name.clone())?;

        Ok(())
    }
}

impl LineHandler for VariableInitialisationLine {
    fn process_line(
        line: &[Symbol],
        program_memory: &mut MemoryManager,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult {
        if line.is_empty() || !matches!(line[0], Symbol::Type(_)) {
            return ProcessingResult::Unmatched;
        }

        // println!("{}", block_coordinator.get_block_handler_type());
        if matches!(block_coordinator.get_block_handler_type(), BlockType::Class) {
            println!("Inner");
            q!(block_coordinator.get_block_handler_mut().handle_line(line));
            return ProcessingResult::Success;
        }

        let (reference_stack, stack_sizes) =
            block_coordinator.get_reference_stack_and_stack_sizes();

        q!(VariableInitialisationLine::handle_initialisation(
            line,
            program_memory,
            reference_stack,
            stack_sizes,
            true
        ));

        ProcessingResult::Success
    }
}
