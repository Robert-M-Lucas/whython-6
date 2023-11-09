use crate::memory::MemoryManager;

use crate::processing::blocks::while_block::WhileBlock;
use crate::processing::blocks::BlockCoordinator;

use crate::processing::lines::LineHandler;
use crate::processing::processor::ProcessingResult;
use crate::processing::symbols::{Block, Symbol};
use crate::q;

pub struct WhileLine {}

impl LineHandler for WhileLine {
    fn process_line(
        line: &[Symbol],
        program_memory: &mut MemoryManager,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult {
        if line.is_empty() {
            return ProcessingResult::Unmatched;
        }

        match line[0] {
            Symbol::Block(Block::While) => {
                q!(block_coordinator.add_block_handler(
                    WhileBlock::new_block(),
                    program_memory,
                    line,
                ));

                ProcessingResult::Success
            }
            _ => ProcessingResult::Unmatched,
        }
    }
}
