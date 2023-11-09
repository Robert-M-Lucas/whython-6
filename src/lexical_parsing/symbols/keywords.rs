use super::Symbol;
use super::SymbolHandler;

#[derive(PartialEq, Copy, Clone, strum_macros::Display, Debug, strum_macros::EnumIter)]
pub enum Keyword {
    Break,
    Continue,
    Dump,
    ViewMemory,
    ViewMemoryDecimal,
    As,
    Import,
}

pub struct KeywordSymbolHandler {}

impl Keyword {
    //noinspection SpellCheckingInspection
    //noinspection SpellCheckingInspection
    pub fn get_code_representation(&self) -> &str {
        match self {
            Keyword::Break => "break",
            Keyword::Continue => "continue",
            Keyword::Dump => "dump",
            Keyword::ViewMemory => "viewmem",
            Keyword::ViewMemoryDecimal => "viewmemdec",
            Keyword::As => "as",
            Keyword::Import => "import",
        }
    }
}

impl SymbolHandler for KeywordSymbolHandler {
    //noinspection SpellCheckingInspection
    fn get_symbol(string: &str) -> Result<Option<Symbol>, String> {
        Ok(match string {
            "break" => Some(Symbol::Keyword(Keyword::Break)),
            "continue" => Some(Symbol::Keyword(Keyword::Continue)),
            "dump" => Some(Symbol::Keyword(Keyword::Dump)),
            "viewmem" => Some(Symbol::Keyword(Keyword::ViewMemory)),
            "viewmemdec" => Some(Symbol::Keyword(Keyword::ViewMemoryDecimal)),
            "as" => Some(Symbol::Keyword(Keyword::As)),
            "import" => Some(Symbol::Keyword(Keyword::Import)),
            _ => None,
        })
    }
}
