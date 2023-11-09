use crate::memory::MemoryManager;
use crate::processing::blocks::BlockCoordinator;
use crate::q;

use crate::processing::instructions::view_memory_6::ViewMemoryInstruction;
use crate::processing::instructions::view_memory_dec_16::ViewMemoryDecInstruction;
use crate::processing::lines::LineHandler;
use crate::processing::processor::ProcessingResult;
use crate::processing::symbols::{Keyword, Symbol};

pub struct ViewMemoryLine {}

impl LineHandler for ViewMemoryLine {
    fn process_line(
        line: &[Symbol],
        program_memory: &mut MemoryManager,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult {
        if line.is_empty() {
            return ProcessingResult::Unmatched;
        }

        let dec = match line[0] {
            Symbol::Keyword(Keyword::ViewMemory) => false,
            Symbol::Keyword(Keyword::ViewMemoryDecimal) => true,
            _ => return ProcessingResult::Unmatched,
        };

        if line.len() != 2 {
            return ProcessingResult::Failure("viewmem must be followed by a variable".to_string());
        }

        let variable = match &line[1] {
            Symbol::Name(name) => {
                q!(q!(block_coordinator.get_reference(name)).get_variable_ref())
            }
            _ => {
                return ProcessingResult::Failure(
                    "viewmem must be followed by a variable".to_string(),
                )
            }
        };

        if dec {
            ViewMemoryDecInstruction::new_alloc(program_memory, variable);
        } else {
            ViewMemoryInstruction::new_alloc(program_memory, variable);
        }

        ProcessingResult::Success
    }
}
