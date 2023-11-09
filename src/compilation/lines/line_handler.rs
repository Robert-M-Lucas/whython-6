pub trait LineHandler {
    /// Attempts to process a line
    ///
    /// # Returns
    /// * `ProcessingResult::Successful` if the line is matched
    /// * `ProcessingResult::Unmatched` if the line is unmatched
    /// * `ProcessingResult::Failure(reason)` if the line is matched but an error occurred while processing it
    fn process_line(
        line: &[Symbol],
        program_memory: &mut MemoryManager,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult;
}
