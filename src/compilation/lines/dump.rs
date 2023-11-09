use crate::memory::MemoryManager;
use crate::processing::blocks::BlockCoordinator;
use crate::processing::instructions::dump_5::DumpInstruction;
use crate::processing::lines::LineHandler;
use crate::processing::processor::ProcessingResult;
use crate::processing::symbols::{Keyword, Symbol};

pub struct DumpLine {}

impl LineHandler for DumpLine {
    fn process_line(
        line: &[Symbol],
        program_memory: &mut MemoryManager,
        _block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult {
        if line.is_empty() {
            return ProcessingResult::Unmatched;
        }

        match line[0] {
            Symbol::Keyword(Keyword::Dump) => {}
            _ => return ProcessingResult::Unmatched,
        };

        if line.len() > 1 {
            return ProcessingResult::Failure(
                "Dump cannot be followed by anything else".to_string(),
            );
        }

        DumpInstruction::new_alloc(program_memory);

        ProcessingResult::Success
    }
}
