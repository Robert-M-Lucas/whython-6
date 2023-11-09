pub struct StackMemory {
    stacks: Vec<Vec<u8>>,
    current_stack: Option<usize>
}

impl StackMemory {
    pub fn new() -> StackMemory {
        StackMemory { stacks: Vec::new(), current_stack: None }
    }

    pub fn add_stack(&mut self, size: usize) {
        #[cfg(debug_assertions)]
        if self.current_stack.unwrap() != self.stacks.len() - 1 { panic!("Tried add a stack when a stack has already been added") }

        self.stacks.push(vec![0; size]);
    }

    pub fn stack_up(&mut self) {
        #[cfg(debug_assertions)]
        if self.current_stack.unwrap() == self.stacks.len() - 1 { panic!("Tried to stack up when there are no extra stacks") }

        *self.current_stack.as_mut().unwrap() += 1;
    }

    pub fn stack_down(&mut self) {
        #[cfg(debug_assertions)]
        if self.current_stack.unwrap() != self.stacks.len() - 1 { panic!("Tried to stack down when not on the top stack") }

        self.stacks.pop();
        *self.current_stack.as_mut().unwrap() -= 1;
    }

    pub fn get_location(&self, mut location: usize) -> &[u8] {
        let mut stack = self.current_stack.unwrap();
        while location >= self.stacks[stack].len() {
            location -= self.stacks[stack].len();
            stack += 1;
        }

        &self.stacks[stack][location..]
    }
}