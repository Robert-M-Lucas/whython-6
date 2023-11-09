mod assigners;
mod blocks;
mod builtins;
mod keywords;
mod literals;
mod operators;
mod punctuation;
mod types;

pub use assigners::Assigner;
use assigners::AssignerSymbolHandler;
use strum::IntoEnumIterator;

pub use literals::Literal;
use literals::LiteralSymbolHandler;
pub use literals::CHAR_DELIMITER;
pub use literals::STRING_DELIMITER;

pub use operators::Operator;
use operators::OperatorSymbolHandler;

pub use types::TypeSymbol;
use types::TypeSymbolHandler;

pub use blocks::Block;
use blocks::BlockSymbolHandler;

pub use builtins::Builtin;
use builtins::BuiltinSymbolHandler;

pub use punctuation::Punctuation;
pub use punctuation::PunctuationSymbolHandler;
pub use punctuation::LIST_SEPARATOR_CHARACTER;

pub use keywords::Keyword;
pub use keywords::KeywordSymbolHandler;

#[derive(PartialEq, Clone, strum_macros::Display, Debug)]
pub enum Symbol {
    Assigner(Assigner),
    Literal(Literal),
    Operator(Operator),
    BracketedSection(Vec<Symbol>),
    Indexer(Box<Symbol>, Vec<Symbol>),
    List(Vec<Vec<Symbol>>),
    MethodCall(Box<Symbol>, String, Vec<Vec<Symbol>>), // ? Value, method, arguments
    Type(TypeSymbol),
    Block(Block),
    Builtin(Builtin),
    Punctuation(Punctuation),
    Name(Vec<String>), // ? E.g. alpha.bravo -> [alpha, bravo]
    Keyword(Keyword),
}

impl Symbol {
    pub fn get_name_string(name: &[String]) -> Result<String, String> {
        if name.len() != 1 {
            return Err("Name must not be a property".to_string());
        }
        Ok(name.first().unwrap().clone())
    }
}

pub trait SymbolHandler {
    /// Converts a string to a symbol. Returns `None` if no symbol matches the string
    fn get_symbol(string: &str) -> Result<Option<Symbol>, String>;
}

/// Converts a string to a symbol. Returns `None` if no symbol matches the string
pub fn get_all_symbol(string: &str) -> Result<Symbol, String> {
    AllSymbolHandler::get_symbol(string)
}

/// Converts an arithmetic block into a `Literal::ParameterList(parameters)`
pub fn try_bracketed_into_parameters(bracketed: &Symbol) -> Result<Literal, String> {
    fn formatting_error() -> String {
        "Parameters must be formatted ([Type] [Name] , [Type] [Name] , [...])".to_string()
    }

    let list = match bracketed {
        Symbol::BracketedSection(list) => list,
        _ => panic!("Must be bracketed section"),
    };

    if list.is_empty() {
        return Ok(Literal::ParameterList(Vec::new()));
    }

    let mut parameter_list: Vec<(TypeSymbol, String)> = Vec::new();

    let mut i: usize = 0;

    while i < list.len() {
        // Type without name
        if list.len() - i == 1 {
            return Err(formatting_error());
        }

        // No type
        let type_symbol = match list[i] {
            Symbol::Type(type_symbol) => type_symbol,
            _ => return Err(formatting_error()),
        };

        // No name
        let name = match &list[i + 1] {
            Symbol::Name(name) => Symbol::get_name_string(name)?,
            _ => return Err(formatting_error()),
        };

        // Check for list separator
        if i + 2 < list.len() {
            match list[i + 2] {
                Symbol::Punctuation(Punctuation::ListSeparator) => (), // =>
                // {
                //     #[allow(unreachable_patterns)]
                //     match punctuation {
                //         Punctuation::ListSeparator => (),
                //         _ => return Err(formatting_error()),
                //     }
                // }
                _ => return Err(formatting_error()),
            }
        }

        parameter_list.push((type_symbol, name));

        i += 3;
    }

    Ok(Literal::ParameterList(parameter_list))
}

//noinspection SpellCheckingInspection
pub const ALLOWED_CHARS_IN_NAME: &str = "abcdefghijklmnopqrstuvwxyz_";
pub const NAME_SEPARATOR: char = '.';
pub const CLASS_SELF_NAME: &str = "self";
pub const FORBIDDEN_NAMES: [&str; 1] = [CLASS_SELF_NAME];

struct AllSymbolHandler {}

impl AllSymbolHandler {
    fn get_symbol(string: &str) -> Result<Symbol, String> {
        let r = AssignerSymbolHandler::get_symbol(string)
            .transpose()
            .or_else(|| OperatorSymbolHandler::get_symbol(string).transpose())
            .or_else(|| TypeSymbolHandler::get_symbol(string).transpose())
            .or_else(|| BlockSymbolHandler::get_symbol(string).transpose())
            .or_else(|| BuiltinSymbolHandler::get_symbol(string).transpose())
            .or_else(|| LiteralSymbolHandler::get_symbol(string).transpose())
            .or_else(|| PunctuationSymbolHandler::get_symbol(string).transpose())
            .or_else(|| KeywordSymbolHandler::get_symbol(string).transpose());

        if let Some(r) = r {
            return r;
        }

        for c in string.chars() {
            if c != NAME_SEPARATOR && !ALLOWED_CHARS_IN_NAME.contains(c) {
                return Err(format!("Symbol '{string}' not recognised and is not a valid name as it contains the character '{c}'"));
            }
        }

        let name: Vec<_> = string.split('.').map(|s| s.to_string()).collect();
        if name.is_empty() {
            return Err(format!("Symbol '{string}' not recognised"));
        }

        for part in &name {
            for forbidden_name in FORBIDDEN_NAMES {
                if part == forbidden_name {
                    return Err(format!("Name '{}' is reserved", part));
                }
            }

            for keyword in Keyword::iter() {
                if part == keyword.get_code_representation() {
                    return Err(format!("Name '{}' is reserved", part));
                }
            }

            // for keyword in Block::iter() {
            //     if part == keyword.get_code_representation() { return Err(format!("Name '{}' is reserved", part)); }
            // }
            //
            // for keyword in TypeSymbol::iter() {
            //     if part == keyword.get_code_representation() { return Err(format!("Name '{}' is reserved", part)); }
            // }
        }

        Ok(Symbol::Name(name))
    }
}
