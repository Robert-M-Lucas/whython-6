use crate::memory::MemoryManager;

use crate::processing::blocks::function_block::FunctionBlock;
use crate::processing::blocks::BlockCoordinator;

use crate::processing::lines::LineHandler;
use crate::processing::processor::ProcessingResult;
use crate::processing::symbols::{Block, Symbol};
use crate::q;

pub struct FunctionLine {}

impl LineHandler for FunctionLine {
    fn process_line(
        line: &[Symbol],
        program_memory: &mut MemoryManager,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult {
        if line.is_empty() {
            return ProcessingResult::Unmatched;
        }

        match line[0] {
            Symbol::Block(Block::Function) => {
                q!(block_coordinator.add_block_handler(
                    FunctionBlock::new_block(),
                    program_memory,
                    line,
                ));
                ProcessingResult::Success
            }
            _ => ProcessingResult::Unmatched,
        }
    }
}
