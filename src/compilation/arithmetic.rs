use crate::lexical_parsing::symbols::{Keyword, Operator, Symbol, TypeSymbol};
use crate::memory::MemoryManager;
use crate::processing::blocks::StackSizes;
use crate::processing::reference_manager::ReferenceStack;
use crate::processing::symbols::{Keyword, Operator, Symbol, TypeSymbol};
use crate::processing::types::{Type, TypeFactory};
use crate::util::ref_or_box::RefOrBox;

/*
macro_rules! get_variable {
    ($output: expr, $symbol: expr, $program_memory: expr, $reference_stack: expr, $stack_sizes: expr) => {
        let store;
        $output = match $symbol {
            Symbol::Name(name) =>
                Ok($reference_stack.get_reference(name.as_str())?.get_variable()?),
            Symbol::Literal(literal) =>
                {
                    store = TypeFactory::get_default_instantiated_type_for_literal(literal, $stack_sizes, $program_memory)?;
                    Ok(&store)
                },
            Symbol::ArithmeticBlock(section) => {
                store = evaluate_arithmetic_to_any_type(section, $program_memory, $reference_stack, $stack_sizes)?;
                Ok(&store)
            }
            _ => Err("Operator must be followed by a Literal or Name".to_string())
        };
    };
}
*/

pub enum ReturnOptions<'a> {
    /// Places the calculated value into the type - returns `None`
    IntoType(&'a dyn Type), //, Option<Box<dyn FnOnce(&mut MemoryManager, &mut StackSizes)>>, usize),
    /// Returns a type from the specified list
    OneOfTypes(&'a [TypeSymbol]),
    /// Can return any type but will return specified type if possible
    PreferType(TypeSymbol),
    /// Returns any type
    AnyType,
}

impl<'a> ReturnOptions<'a> {
    // pub fn remove_return_into_type_options(&mut self) {
    //     match self {
    //         ReturnOptions::ReturnIntoType(_, run_before_last_step, offset) => {
    //             *run_before_last_step = None;
    //             *offset = 0;
    //         }
    //         _ => {}
    //     }
    // }

    pub fn get_prefered_type(&self) -> Option<TypeSymbol> {
        match self {
            ReturnOptions::IntoType(into) => Some(into.get_type_symbol()),
            ReturnOptions::OneOfTypes(options) => Some(options[0]),
            ReturnOptions::PreferType(prefered) => Some(*prefered),
            ReturnOptions::AnyType => None,
        }
    }
}

/// Evaluates an arithmetic section and puts the result into a type
pub fn evaluate_arithmetic_into_type(
    section: &[Symbol],
    destination: &dyn Type,
    program_memory: &mut MemoryManager,
    reference_stack: &ReferenceStack,
    stack_sizes: &mut StackSizes,
    // run_before_last_step: Option<fn(&mut MemoryManager, &mut StackSizes)>,
    // offset: usize
) -> Result<(), String> {
    evaluate_arithmetic_section(
        section,
        &ReturnOptions::IntoType(destination), //, run_before_last_step, offset),
        program_memory,
        reference_stack,
        stack_sizes,
    )?;
    Ok(())
}

/// Evaluates an arithmetic section and returns a type from the specified list
pub fn evaluate_arithmetic_to_types<'a>(
    section: &[Symbol],
    return_type_options: &[TypeSymbol],
    program_memory: &mut MemoryManager,
    reference_stack: &'a ReferenceStack,
    stack_sizes: &mut StackSizes,
) -> Result<RefOrBox<'a, dyn Type + 'a>, String> {
    Ok(evaluate_arithmetic_section(
        section,
        &ReturnOptions::OneOfTypes(return_type_options),
        program_memory,
        reference_stack,
        stack_sizes,
    )?
    .unwrap())
}

/// Evaluates an arithmetic section and returns any type
pub fn evaluate_arithmetic_to_any_type<'a>(
    section: &[Symbol],
    program_memory: &mut MemoryManager,
    reference_stack: &'a ReferenceStack,
    stack_sizes: &mut StackSizes,
) -> Result<RefOrBox<'a, dyn Type + 'a>, String> {
    Ok(evaluate_arithmetic_section(
        section,
        &ReturnOptions::AnyType,
        program_memory,
        reference_stack,
        stack_sizes,
    )?
    .unwrap())
}

fn evaluate_arithmetic_section<'a>(
    section: &[Symbol],
    return_options: &ReturnOptions<'_>,
    program_memory: &mut MemoryManager,
    reference_stack: &'a ReferenceStack,
    stack_sizes: &mut StackSizes,
) -> Result<Option<RefOrBox<'a, dyn Type + 'a>>, String> {
    //noinspection SpellCheckingInspection
    fn get_formatting_error() -> String {
        "Arithmetic sections must be formated [Operator] [Value], [Value] [Operator] [Value] or [Value] as [Type]".to_string()
    }

    if section.is_empty() {
        return Err("Cannot evaluate a section with no symbols".to_string());
    }

    // ? No operation
    if section.len() == 1 {
        return handle_single_symbol(
            &section[0],
            return_options,
            program_memory,
            reference_stack,
            stack_sizes,
        );
    }

    match &section[0] {
        // ? Prefix operator e.g. ! A
        Symbol::Operator(operator) => {
            if section.len() != 2 {
                return Err("Operator must be followed by a Literal or Name".to_string());
            }

            let return_option = if let Some(preference) = return_options.get_prefered_type() {
                ReturnOptions::PreferType(preference)
            } else {
                ReturnOptions::AnyType
            };

            let operand = handle_single_symbol(
                &section[1],
                &return_option,
                program_memory,
                reference_stack,
                stack_sizes,
            )?
            .unwrap();

            handle_prefix_operation(
                operator,
                operand,
                return_options,
                program_memory,
                stack_sizes,
            )
        }
        // ? Normal operation e.g. A + B or Casting e.g. A as bool
        _ => {
            if section.len() != 3 {
                return Err(get_formatting_error());
            }

            match &section[1] {
                // ? Casting
                Symbol::Keyword(Keyword::As) => {
                    let type_symbol = match &section[2] {
                        Symbol::Type(type_symbol) => type_symbol,
                        _ => {
                            return Err(get_formatting_error());
                        }
                    };

                    handle_casting(
                        &section[0],
                        type_symbol,
                        return_options,
                        program_memory,
                        reference_stack,
                        stack_sizes,
                    )
                }
                // ? Normal operation
                Symbol::Operator(operator) => {
                    let return_option = if let Some(preference) = return_options.get_prefered_type()
                    {
                        ReturnOptions::PreferType(preference)
                    } else {
                        ReturnOptions::AnyType
                    };

                    let lhs = handle_single_symbol(
                        &section[0],
                        &return_option,
                        program_memory,
                        reference_stack,
                        stack_sizes,
                    )?
                    .unwrap();

                    let rhs = handle_single_symbol(
                        &section[2],
                        &return_option,
                        program_memory,
                        reference_stack,
                        stack_sizes,
                    )?
                    .unwrap();

                    handle_operation(
                        operator,
                        lhs,
                        rhs,
                        return_options,
                        program_memory,
                        stack_sizes,
                    )
                }
                _ => Err(get_formatting_error()),
            }
        }
    }
}

fn incorrect_type_error(expected: &[TypeSymbol], received: &[TypeSymbol]) -> String {
    let mut expected_text = "[any]".to_string();
    if !expected.is_empty() {
        expected_text = "[".to_string();
        for e in expected {
            expected_text += (e.to_string() + ", ").as_str();
        }
        expected_text = expected_text[..expected_text.len() - 2].to_string();
    }

    let mut received_text = "[any]".to_string();
    if !received.is_empty() {
        received_text = "[".to_string();
        for r in received {
            received_text += (r.to_string() + ", ").as_str();
        }
        received_text = received_text[..received_text.len() - 2].to_string();
    }

    format!(
        "Expected type {}, received {}",
        expected_text, received_text
    )
}

fn handle_single_symbol<'a>(
    symbol: &Symbol,
    return_options: &ReturnOptions,
    program_memory: &mut MemoryManager,
    reference_stack: &'a ReferenceStack,
    stack_sizes: &mut StackSizes,
) -> Result<Option<RefOrBox<'a, dyn Type + 'a>>, String> {
    match symbol {
        Symbol::Name(name) => {
            let variable = reference_stack.get_reference(name)?.get_variable_ref()?;
            match return_options {
                ReturnOptions::IntoType(output) => {
                    //, run_before_last_step, offset) => {
                    // if let Some(f) = run_before_last_step {
                    //     f(program_memory, stack_sizes);
                    // }
                    output.runtime_copy_from(variable, program_memory)?; //, *offset)?;
                    Ok(None)
                }
                ReturnOptions::AnyType | ReturnOptions::PreferType(_) => {
                    Ok(Some(RefOrBox::from_ref(variable)))
                }
                ReturnOptions::OneOfTypes(types) => {
                    let variable_type = variable.get_type_symbol();
                    if !types.is_empty() && !types.iter().any(|t| *t == variable_type) {
                        Err(incorrect_type_error(types, &[variable_type]))
                    } else {
                        Ok(Some(RefOrBox::from_ref(variable)))
                    }
                }
            }
        }
        Symbol::Literal(literal) => {
            match return_options {
                ReturnOptions::IntoType(output) => {
                    //, run_before_last_step, offset) => {
                    // if let Some(f) = run_before_last_step {
                    //     f(program_memory, stack_sizes);
                    // }
                    output.runtime_copy_from_literal(literal, program_memory)?;
                    Ok(None)
                }
                ReturnOptions::AnyType => Ok(Some(RefOrBox::from_box(
                    TypeFactory::get_default_instantiated_type_for_literal(
                        literal,
                        stack_sizes,
                        program_memory,
                        None,
                    )?,
                ))),
                ReturnOptions::PreferType(preferred) => Ok(Some(RefOrBox::from_box(
                    TypeFactory::get_default_instantiated_type_for_literal(
                        literal,
                        stack_sizes,
                        program_memory,
                        Some(preferred),
                    )?,
                ))),
                ReturnOptions::OneOfTypes(types) => {
                    // TODO: Potentially request types from literals i.e. not default
                    let default_type = TypeFactory::get_default_instantiated_type_for_literal(
                        literal,
                        stack_sizes,
                        program_memory,
                        None,
                    )?;
                    let default_type_type = default_type.get_type_symbol();
                    if !types.is_empty() && !types.iter().any(|t| *t == default_type_type) {
                        Err(incorrect_type_error(types, &[default_type_type]))
                    } else {
                        Ok(Some(RefOrBox::from_box(default_type)))
                    }
                }
            }
        }
        Symbol::BracketedSection(section) => {
            // return_options.remove_return_into_type_options();
            evaluate_arithmetic_section(
                section,
                return_options,
                program_memory,
                reference_stack,
                stack_sizes,
            )
        }
        _ => Err("Expected an expression".to_string()),
    }
}

fn operator_not_implemented_error(
    lhs: &TypeSymbol,
    operator: &Operator,
    rhs: Option<&TypeSymbol>,
) -> String {
    if let Some(rhs) = rhs {
        format!("{} not supported between {} and {}", operator, lhs, rhs)
    } else {
        format!("{} not supported on {}", operator, lhs)
    }
}

// TODO: Consider removing unused arguments
fn handle_prefix_operation<'a>(
    operator: &Operator,
    operand: RefOrBox<'a, dyn Type + 'a>,
    return_options: &ReturnOptions,
    program_memory: &mut MemoryManager,
    stack_sizes: &mut StackSizes,
) -> Result<Option<RefOrBox<'a, dyn Type + 'a>>, String> {
    let operand = operand.as_ref();

    match return_options {
        ReturnOptions::IntoType(output) => {
            //, run_before_last_step, offset) => {
            // if let Some(f) = run_before_last_step {
            //     f(program_memory, stack_sizes);
            // }
            operand.operate_prefix(operator, *output, program_memory, stack_sizes)?;
            Ok(None)
        }
        ReturnOptions::AnyType | ReturnOptions::PreferType(_) => {
            let return_types = operand.get_prefix_operation_result_type(operator);
            if return_types.is_empty() {
                Err(operator_not_implemented_error(
                    &operand.get_type_symbol(),
                    operator,
                    None,
                ))
            } else {
                let mut new_type = TypeFactory::get_unallocated_type(&return_types[0])?;
                new_type.allocate_variable(stack_sizes, program_memory)?;
                operand.operate_prefix(operator, new_type.as_ref(), program_memory, stack_sizes)?;

                Ok(Some(RefOrBox::from_box(new_type)))
            }
        }
        ReturnOptions::OneOfTypes(types) => {
            let return_types = operand.get_prefix_operation_result_type(operator);

            if return_types.is_empty() {
                return Err(operator_not_implemented_error(
                    &operand.get_type_symbol(),
                    operator,
                    None,
                ));
            }

            let return_type = return_types.iter().find(|_t| {
                for rt in types.iter() {
                    if *rt == **_t {
                        return true;
                    }
                }
                false
            });

            if let Some(return_type) = return_type {
                let mut new_type = TypeFactory::get_unallocated_type(return_type)?;
                new_type.allocate_variable(stack_sizes, program_memory)?;
                operand.operate_prefix(operator, new_type.as_ref(), program_memory, stack_sizes)?;

                Ok(Some(RefOrBox::from_box(new_type)))
            } else {
                Err(incorrect_type_error(types, &return_types))
            }
        }
    }
}

fn handle_operation<'a>(
    operator: &Operator,
    lhs: RefOrBox<'a, dyn Type + 'a>,
    rhs: RefOrBox<'a, dyn Type + 'a>,
    return_options: &ReturnOptions,
    program_memory: &mut MemoryManager,
    stack_sizes: &mut StackSizes,
) -> Result<Option<RefOrBox<'a, dyn Type + 'a>>, String> {
    let lhs = lhs.as_ref();
    let rhs = rhs.as_ref();

    match return_options {
        ReturnOptions::IntoType(output) => {
            // , run_before_last_step, offset) => {
            // if let Some(f) = run_before_last_step {
            //     f(program_memory, stack_sizes);
            // }
            lhs.operate(operator, rhs, *output, program_memory, stack_sizes)?;
            Ok(None)
        }
        ReturnOptions::AnyType | ReturnOptions::PreferType(_) => {
            let return_types = lhs.get_operation_result_type(operator, &rhs.get_type_symbol());
            if return_types.is_empty() {
                Err(operator_not_implemented_error(
                    &lhs.get_type_symbol(),
                    operator,
                    Some(&rhs.get_type_symbol()),
                ))
            } else {
                let mut new_type = TypeFactory::get_unallocated_type(&return_types[0])?;
                new_type.allocate_variable(stack_sizes, program_memory)?;
                lhs.operate(
                    operator,
                    rhs,
                    new_type.as_ref(),
                    program_memory,
                    stack_sizes,
                )?;

                Ok(Some(RefOrBox::from_box(new_type)))
            }
        }
        ReturnOptions::OneOfTypes(types) => {
            let return_types = lhs.get_operation_result_type(operator, &rhs.get_type_symbol());

            if return_types.is_empty() {
                return Err(operator_not_implemented_error(
                    &lhs.get_type_symbol(),
                    operator,
                    Some(&rhs.get_type_symbol()),
                ));
            }

            let return_type = return_types.iter().find(|_t| {
                for rt in types.iter() {
                    if *rt == **_t {
                        return true;
                    }
                }
                false
            });

            if let Some(return_type) = return_type {
                let mut new_type = TypeFactory::get_unallocated_type(return_type)?;
                new_type.allocate_variable(stack_sizes, program_memory)?;
                lhs.operate(
                    operator,
                    rhs,
                    new_type.as_ref(),
                    program_memory,
                    stack_sizes,
                )?;

                Ok(Some(RefOrBox::from_box(new_type)))
            } else {
                Err(incorrect_type_error(types, &return_types))
            }
        }
    }
}

fn handle_casting<'a>(
    symbol: &Symbol,
    type_symbol: &TypeSymbol,
    return_options: &ReturnOptions,
    program_memory: &mut MemoryManager,
    reference_stack: &ReferenceStack,
    stack_sizes: &mut StackSizes,
) -> Result<Option<RefOrBox<'a, dyn Type + 'a>>, String> {
    match symbol {
        Symbol::Literal(literal) => {
            // ? Ignore cast if going into correct type
            if let ReturnOptions::IntoType(output) = return_options {
                if output.get_type_symbol() == *type_symbol {
                    output.runtime_copy_from_literal(literal, program_memory)?;
                    return Ok(None);
                }
            }

            let mut new_type = TypeFactory::get_unallocated_type(type_symbol)?;
            new_type.allocate_variable(stack_sizes, program_memory)?;
            new_type.runtime_copy_from_literal(literal, program_memory)?;

            match return_options {
                ReturnOptions::IntoType(output) => {
                    output.runtime_copy_from(new_type.as_ref(), program_memory)?;
                    Ok(None)
                }
                ReturnOptions::OneOfTypes(return_types) => {
                    if return_types.iter().any(|t| *t == *type_symbol) {
                        Ok(Some(RefOrBox::from_box(new_type)))
                    } else {
                        let return_type = TypeFactory::get_unallocated_type(&return_types[0])?;
                        return_type.runtime_copy_from(new_type.as_ref(), program_memory)?;
                        Ok(Some(RefOrBox::from_box(return_type)))
                    }
                }
                ReturnOptions::AnyType | ReturnOptions::PreferType(_) => {
                    Ok(Some(RefOrBox::from_box(new_type)))
                }
            }
        }
        _ => {
            let value = evaluate_arithmetic_to_any_type(
                &[symbol.clone()],
                program_memory,
                reference_stack,
                stack_sizes,
            )?;
            let value = value.as_ref();

            // ? Ignore cast if going into correct type
            if let ReturnOptions::IntoType(output) = return_options {
                if output.get_type_symbol() == *type_symbol {
                    output.runtime_copy_from(value, program_memory)?;
                    return Ok(None);
                }
            }

            let mut new_type = TypeFactory::get_unallocated_type(type_symbol)?;
            new_type.allocate_variable(stack_sizes, program_memory)?;
            new_type.runtime_copy_from(value, program_memory)?;

            match return_options {
                ReturnOptions::IntoType(output) => {
                    output.runtime_copy_from(new_type.as_ref(), program_memory)?;
                    Ok(None)
                }
                ReturnOptions::OneOfTypes(return_types) => {
                    if return_types.iter().any(|t| *t == *type_symbol) {
                        Ok(Some(RefOrBox::from_box(new_type)))
                    } else {
                        let return_type = TypeFactory::get_unallocated_type(&return_types[0])?;
                        return_type.runtime_copy_from(new_type.as_ref(), program_memory)?;
                        Ok(Some(RefOrBox::from_box(return_type)))
                    }
                }
                ReturnOptions::AnyType | ReturnOptions::PreferType(_) => {
                    Ok(Some(RefOrBox::from_box(new_type)))
                }
            }
        }
    }
}
