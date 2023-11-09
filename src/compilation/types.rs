use crate::processing::symbols::{Literal, Operator, Symbol, TypeSymbol};

use self::boolean::BoolWrapper;

mod defaults;
use crate::address::Address;
use crate::bx;
use crate::memory::MemoryManager;
use crate::processing::blocks::StackSizes;
use crate::processing::instructions::copy_3::CopyInstruction;
use crate::processing::types::pointer::PointerWrapper;

pub mod boolean;
pub mod pointer;

pub trait UninstantiatedType {
    fn instantiate(&self) -> Box<dyn Type>;

    fn get_type_symbol(&self) -> TypeSymbol;
}

pub trait Type {
    fn get_type_symbol(&self) -> TypeSymbol;

    fn allocate_variable(
        &mut self,
        _stack: &mut StackSizes,
        _program_memory: &mut MemoryManager,
    ) -> Result<(), String> {
        Err(format!(
            "{:?} cannot be allocated as a variable",
            self.get_type_symbol()
        ))
    }

    fn get_constant(&self, _literal: &Literal) -> Result<Address, String> {
        Err(format!(
            "{:?} cannot be created as a constant",
            self.get_type_symbol()
        ))
    }

    fn runtime_copy_from(
        &self,
        other: &dyn Type,
        program_memory: &mut MemoryManager,
    ) -> Result<CopyInstruction, String>;

    fn runtime_copy_from_literal(
        &self,
        literal: &Literal,
        program_memory: &mut MemoryManager,
    ) -> Result<CopyInstruction, String>;

    fn get_prefix_operation_result_type(&self, operator: &Operator) -> Vec<TypeSymbol>;

    fn get_operation_result_type(&self, operator: &Operator, rhs: &TypeSymbol) -> Vec<TypeSymbol>;

    fn operate_prefix(
        &self,
        operator: &Operator,
        destination: &dyn Type,
        program_memory: &mut MemoryManager,
        stack_sizes: &mut StackSizes,
    ) -> Result<(), String>;

    fn operate(
        &self,
        operator: &Operator,
        rhs: &dyn Type,
        destination: &dyn Type,
        program_memory: &mut MemoryManager,
        stack_sizes: &mut StackSizes,
    ) -> Result<(), String>;

    fn get_address(&self) -> &Address;

    fn get_length(&self) -> usize;

    fn get_address_mut(&mut self) -> &mut Address;

    fn run_method(
        &self,
        method_name: &String,
        _arguments: &[Vec<Symbol>],
        _stack: &mut StackSizes,
        _program_memory: &mut MemoryManager,
    ) -> Result<(), String> {
        Err(format!(
            "'{}' not implemented for {:?}",
            method_name,
            self.get_type_symbol()
        ))
    }

    fn duplicate(&self) -> Box<dyn Type>;
}

pub trait Operation<LHS> {
    fn get_symbol(&self) -> Operator;

    fn get_result_type(&self, rhs: &TypeSymbol) -> Option<TypeSymbol>;

    fn operate(
        &self,
        lhs: &LHS,
        rhs: &dyn Type,
        destination: &dyn Type,
        program_memory: &mut MemoryManager,
        stack_sizes: &mut StackSizes,
    ) -> Result<(), String>;
}

pub trait PrefixOperation<LHS> {
    fn get_symbol(&self) -> Operator;

    fn get_result_type(&self) -> Option<TypeSymbol>;

    fn operate_prefix(
        &self,
        lhs: &LHS,
        destination: &dyn Type,
        program_memory: &mut MemoryManager,
        stack_sizes: &mut StackSizes,
    ) -> Result<(), String>;
}

// TODO: Refine
pub struct TypeFactory {
    uninstantiated_types: Vec<Box<dyn UninstantiatedType>>,
}

impl TypeFactory {
    pub fn get() -> Self {
        Self {
            uninstantiated_types: vec![bx!(BoolWrapper {}), bx!(PointerWrapper {})],
        }
    }

    pub fn get_unallocated_type(new_type: &TypeSymbol) -> Result<Box<dyn Type>, String> {
        let factory = Self::get();
        let Some(wrapper) = factory
            .uninstantiated_types
            .iter()
            .find(|t| t.get_type_symbol() == *new_type)
            else {
                return Err(format!("Type {:?} cannot be instantiated", new_type));
            };

        return Ok(wrapper.instantiate());
    }

    pub fn get_default_type_for_literal(
        literal: &Literal,
        prefered_type: Option<&TypeSymbol>,
    ) -> Result<TypeSymbol, String> {
        match literal {
            Literal::Bool(_) => Ok(TypeSymbol::Boolean),
            Literal::Int(_) => Ok(match prefered_type {
                None => TypeSymbol::Integer,
                Some(TypeSymbol::Pointer) => TypeSymbol::Pointer,
                _ => TypeSymbol::Integer,
            }),
            _ => Err(format!(
                "{} does not have a default type (use as syntax)",
                literal
            )),
        }
    }

    pub fn get_default_instantiated_type_for_literal(
        literal: &Literal,
        stack: &mut StackSizes,
        program_memory: &mut MemoryManager,
        prefered_type: Option<&TypeSymbol>,
    ) -> Result<Box<dyn Type>, String> {
        let type_symbol = Self::get_default_type_for_literal(literal, prefered_type)?;
        let mut t = Self::get_unallocated_type(&type_symbol)?;
        t.allocate_variable(stack, program_memory)?;
        t.runtime_copy_from_literal(literal, program_memory)?;
        Ok(t)
    }
}

impl Default for TypeFactory {
    fn default() -> Self {
        Self::get()
    }
}
