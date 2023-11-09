use super::{Symbol, TypeSymbol};
use super::SymbolHandler;

#[derive(PartialEq, Clone, strum_macros::Display, Debug)]
pub enum Operator {
    Cast(TypeSymbol),
    Add,
    Subtract,
    Product,
    Divide,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Equal,
    NotEqual,
    Or,
    And,
    Not,
}

pub struct OperatorSymbolHandler {}

impl SymbolHandler for OperatorSymbolHandler {
    fn get_symbol(string: &str) -> Result<Option<Symbol>, String> {
        Ok(match string {
            "+" => Some(Symbol::Operator(Operator::Add)),
            "-" => Some(Symbol::Operator(Operator::Subtract)),
            "*" => Some(Symbol::Operator(Operator::Product)),
            "/" => Some(Symbol::Operator(Operator::Divide)),
            ">" => Some(Symbol::Operator(Operator::Greater)),
            "<" => Some(Symbol::Operator(Operator::Less)),
            ">=" => Some(Symbol::Operator(Operator::GreaterEqual)),
            "<=" => Some(Symbol::Operator(Operator::LessEqual)),
            "==" => Some(Symbol::Operator(Operator::Equal)),
            "!=" => Some(Symbol::Operator(Operator::NotEqual)),
            "|" => Some(Symbol::Operator(Operator::Or)),
            "&" => Some(Symbol::Operator(Operator::And)),
            "!" => Some(Symbol::Operator(Operator::Not)),
            _ => None, // string => {
                       //     // TODO: Error if invalid type symbol but '<x>' is still present?
                       //     if string.len() > 2 {
                       //         if string.starts_with('<') && string.ends_with('>') {
                       //             let type_name = substring(string, 1, string.len() - 2);
                       //             let type_symbol = TypeSymbolHandler::get_raw_symbol(type_name.as_str());
                       //
                       //             type_symbol.map(|type_symbol| Symbol::Operator(Operator::Cast(type_symbol)))
                       //         } else {
                       //             None
                       //         }
                       //     } else {
                       //         None
                       //     }
                       // }
        })
    }
}

// impl Operator {
//     pub(crate) fn get_name(&self) -> &str {
//         return match self {
//             Operator::Add => "Add",
//             Operator::Subtract => "Subtract",
//             Operator::Product => "Product",
//             Operator::Divide => "Divide",
//             Operator::Greater => "Greater",
//             Operator::Less => "Less",
//             Operator::GreaterEqual => "GreaterEqual",
//             Operator::LessEqual => "LessEqual",
//             Operator::Equal => "Equal",
//             Operator::NotEqual => "NotEqual",
//             Operator::Or => "Or",
//             Operator::And => "And",
//             Operator::Not => "Not"
//         }
//     }
// }
