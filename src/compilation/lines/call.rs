use super::LineHandler;
use crate::memory::MemoryManager;
use crate::processing::blocks::BlockCoordinator;
use crate::processing::processor::ProcessingResult;

use crate::processing::symbols::Symbol;

use crate::q;
use crate::util::must_use_option::MustUseOption;

pub struct CallLine {}

impl LineHandler for CallLine {
    fn process_line(
        line: &[Symbol],
        program_memory: &mut MemoryManager,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult {
        if line.len() < 2
            || !matches!(line[0], Symbol::Name(_))
            || !matches!(line[1], Symbol::List(_))
        {
            return ProcessingResult::Unmatched;
        }

        if line.len() > 2 {
            return ProcessingResult::Failure(
                "A call can't be followed by anything on the same line".to_string(),
            );
        }

        let name = match &line[0] {
            Symbol::Name(name) => name,
            _ => panic!(),
        };

        let args = match &line[1] {
            Symbol::List(args) => args,
            _ => panic!(),
        };

        // let (function_reference, offset) = q!(block_coordinator.get_reference_and_offset(name));

        let (stack_sizes, reference_stack) =
            block_coordinator.get_stack_sizes_and_reference_stack();

        let function_reference = q!(q!(reference_stack.get_reference(name)).get_function_ref());

        let incomplete_function_call =
            q!(function_reference.call(None, args, program_memory, reference_stack, stack_sizes));

        if let MustUseOption::Some(incomplete_function_call) = incomplete_function_call {
            reference_stack
                .get_reference_mut(name)
                .unwrap()
                .get_function_mut()
                .unwrap()
                .add_incomplete_function_call(incomplete_function_call);
        }

        // q!(reference_stack.register_reference_with_offset(function_reference, offset));

        ProcessingResult::Success
    }
}
