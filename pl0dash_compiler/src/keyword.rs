use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct UndefinedKeywordError;

impl fmt::Display for UndefinedKeywordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Undefined keyword")
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Keyword {
    Begin,
    End,
    If,
    Then,
    While,
    Do,
    Ret,
    Func,
    Var,
    Const,
    Odd,
    Write,
    WriteLn,
}

impl TryFrom<&str> for Keyword {
    type Err = UndefinedKeywordError;

    fn try_from(s: &str) -> Result<Self, Self::Err> {
        match s {
            "begin"    => Ok(Keyword::Begin),
            "end"      => Ok(Keyword::End),
            "if"       => Ok(Keyword::If),
            "then"     => Ok(Keyword::Then),
            "while"    => Ok(Keyword::While),
            "do"       => Ok(Keyword::Do),
            "return"   => Ok(Keyword::Ret),
            "function" => Ok(Keyword::Func),
            "var"      => Ok(Keyword::Var),
            "const"    => Ok(Keyword::Const),
            "odd"      => Ok(Keyword::Odd),
            "write"    => Ok(Keyword::Write),
            "writeln"  => Ok(Keyword::WriteLn),
                     _ => Err(UndefinedKeywordError),
        }
    }
}