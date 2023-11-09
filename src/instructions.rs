use crate::instructions::example_instruction::ExampleInstruction;

mod example_instruction;

macro_rules! instruction {
    ($code: expr, $instruction: ident) => {
        if $code == $instruction::CODE { $instruction::execute(); return; }
    };
}

pub fn execute_code(code: u8) {
    instruction!(code, ExampleInstruction);
}