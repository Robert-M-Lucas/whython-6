use super::LineHandler;
use crate::memory::MemoryManager;
use crate::processing::arithmetic::evaluate_arithmetic_into_type;
use crate::processing::blocks::BlockCoordinator;
use crate::processing::processor::ProcessingResult;

use crate::processing::symbols::Symbol;

use crate::q;

pub struct VariableAssignmentLine {}

impl LineHandler for VariableAssignmentLine {
    fn process_line(
        line: &[Symbol],
        program_memory: &mut MemoryManager,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult {
        if line.len() < 2
            || !matches!(line[0], Symbol::Name(_))
            || !matches!(line[1], Symbol::Assigner(_))
        {
            return ProcessingResult::Unmatched;
        }

        if line.len() < 3 {
            return ProcessingResult::Failure(
                "Name and Assigner must be followed by a value".to_string(),
            );
        }

        let name = match &line[0] {
            Symbol::Name(name) => name,
            _ => panic!(),
        };

        let (stack_sizes, reference_stack) =
            block_coordinator.get_stack_sizes_and_reference_stack();

        let variable = q!(q!(reference_stack.get_reference(name)).get_variable_ref());

        let assigner = match &line[1] {
            Symbol::Assigner(assigner) => assigner,
            _ => panic!(),
        };

        q!(evaluate_arithmetic_into_type(
            &assigner.get_expanded_equivalent(line[0].clone(), Vec::from(&line[2..])),
            variable,
            program_memory,
            reference_stack,
            stack_sizes
        ));

        ProcessingResult::Success
    }
}
