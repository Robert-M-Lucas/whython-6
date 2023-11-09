use crate::address::Address;
use crate::errors::create_literal_not_impl_error;
use crate::memory::MemoryManager;
use crate::processing::blocks::StackSizes;
use crate::processing::instructions::add_instruction_13::AddInstruction;
use crate::processing::instructions::binary_not_7::BinaryNotInstruction;
use crate::processing::instructions::copy_3::CopyInstruction;
use crate::processing::instructions::equality_14::EqualityInstruction;
use crate::processing::instructions::not_equal_15::NotEqualInstruction;
use crate::processing::symbols::Literal;
use crate::processing::types::{Operation, Type};
use crate::util::{warn, USIZE_BYTES};
use crate::{
    bx, default_get_type_symbol_impl, default_type_initialiser, default_type_operate_impl,
    default_type_struct, default_type_wrapper_struct_and_impl,
    processing::symbols::{Operator, TypeSymbol},
};

default_type_wrapper_struct_and_impl!(PointerWrapper, PointerType, TypeSymbol::Pointer);
default_type_struct!(PointerType);
default_type_initialiser!(PointerType, (Add, Subtract, Equal, NotEqual), ());

impl PointerType {
    pub fn duplicate_known(&self) -> PointerType {
        let mut t = PointerType::new();
        t.address = self.address.as_ref().cloned();
        t
    }
}

impl Clone for PointerType {
    fn clone(&self) -> Self {
        self.duplicate_known()
    }
}

impl Type for PointerType {
    default_get_type_symbol_impl!(PointerType, TypeSymbol::Pointer);

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
            stack.increment_stack_size(USIZE_BYTES),
        ));

        Ok(())
    }

    fn get_constant(&self, literal: &Literal) -> Result<Address, String> {
        match literal {
            Literal::Int(value) => {
                let ptr: Result<usize, _> = (*value).try_into();
                if let Ok(ptr) = ptr {
                    Ok(Address::Immediate(Vec::from(ptr.to_le_bytes())))
                } else {
                    Err(format!("The value ({}) can't fit into a {} (the value must be greater than zero and fit within your platform pointer width [{} bytes])", *value, self.get_type_symbol(), USIZE_BYTES))
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
            TypeSymbol::Pointer => Ok(CopyInstruction::new_alloc(
                program_memory,
                other.get_address(),
                self.address.as_ref().unwrap(),
                USIZE_BYTES,
            )),
            s => Err(format!(
                "Copy not implemented from type '{}' to '{}'",
                s,
                TypeSymbol::Pointer
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
            USIZE_BYTES,
        ))
    }

    default_type_operate_impl!(PointerType);

    fn get_address(&self) -> &Address {
        self.address.as_ref().unwrap()
    }

    fn get_length(&self) -> usize {
        USIZE_BYTES
    }

    fn get_address_mut(&mut self) -> &mut Address {
        self.address.as_mut().unwrap()
    }

    fn duplicate(&self) -> Box<dyn Type> {
        bx!(self.duplicate_known())
    }
}

pub struct Add {}

impl Operation<PointerType> for Add {
    fn get_symbol(&self) -> Operator {
        Operator::Add
    }

    fn get_result_type(&self, rhs: &TypeSymbol) -> Option<TypeSymbol> {
        // let rhs = rhs?;
        match rhs {
            TypeSymbol::Pointer => Some(TypeSymbol::Pointer),
            _ => None,
        }
    }

    fn operate(
        &self,
        lhs: &PointerType,
        rhs: &dyn Type,
        destination: &dyn Type,
        program_memory: &mut MemoryManager,
        _stack_sizes: &mut StackSizes,
    ) -> Result<(), String> {
        assert_eq!(destination.get_type_symbol(), TypeSymbol::Pointer);
        assert_eq!(rhs.get_type_symbol(), TypeSymbol::Pointer);

        let (address_from, length) = (lhs.get_address(), lhs.get_length());
        AddInstruction::new_alloc(
            program_memory,
            address_from,
            rhs.get_address(),
            destination.get_address(),
            length,
        );
        Ok(())
    }
}

pub struct Subtract {}

impl Operation<PointerType> for Subtract {
    fn get_symbol(&self) -> Operator {
        Operator::Subtract
    }

    fn get_result_type(&self, rhs: &TypeSymbol) -> Option<TypeSymbol> {
        // let rhs = rhs?;
        match rhs {
            TypeSymbol::Pointer => Some(TypeSymbol::Pointer),
            _ => None,
        }
    }

    fn operate(
        &self,
        lhs: &PointerType,
        rhs: &dyn Type,
        destination: &dyn Type,
        program_memory: &mut MemoryManager,
        stack_sizes: &mut StackSizes,
    ) -> Result<(), String> {
        assert_eq!(destination.get_type_symbol(), TypeSymbol::Pointer);
        assert_eq!(rhs.get_type_symbol(), TypeSymbol::Pointer);

        let mut magic_number = PointerType::new();
        magic_number
            .allocate_variable(stack_sizes, program_memory)
            .unwrap();
        //? Not the subtracted value
        BinaryNotInstruction::new_alloc(
            program_memory,
            rhs.get_address(),
            magic_number.get_address(),
            USIZE_BYTES,
        );
        //? Add one to the magic number
        AddInstruction::new_alloc(
            program_memory,
            magic_number.get_address(),
            &Address::Immediate(Vec::from(1usize.to_le_bytes())),
            magic_number.get_address(),
            USIZE_BYTES,
        );

        let (address_from, length) = (lhs.get_address(), lhs.get_length());
        AddInstruction::new_alloc(
            program_memory,
            address_from,
            magic_number.get_address(),
            destination.get_address(),
            length,
        );
        Ok(())
    }
}

pub struct Equal {}

impl Operation<PointerType> for Equal {
    fn get_symbol(&self) -> Operator {
        Operator::Equal
    }

    fn get_result_type(&self, rhs: &TypeSymbol) -> Option<TypeSymbol> {
        // let rhs = rhs?;
        match rhs {
            TypeSymbol::Pointer => Some(TypeSymbol::Boolean),
            _ => None,
        }
    }

    fn operate(
        &self,
        lhs: &PointerType,
        rhs: &dyn Type,
        destination: &dyn Type,
        program_memory: &mut MemoryManager,
        _stack_sizes: &mut StackSizes,
    ) -> Result<(), String> {
        assert_eq!(destination.get_type_symbol(), TypeSymbol::Boolean);
        assert_eq!(rhs.get_type_symbol(), TypeSymbol::Pointer);

        let (address_from, length) = (lhs.get_address(), lhs.get_length());
        EqualityInstruction::new_alloc(
            program_memory,
            address_from,
            rhs.get_address(),
            destination.get_address(),
            length,
        );
        Ok(())
    }
}

pub struct NotEqual {}

impl Operation<PointerType> for NotEqual {
    fn get_symbol(&self) -> Operator {
        Operator::NotEqual
    }

    fn get_result_type(&self, rhs: &TypeSymbol) -> Option<TypeSymbol> {
        // let rhs = rhs?;
        match rhs {
            TypeSymbol::Pointer => Some(TypeSymbol::Boolean),
            _ => None,
        }
    }

    fn operate(
        &self,
        lhs: &PointerType,
        rhs: &dyn Type,
        destination: &dyn Type,
        program_memory: &mut MemoryManager,
        _stack_sizes: &mut StackSizes,
    ) -> Result<(), String> {
        assert_eq!(destination.get_type_symbol(), TypeSymbol::Boolean);
        assert_eq!(rhs.get_type_symbol(), TypeSymbol::Pointer);

        let (address_from, length) = (lhs.get_address(), lhs.get_length());
        NotEqualInstruction::new_alloc(
            program_memory,
            address_from,
            rhs.get_address(),
            destination.get_address(),
            length,
        );
        Ok(())
    }
}
