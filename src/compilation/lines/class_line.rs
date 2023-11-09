use crate::memory::MemoryManager;

use crate::processing::blocks::class_block::ClassBlock;

use crate::processing::blocks::BlockCoordinator;

use crate::processing::lines::LineHandler;
use crate::processing::processor::ProcessingResult;
use crate::processing::symbols::{Block, Symbol};
use crate::q;

pub struct ClassLine {}

impl LineHandler for ClassLine {
    fn process_line(
        line: &[Symbol],
        program_memory: &mut MemoryManager,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult {
        if line.is_empty() {
            return ProcessingResult::Unmatched;
        }

        match line[0] {
            Symbol::Block(Block::Class) => {
                q!(block_coordinator.add_block_handler(
                    ClassBlock::new_block(),
                    program_memory,
                    line,
                ));
                block_coordinator.skip_sub_block_check = true;
                ProcessingResult::Success
            }
            _ => ProcessingResult::Unmatched,
        }
    }
}
