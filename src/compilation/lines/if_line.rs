use crate::memory::MemoryManager;

use crate::processing::blocks::if_block::IfBlock;
use crate::processing::blocks::BlockCoordinator;

use crate::processing::lines::LineHandler;
use crate::processing::processor::ProcessingResult;
use crate::processing::symbols::{Block, Symbol};

pub struct IfLine {}

impl LineHandler for IfLine {
    fn process_line(
        line: &[Symbol],
        program_memory: &mut MemoryManager,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult {
        if line.is_empty() {
            return ProcessingResult::Unmatched;
        }

        match line[0] {
            Symbol::Block(Block::If) => {
                match block_coordinator.add_block_handler(
                    IfBlock::new_block(),
                    program_memory,
                    line,
                ) {
                    Err(e) => ProcessingResult::Failure(e),
                    Ok(_) => ProcessingResult::Success,
                }
            }
            //? If not intercepted, there was no if
            Symbol::Block(Block::Elif | Block::Else) => ProcessingResult::Failure(format!(
                "{} and {} can only follow an {} statement",
                Block::Elif,
                Block::Else,
                Block::If
            )),
            _ => ProcessingResult::Unmatched,
        }
    }
}
