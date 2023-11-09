use super::Symbol;
use super::SymbolHandler;

#[derive(PartialEq, Copy, Clone, strum_macros::Display, Debug, strum_macros::EnumIter)]
pub enum Builtin {
    Print,
    PrintChars,
    Input,
}

pub struct BuiltinSymbolHandler {}

impl SymbolHandler for BuiltinSymbolHandler {
    //noinspection SpellCheckingInspection
    fn get_symbol(string: &str) -> Result<Option<Symbol>, String> {
        Ok(match string {
            "print" => Some(Symbol::Builtin(Builtin::Print)),
            "printc" => Some(Symbol::Builtin(Builtin::PrintChars)),
            "input" => Some(Symbol::Builtin(Builtin::Input)),
            _ => None,
        })
    }
}
