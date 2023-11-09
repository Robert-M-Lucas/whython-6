use crate::memory::MemoryManager;
use crate::processing::blocks::BlockCoordinator;
use crate::processing::lines::LineHandler;
use crate::processing::processor::ProcessingResult;
use crate::processing::symbols::{Keyword, Symbol};

pub struct BreakContinueLine {}

impl LineHandler for BreakContinueLine {
    fn process_line(
        line: &[Symbol],
        memory_managers: &mut MemoryManager,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult {
        if line.is_empty() {
            return ProcessingResult::Unmatched;
        }

        #[allow(unreachable_patterns)]
        match line[0] {
            Symbol::Keyword(keyword) => match keyword {
                Keyword::Break => match block_coordinator.break_block_handler(memory_managers) {
                    Ok(_) => ProcessingResult::Success,
                    Err(e) => ProcessingResult::Failure(e),
                },
                Keyword::Continue => {
                    match block_coordinator.continue_block_handler(memory_managers) {
                        Ok(_) => ProcessingResult::Success,
                        Err(e) => ProcessingResult::Failure(e),
                    }
                }
                _ => ProcessingResult::Unmatched,
            },
            _ => ProcessingResult::Unmatched,
        }
    }
}
