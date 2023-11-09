use crate::address::Address;
use crate::errors::create_literal_not_impl_error;
use crate::memory::MemoryManager;
use crate::processing::blocks::StackSizes;
use crate::processing::instructions::binary_and_8::BinaryAndInstruction;
use crate::processing::instructions::binary_not_7::BinaryNotInstruction;
use crate::processing::instructions::binary_or_12::BinaryOrInstruction;
use crate::processing::instructions::copy_3::CopyInstruction;
use crate::processing::symbols::Literal;
use crate::processing::types::PrefixOperation;
use crate::util::warn;
use crate::{
    bx, default_get_type_symbol_impl, default_type_initialiser, default_type_operate_impl,
    default_type_struct, default_type_wrapper_struct_and_impl,
    processing::symbols::{Operator, TypeSymbol},
};

use super::{Operation, Type};

default_type_wrapper_struct_and_impl!(BoolWrapper, BoolType, TypeSymbol::Boolean);
default_type_struct!(BoolType);
default_type_initialiser!(BoolType, (BoolAnd, BoolOr), (BoolNot));

pub const BOOL_TRUE: u8 = 0xFF;
pub const BOOL_FALSE: u8 = 0x00;

pub const BOOLEAN_SIZE: usize = 1;

impl Type for BoolType {
    default_get_type_symbol_impl!(BoolType, TypeSymbol::Boolean);

    fn allocate_variable(
        &mut self,
        stack: &mut StackSizes,
        _program_memory: &mut MemoryManager,
    ) -> Result<(), String> {
        if self.address.is_some() {
            warn(
                format!(
                    "Allocating {:?} when it already has a memory address",
                    self.get_type_symbol()
                )
                .as_str(),
            )
        }
        self.address = Some(Address::StackDirect(
            stack.increment_stack_size(BOOLEAN_SIZE),
        ));

        Ok(())
    }

    fn get_constant(&self, literal: &Literal) -> Result<Address, String> {
        match literal {
            Literal::Bool(value) => {
                if *value {
                    Ok(Address::Immediate(vec![BOOL_TRUE]))
                } else {
                    Ok(Address::Immediate(vec![BOOL_FALSE]))
                }
            }
            Literal::Int(value) => {
                if *value == 0 {
                    Ok(Address::Immediate(vec![BOOL_FALSE]))
                } else {
                    Ok(Address::Immediate(vec![BOOL_TRUE]))
                }
            }
            other => create_literal_not_impl_error(other, self.get_type_symbol()),
        }
    }

    fn runtime_copy_from(
        &self,
        other: &dyn Type,
        program_memory: &mut MemoryManager,
    ) -> Result<CopyInstruction, String> {
        match other.get_type_symbol() {
            TypeSymbol::Boolean => Ok(CopyInstruction::new_alloc(
                program_memory,
                other.get_address(),
                self.address.as_ref().unwrap(),
                BOOLEAN_SIZE,
            )),
            s => Err(format!(
                "Copy not implemented from type '{}' to '{}'",
                s,
                TypeSymbol::Boolean
            )),
        }
    }

    fn runtime_copy_from_literal(
        &self,
        literal: &Literal,
        program_memory: &mut MemoryManager,
    ) -> Result<CopyInstruction, String> {
        let constant = self.get_constant(literal)?;

        Ok(CopyInstruction::new_alloc(
            program_memory,
            &constant,
            self.address.as_ref().unwrap(),
            BOOLEAN_SIZE,
        ))
    }

    default_type_operate_impl!(BoolType);

    fn get_address(&self) -> &Address {
        self.address.as_ref().unwrap()
    }

    fn get_length(&self) -> usize {
        BOOLEAN_SIZE
    }

    fn get_address_mut(&mut self) -> &mut Address {
        self.address.as_mut().unwrap()
    }

    fn duplicate(&self) -> Box<dyn Type> {
        let mut t = BoolType::new();
        t.address = self.address.as_ref().cloned();
        bx!(t)
    }
}

pub struct BoolAnd {}

impl Operation<BoolType> for BoolAnd {
    fn get_symbol(&self) -> Operator {
        Operator::And
    }

    fn get_result_type(&self, rhs: &TypeSymbol) -> Option<TypeSymbol> {
        // let rhs = rhs?;
        match rhs {
            TypeSymbol::Boolean => Some(TypeSymbol::Boolean),
            _ => None,
        }
    }

    fn operate(
        &self,
        lhs: &BoolType,
        rhs: &dyn Type,
        destination: &dyn Type,
        program_memory: &mut MemoryManager,
        _stack_sizes: &mut StackSizes,
    ) -> Result<(), String> {
        assert_eq!(destination.get_type_symbol(), TypeSymbol::Boolean);
        assert_eq!(rhs.get_type_symbol(), TypeSymbol::Boolean);

        let (address_from, length) = (lhs.get_address(), lhs.get_length());
        BinaryAndInstruction::new_alloc(
            program_memory,
            address_from,
            rhs.get_address(),
            destination.get_address(),
            length,
        );
        Ok(())
    }
}

pub struct BoolOr {}

impl Operation<BoolType> for BoolOr {
    fn get_symbol(&self) -> Operator {
        Operator::Or
    }

    fn get_result_type(&self, rhs: &TypeSymbol) -> Option<TypeSymbol> {
        // let rhs = rhs?;
        match rhs {
            TypeSymbol::Boolean => Some(TypeSymbol::Boolean),
            _ => None,
        }
    }

    fn operate(
        &self,
        lhs: &BoolType,
        rhs: &dyn Type,
        destination: &dyn Type,
        program_memory: &mut MemoryManager,
        _stack_sizes: &mut StackSizes,
    ) -> Result<(), String> {
        assert_eq!(destination.get_type_symbol(), TypeSymbol::Boolean);
        assert_eq!(rhs.get_type_symbol(), TypeSymbol::Boolean);

        let (address_from, length) = (lhs.get_address(), lhs.get_length());
        BinaryOrInstruction::new_alloc(
            program_memory,
            address_from,
            rhs.get_address(),
            destination.get_address(),
            length,
        );
        Ok(())
    }
}

pub struct BoolNot {}

impl PrefixOperation<BoolType> for BoolNot {
    fn get_symbol(&self) -> Operator {
        Operator::And
    }

    fn get_result_type(&self) -> Option<TypeSymbol> {
        Some(TypeSymbol::Boolean)
    }

    fn operate_prefix(
        &self,
        lhs: &BoolType,
        destination: &dyn Type,
        program_memory: &mut MemoryManager,
        _stack_sizes: &mut StackSizes,
    ) -> Result<(), String> {
        assert_eq!(destination.get_type_symbol(), TypeSymbol::Boolean);

        let (address_from, length) = (lhs.get_address(), lhs.get_length());
        BinaryNotInstruction::new_alloc(
            program_memory,
            address_from,
            destination.get_address(),
            length,
        );
        Ok(())
    }
}
