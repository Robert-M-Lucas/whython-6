use crate::bx;
use crate::memory::MemoryManager;
use crate::processing::blocks::{BlockHandler, BlockType, StackSizes};
use crate::processing::reference_manager::class::ClassReference;
use crate::processing::reference_manager::{Reference, ReferenceStack};
use crate::processing::symbols::{Block, Symbol, CLASS_SELF_NAME, TypeSymbol};

pub struct ClassBlock {
    name: Option<String>,
    properties_phase: bool,
    allow_line: bool,
}

impl ClassBlock {
    pub fn new_block() -> Box<dyn BlockHandler> {
        bx!(Self { name: None, properties_phase: true, allow_line: false })
    }
}

impl BlockHandler for ClassBlock {
    fn get_block_type(&self) -> BlockType {
        BlockType::Class
    }

    fn on_entry(
        &mut self,
        _program_memory: &mut MemoryManager,
        reference_stack: &mut ReferenceStack,
        _stack_sizes: &mut StackSizes,
        symbol_line: &[Symbol],
    ) -> Result<(), String> {
        if symbol_line.len() != 2 {
            return Err(format!(
                "Class declaration must be formatted {} [Name]",
                Block::Class.get_code_representation()
            ));
        }

        let name = match &symbol_line[1] {
            Symbol::Name(name) => {
                if name.len() != 1 {
                    return Err("Class names cannot have separators".to_string());
                }
                name[0].clone()
            }
            _ => {
                return Err(format!(
                    "Class declaration must be formatted {} [Name]",
                    Block::Class.get_code_representation()
                ))
            }
        };

        self.name = Some(name.clone());

        reference_stack
            .register_reference_with_offset(
                Reference::Class(ClassReference::new_empty(name)),
                vec![CLASS_SELF_NAME.to_string()],
                1,
            )
            .unwrap();

        Ok(())
    }

    fn on_forced_exit(
        &mut self,
        _program_memory: &mut MemoryManager,
        reference_stack: &mut ReferenceStack,
        _stack_sizes: &mut StackSizes,
    ) -> Result<(), String> {
        reference_stack
            .get_reference_handler_mut(&[CLASS_SELF_NAME.to_string()])
            .unwrap()
            .name = self.name.take().unwrap();
        Ok(())
    }

    fn update_sub_block(&mut self, block_type: Option<BlockType>) -> Result<(), String> {
        if self.allow_line { self.allow_line = false; return Ok(()); }
        self.allow_line = false;

        match block_type {
            Some(BlockType::Function) => {
                self.properties_phase = false;
                Ok(())
            },
            _ => Err("Classes can only contain function or attributes (before the first function)".to_string()),
        }
    }

    fn handle_line(&mut self, line: &[Symbol]) -> Result<(), String> {
        self.allow_line = true;
        Ok(())
    }
}
