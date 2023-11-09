use super::Symbol;
use super::SymbolHandler;

#[derive(PartialEq, Copy, Clone, strum_macros::Display, Debug, strum_macros::EnumIter)]
pub enum Block {
    While,
    Loop,
    If,
    Elif,
    Else,
    Function,
    Class,
    BaseBlock,
}

pub struct BlockSymbolHandler {}

impl Block {
    pub fn get_code_representation(&self) -> &str {
        match self {
            Block::While => "while",
            Block::Loop => "loop",
            Block::If => "if",
            Block::Elif => "elif",
            Block::Else => "else",
            Block::Function => "fn",
            Block::Class => "class",
            Block::BaseBlock => "block",
        }
    }
}

impl SymbolHandler for BlockSymbolHandler {
    fn get_symbol(string: &str) -> Result<Option<Symbol>, String> {
        Ok(match string {
            "while" => Some(Symbol::Block(Block::While)),
            "loop" => Some(Symbol::Block(Block::Loop)),
            "if" => Some(Symbol::Block(Block::If)),
            "elif" => Some(Symbol::Block(Block::Elif)),
            "else" => Some(Symbol::Block(Block::Else)),
            "fn" => Some(Symbol::Block(Block::Function)),
            "class" => Some(Symbol::Block(Block::Class)),
            "block" => Some(Symbol::Block(Block::BaseBlock)),
            _ => None,
        })
    }
}
