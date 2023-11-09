use crate::bx;
use crate::memory::MemoryManager;
use crate::processing::blocks::{BlockHandler, BlockType, StackSizes};
use crate::processing::instructions::dynamic_jump_11::DynamicJumpInstruction;
use crate::processing::instructions::jump_instruction_10::JumpInstruction;
use crate::processing::instructions::stack_create_0::StackCreateInstruction;
use crate::processing::instructions::stack_up_1::StackUpInstruction;
use crate::processing::lines::variable_initialisation::VariableInitialisationLine;
use crate::processing::reference_manager::function::FunctionReference;
use crate::processing::reference_manager::{Reference, ReferenceStack};
use crate::processing::symbols::{Block, Symbol, CLASS_SELF_NAME};
use crate::processing::types::pointer::PointerType;
use crate::processing::types::Type;

pub struct FunctionBlock {
    name: Option<Vec<String>>,
    start_position: Option<usize>,
    previous_reference_limit: Option<usize>,
    skip_instruction: Option<JumpInstruction>,
    stack_create_instruction: Option<StackCreateInstruction>,
    return_pointer: Option<PointerType>,
    stack_size_insertion_addresses: Vec<usize>,
}

impl FunctionBlock {
    pub fn new_block() -> Box<dyn BlockHandler> {
        bx!(Self {
            name: None,
            start_position: None,
            previous_reference_limit: None,
            skip_instruction: None,
            stack_create_instruction: None,
            return_pointer: None,
            stack_size_insertion_addresses: Vec::new(),
        })
    }
}

/*
    - Reference stack (1) created automatically
    - Add parameters as references
    - Add reference stack (2)
    - (function body)
    - Remove reference stack (2)
    - Move parameter references from reference stack (1) to a FunctionReference
    - FunctionReference is then added to a ClassReference above (if it exists) or just added to the above stack
*/

impl BlockHandler for FunctionBlock {
    fn get_block_type(&self) -> BlockType {
        BlockType::Function
    }

    fn on_entry(
        &mut self,
        program_memory: &mut MemoryManager,
        reference_stack: &mut ReferenceStack,
        stack_sizes: &mut StackSizes,
        symbol_line: &[Symbol],
    ) -> Result<(), String> {
        //? Add skip instruction to skip function in normal execution
        self.skip_instruction = Some(JumpInstruction::new_alloc(program_memory, 0));

        //? Save start position for FunctionReference
        self.start_position = Some(program_memory.get_position());

        // ! Don't create stack here - create at call point to allow return address passing and parameter passing
        // // Create new stack
        // self.stack_create_instruction =
        //     Some(StackCreateInstruction::new_alloc(program_memory, 0, 0));

        // Stack up
        StackUpInstruction::new_alloc(program_memory);
        // ? Add stack
        stack_sizes.add_stack();

        // //? Add return pointer
        // let mut return_pointer = PointerType::new();
        // return_pointer.allocate_variable(stack_sizes, program_memory).unwrap();
        // self.return_pointer = Some(return_pointer);

        fn declaration_error() -> Result<(), String> {
            Err(format!(
                "Function declaration must be formatted {} [Name] [Parameter List]",
                Block::Function.get_code_representation()
            ))
        }

        if symbol_line.len() != 3 {
            return declaration_error();
        }

        self.name = Some(match &symbol_line[1] {
            Symbol::Name(name) => {
                if name.len() != 1 {
                    return Err(
                        "Invalid function name - function names cannot contain separators"
                            .to_string(),
                    );
                }
                name.clone()
            }
            _ => return declaration_error(),
        });

        //? Save previous reference limit and apply new
        self.previous_reference_limit = Some(reference_stack.get_reference_depth_limit());

        //? If in class
        if reference_stack
            .get_reference(&[CLASS_SELF_NAME.to_string()])
            .is_ok()
        {
            //? Add to class if in class
            self.name
                .as_mut()
                .unwrap()
                .insert(0, CLASS_SELF_NAME.to_string());

            //? Allow class to be referenced
            reference_stack.set_reference_depth_limit(reference_stack.get_depth() - 1);
        } else {
            //? Dont allow anything outside of function to be referenced
            reference_stack.set_reference_depth_limit(reference_stack.get_depth());
        }

        let mut return_pointer = PointerType::new();
        return_pointer.allocate_variable(stack_sizes, program_memory)?;
        self.return_pointer = Some(return_pointer);

        let parameter_list = match &symbol_line[2] {
            Symbol::List(parameters) => parameters,
            _ => return declaration_error(),
        };

        for parameter in parameter_list {
            VariableInitialisationLine::handle_initialisation(
                parameter,
                program_memory,
                reference_stack,
                stack_sizes,
                false,
            )?;
        }

        //? Add parameters to FunctionReference
        let parameters = reference_stack.get_top_stack();
        let mut cloned_parameters = Vec::with_capacity(parameters.len());
        for p in parameters {
            cloned_parameters.push((
                p.name.clone(),
                p.reference().clone_variable()?.get_variable().unwrap(),
            ));
        }

        // reference_stack.add_handler();
        // for p in parameters {
        //     reference_stack.register_reference(p).unwrap();
        // }

        // let mut parameters_formatted = Vec::new();
        // for p in cloned_parameters {
        //     let (name, reference) = (p.0, p.1);
        //     if let Reference::Variable(reference) = reference {
        //         parameters_formatted.push((name, reference));
        //     } else {
        //         panic!("Parameter not a variable!")
        //     }
        // }

        //? Register function reference
        let function_reference = FunctionReference::new(
            self.start_position.unwrap(),
            self.return_pointer.as_ref().unwrap().duplicate_known(),
            cloned_parameters,
            None,
        );
        let name = self.name.as_ref().unwrap().clone();
        if name.len() > 1 {
            reference_stack.register_reference(Reference::Function(function_reference), name)?;
        } else {
            reference_stack.register_reference_with_offset(
                Reference::Function(function_reference),
                name,
                1,
            )?;
        }

        //? Add new stack to separate parameters from function body
        reference_stack.add_handler();

        Ok(())
    }

    fn on_exit(
        &mut self,
        program_memory: &mut MemoryManager,
        reference_stack: &mut ReferenceStack,
        stack_sizes: &mut StackSizes,
        _symbol_line: &[Symbol],
    ) -> Result<bool, String> {
        self.on_forced_exit(program_memory, reference_stack, stack_sizes)?;
        Ok(true)
    }

    fn on_forced_exit(
        &mut self,
        program_memory: &mut MemoryManager,
        reference_stack: &mut ReferenceStack,
        stack_sizes: &mut StackSizes,
    ) -> Result<(), String> {
        //? Remove extra handler
        reference_stack.remove_handler();

        // // Stack down
        // self.stack_create_instruction
        //     .as_mut()
        //     .expect("No stack create instruction")
        //     .change_stack_size(program_memory, stack_sizes.get_size());

        //? Jump back
        DynamicJumpInstruction::new_alloc(
            program_memory,
            self.return_pointer.as_ref().unwrap().get_address(),
        );

        //? Update reference
        reference_stack
            .get_reference_mut(self.name.as_ref().unwrap())
            .unwrap()
            .get_function_mut()
            .unwrap()
            .set_stack_size_and_complete(stack_sizes.get_stack_size(), program_memory);
        // let mut reference = reference_stack.get_and_remove_reference(self.name.as_ref().unwrap().as_str()).unwrap().0;
        // reference.get_function_mut().unwrap().set_stack_size_and_complete(stack_sizes.get_size(), program_memory);
        // reference_stack.register_reference_with_offset(reference, 1).unwrap();

        //? Undo reference limit
        reference_stack.set_reference_depth_limit(self.previous_reference_limit.unwrap());

        //? Remove stack
        stack_sizes.remove_stack();

        self.skip_instruction
            .as_mut()
            .unwrap()
            .set_destination(program_memory.get_position(), program_memory);
        Ok(())
    }

    // Don't allow break to propagate
    fn on_break(&mut self, _program_memory: &mut MemoryManager) -> Result<bool, String> {
        Err("Can't break out of a function".to_string())
    }

    // Don't allow continue to propagate
    fn on_continue(&mut self, _program_memory: &mut MemoryManager) -> Result<bool, String> {
        Err("Can't continue a function".to_string())
    }
}
