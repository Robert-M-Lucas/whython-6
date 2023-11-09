use super::Symbol;
use super::SymbolHandler;

#[derive(PartialEq, Copy, Clone, strum_macros::Display, Debug)]
pub enum Punctuation {
    ListSeparator,
}

pub struct PunctuationSymbolHandler {}

pub const LIST_SEPARATOR_CHARACTER: char = ',';

impl SymbolHandler for PunctuationSymbolHandler {
    fn get_symbol(string: &str) -> Result<Option<Symbol>, String> {
        Ok(
            if string.len() == 1 && string.starts_with(LIST_SEPARATOR_CHARACTER) {
                Some(Symbol::Punctuation(Punctuation::ListSeparator))
            } else {
                None
            },
        )
    }
}

// impl Punctuation {
//     pub(crate) fn get_name(&self) -> &str {
//         return match self {
//             Punctuation::ListSeparator => "ListSeparator"
//         }
//     }
// }
