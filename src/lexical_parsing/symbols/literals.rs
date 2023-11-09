use super::{Symbol, TypeSymbol};
use super::SymbolHandler;

#[derive(PartialEq, Clone, strum_macros::Display, Debug)]
pub enum Literal {
    String(String),
    Char(char),
    Int(i128),
    Bool(bool),
    ParameterList(Vec<(TypeSymbol, String)>),
    None,
}

pub struct LiteralSymbolHandler {}

pub const STRING_DELIMITER: char = '"';
pub const CHAR_DELIMITER: char = '\'';

pub const STRING_ESCAPE_CHAR: char = '\\';

const ESCAPE_CODES: [(char, char); 5] = [
    ('n', '\n'),
    ('\\', '\\'),
    ('0', '\0'),
    ('"', '"'),
    ('\'', '\''),
];

/// Takes an input string and replaces escape codes with their corresponding values
fn format_escape_codes(input: String) -> String {
    let mut output = String::new();
    let mut next = false;
    'char_loop: for c in input.chars() {
        if next {
            next = false;
            for code in ESCAPE_CODES {
                if c == code.0 {
                    output.push(code.1);
                    continue 'char_loop;
                }
            }
        }

        if c == '\\' && !next {
            next = true;
        } else {
            output.push(c);
        }
    }
    output
}

impl SymbolHandler for LiteralSymbolHandler {
    fn get_symbol(string: &str) -> Result<Option<Symbol>, String> {
        let result = match string {
            // Boolean
            "true" => Some(Symbol::Literal(Literal::Bool(true))),
            "false" => Some(Symbol::Literal(Literal::Bool(false))),
            "none" => Some(Symbol::Literal(Literal::None)),
            _ => None,
        };

        if result.is_some() {
            return Ok(result);
        }

        let result = {
            let first_char = string.chars().next().unwrap();
            if string.len() >= 2
                && (STRING_DELIMITER == first_char || CHAR_DELIMITER == first_char)
                && string.chars().last().unwrap() == first_char
            {
                let formatted_string = format_escape_codes(string[1..string.len() - 1].to_string());

                if first_char == CHAR_DELIMITER {
                    if formatted_string.len() != 1 {
                        return Err("Char literals cannot contain multiple chars".to_string());
                    }
                }
                Some(Symbol::Literal(Literal::String(formatted_string)));
            }
            None
        };

        if result.is_some() {
            return Ok(result);
        }

        return Ok(match string.parse::<i128>() {
            Ok(ok) => Some(Symbol::Literal(Literal::Int(ok))),
            Err(_) => None,
        });
    }
}

// impl Literal {
//     pub(crate) fn get_name(&self) -> &str {
//         return match self {
//             Literal::StringLiteral(_) => "StringLiteral",
//             Literal::IntLiteral(_) => "IntLiteral",
//             Literal::BoolLiteral(_) => "BoolLiteral",
//             Literal::ParameterList(_) => "ParameterList",
//             Literal::None => "None",
//         }
//     }
// }
