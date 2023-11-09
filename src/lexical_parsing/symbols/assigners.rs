use crate::lexical_parsing::symbols::Symbol::BracketedSection;
use super::Operator;
use super::Symbol;
use super::SymbolHandler;

#[derive(PartialEq, Copy, Clone, strum_macros::Display, Debug)]
pub enum Assigner {
    Setter,
    AdditionSetter,
    SubtractionSetter,
    ProductSetter,
    DivisionSetter,
}

impl Assigner {
    pub fn get_expanded_equivalent(&self, lhs: Symbol, rhs: Vec<Symbol>) -> Vec<Symbol> {
        let equivalent = match self {
            Assigner::Setter => {
                return rhs;
            }
            Assigner::AdditionSetter => Operator::Add,
            Assigner::SubtractionSetter => Operator::Subtract,
            Assigner::ProductSetter => Operator::Product,
            Assigner::DivisionSetter => Operator::Divide,
        };

        vec![lhs, Symbol::Operator(equivalent), BracketedSection(rhs)]
    }
}

pub struct AssignerSymbolHandler {}

impl SymbolHandler for AssignerSymbolHandler {
    fn get_symbol(string: &str) -> Result<Option<Symbol>, String> {
        Ok(match string {
            "=" => Some(Symbol::Assigner(Assigner::Setter)),
            "+=" => Some(Symbol::Assigner(Assigner::AdditionSetter)),
            "-=" => Some(Symbol::Assigner(Assigner::SubtractionSetter)),
            "*=" => Some(Symbol::Assigner(Assigner::ProductSetter)),
            "/=" => Some(Symbol::Assigner(Assigner::DivisionSetter)),
            _ => None,
        })
    }
}
