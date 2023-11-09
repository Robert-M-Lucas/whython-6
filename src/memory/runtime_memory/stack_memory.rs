use std::fs;
use crate::util::dump_bytes;

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
        if self.current_stack.is_some() && self.current_stack.unwrap() != self.stacks.len() - 1 { panic!("Tried add a stack when a stack has already been added") }

        self.stacks.push(vec![0; size]);
    }

    pub fn stack_up(&mut self) {
        if self.current_stack.is_none() {
            self.current_stack = Some(0);
        }
        else {
            #[cfg(debug_assertions)]
            if self.current_stack.unwrap() == self.stacks.len() - 1 { panic!("Tried to stack up when there are no extra stacks") }

            *self.current_stack.as_mut().unwrap() += 1;
        }
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

    pub fn get_location_mut(&mut self, mut location: usize) -> &mut [u8] {
        let mut stack = self.current_stack.unwrap();
        while location >= self.stacks[stack].len() {
            location -= self.stacks[stack].len();
            stack += 1;
        }

        &mut self.stacks[stack][location..]
    }

    /// Writes all data to a specified folder for debugging
    pub fn dump_bytes(&self, folder_name: &str) {
        fs::create_dir_all(folder_name).unwrap();
        for i in self.stacks.iter().enumerate() {
            dump_bytes(format!("{}/stack-{}.b", folder_name, i.0).as_str(), i.1);
        }
    }
}