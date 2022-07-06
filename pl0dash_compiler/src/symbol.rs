use std::fmt;
use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Symbol {
    Plus,
    Minus,
    Mult,
    Div,
    Lparen,
    Rparen,
    Equal,
    Lss,
    Gtr,
    NotEq,
    LssEq,
    GtrEq,
    Comma,
    Period,
    SemiColon,
    Assign,
}

#[derive(Debug, Clone)]
pub struct UndefinedSymbol;

impl fmt::Display for UndefinedSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Undefined symbol")
    }
}

impl TryFrom<&[u8]> for Symbol {
    type Error = UndefinedSymbol;

    fn try_from(b: &[u8]) -> Result<Self, Self::Error> {
        match b {
            CharClass::Plus      => Ok(Symbol::Plus),
            CharClass::Minus     => Ok(Symbol::Minus),
            CharClass::Aster     => Ok(Symbol::Mult),
            CharClass::Slash     => Ok(Symbol::Div),
            CharClass::Lparen    => Ok(Symbol::Lparen),
            CharClass::Rparen    => Ok(Symbol::Rparen),
            CharClass::Equal     => Ok(Symbol::Equal),
            CharClass::Lss       => Ok(Symbol::Lss),
            CharClass::Gtr       => Ok(Symbol::Gtr),
            CharClass::Comma     => Ok(Symbol::Comma),
            CharClass::Period    => Ok(Symbol::Period),
            CharClass::SemiColon => Ok(Symbol::SemiColon),
                               _ => Err(UndefinedSymbol),
        }
    }
}