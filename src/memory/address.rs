use either::{Either, Left, Right};
use crate::memory::address::Address::{HeapDirect, HeapIndirect, Immediate, ProgramDirect, ProgramIndirect, StackDirect, StackIndirect};
use crate::memory::runtime_memory::RuntimeMemory;
use crate::util::{read_usize, USIZE_BYTES};

const ADDRESS_CODE_BYTES: usize = 1;
const IMMEDIATE_CODE: u8 = 0;
const STACK_DIRECT_CODE: u8 = 2;
const STACK_INDIRECT_CODE: u8 = 3;
const HEAP_DIRECT_CODE: u8 = 5;
const HEAP_INDIRECT_CODE: u8 = 6;
const PROGRAM_DIRECT_CODE: u8 = 8;
const PROGRAM_INDIRECT_CODE: u8 = 9;

#[derive(Clone)]
pub enum Address {
    Immediate(Vec<u8>),
    StackDirect(usize),
    StackIndirect(usize),
    HeapDirect(usize),
    HeapIndirect(usize),
    ProgramDirect(usize),
    ProgramIndirect(usize),
}

impl Address {
    pub fn get_address(data: &[u8], expected_len: Option<usize>) -> (Address, usize) {
        let code = data[0];
        match code {
            IMMEDIATE_CODE =>
                (
                    Immediate(
                        Vec::from(
                            &data[ADDRESS_CODE_BYTES..ADDRESS_CODE_BYTES + expected_len.expect("Received unexpected immediate address")]
                        )
                    ),
                    ADDRESS_CODE_BYTES + expected_len.unwrap()
                ),
            STACK_DIRECT_CODE => (StackDirect(read_usize(&data[ADDRESS_CODE_BYTES..])), ADDRESS_CODE_BYTES + USIZE_BYTES),
            STACK_INDIRECT_CODE => (StackIndirect(read_usize(&data[ADDRESS_CODE_BYTES..])), ADDRESS_CODE_BYTES + USIZE_BYTES),
            HEAP_DIRECT_CODE => (HeapDirect(read_usize(&data[ADDRESS_CODE_BYTES..])), ADDRESS_CODE_BYTES + USIZE_BYTES),
            HEAP_INDIRECT_CODE => (HeapIndirect(read_usize(&data[ADDRESS_CODE_BYTES..])), ADDRESS_CODE_BYTES + USIZE_BYTES),
            PROGRAM_DIRECT_CODE => (ProgramDirect(read_usize(&data[ADDRESS_CODE_BYTES..])), ADDRESS_CODE_BYTES + USIZE_BYTES),
            PROGRAM_INDIRECT_CODE => (ProgramIndirect(read_usize(&data[ADDRESS_CODE_BYTES..])), ADDRESS_CODE_BYTES + USIZE_BYTES),
            _ => panic!("Invalid address code '{}'", code)
        }
    }

    pub fn evaluate_direct_address<'a>(address: &Address, runtime_memory: &'a RuntimeMemory) -> &'a [u8] {
        match address {
            StackDirect(address) => runtime_memory.stack().get_location(*address),
            StackIndirect(address) => runtime_memory.heap().get_location(*address),
            ProgramIndirect(address) => runtime_memory.program().get_location(*address),
            _ => panic!("Non-direct address")
        }
    }

    pub fn follow_indirect_address(address: &Address, runtime_memory: &RuntimeMemory) -> Address {
        match address {
            StackIndirect(address) => Address::get_address(runtime_memory.stack().get_location(*address), None).0,
            HeapIndirect(address) => Address::get_address(runtime_memory.heap().get_location(*address), None).0,
            ProgramIndirect(address) => Address::get_address(runtime_memory.program().get_location(*address), None).0,
            _ => panic!("Non-indirect address!")
        }
    }

    pub fn evaluate_to_direct(self, runtime_memory: &RuntimeMemory) -> Address {
        let address = match self {
            Immediate(_) => panic!("Immediate address can't be evaluated to a direct address"),
            StackDirect(_) | HeapDirect(_) | ProgramDirect(_) => return self,
            StackIndirect(_) | HeapIndirect(_) | ProgramIndirect(_) => Address::follow_indirect_address(&self, runtime_memory)
        };

        address.evaluate_to_direct(runtime_memory)
    }

    pub fn evaluate_address_to_data(self, runtime_memory: &RuntimeMemory) -> Either<&[u8], Vec<u8>> {
        let address = match self {
            Immediate(data) => return Right(data),
            StackDirect(_) | HeapDirect(_) | ProgramDirect(_) => return Left(Address::evaluate_direct_address(&self, runtime_memory)),
            StackIndirect(_) | HeapIndirect(_) | ProgramIndirect(_) => Address::follow_indirect_address(&self, runtime_memory)
        };

        address.evaluate_address_to_data(runtime_memory)
    }


    pub fn get_bytes(&self) -> Vec<u8> {
        fn make_vec(code: u8, address: usize) -> Vec<u8> {
            let mut v = Vec::with_capacity(ADDRESS_CODE_BYTES + USIZE_BYTES);
            v.push(code);
            v.extend(address.to_le_bytes());
            v
        }

        match self {
            Address::Immediate(data) => {
                let mut v = Vec::with_capacity(ADDRESS_CODE_BYTES + data.len());
                v.push(IMMEDIATE_CODE);
                v.extend(data);
                v
            }
            Address::StackDirect(address) => { make_vec(STACK_DIRECT_CODE, *address) }
            Address::StackIndirect(address) => { make_vec(STACK_INDIRECT_CODE, *address) }
            Address::HeapDirect(address) => { make_vec(HEAP_DIRECT_CODE, *address) }
            Address::HeapIndirect(address) => { make_vec(HEAP_INDIRECT_CODE, *address) }
            Address::ProgramDirect(address) => { make_vec(PROGRAM_DIRECT_CODE, *address) }
            Address::ProgramIndirect(address) => { make_vec(PROGRAM_INDIRECT_CODE, *address) }
        }
    }
}