use std::fmt;

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
    type Err = UndefinedSymbol;

    pub fn try_from(b: &[u8]) -> Result<Self, Self::Err> {
        match b {
            b"+"  => Ok(Symbol::Plus),
            b"-"  => Ok(Symbol::Minus),
            b"*"  => Ok(Symbol::Asterisk),
            b"/"  => Ok(Symbol::Slash),
            b"("  => Ok(Symbol::Lparen),
            b")"  => Ok(Symbol::Rparen),
            b"["  => Ok(Symbol::SqParL),
            b"]"  => Ok(Symbol::SqParR),
            b"="  => Ok(Symbol::Equal),
            b"<"  => Ok(Symbol::Lss),
            b">"  => Ok(Symbol::Gtr),
            b"<>" => Ok(Symbol::NotEq),
            b"<=" => Ok(Symbol::LssEq),
            b">=" => Ok(Symbol::GtrEq),
            b","  => Ok(Symbol::Comma),
            b"."  => Ok(Symbol::Period),
            b";"  => Ok(Symbol::SemiColon),
            b":=" => Ok(Symbol::Assign)
                _ => Err(UndefinedSymbol),
        }
    }
}