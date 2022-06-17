use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CharClass {
    Digit,
    Letter,
    Plus,
    Minus,
    Aster,
    Slash,
    Lparen,
    Rparen,
    Equal,
    Lss,
    Gtr,
    Comma,
    Period,
    Semicolon,
    Colon,
    Other
}

impl CharClass {
    pub fn from_u8(b: u8) -> Self {
        match b {
            b'0'..=b'9' => {
                CharClass::Digit
            },
            b'a'..=b'z' | b'A'..=b'Z' => {
                CharClass::Letter
            },
            b'+' => CharClass::Plus,
            b'-' => CharClass::Minus,
            b'*' => CharClass::Aster,
            b'/' => CharClass::Slash,
            b'(' => CharClass::Lparen,
            b')' => CharClass::Rparen,
            b'=' => CharClass::Equal,
            b'<' => CharClass::Lss,
            b'>' => CharClass::Gtr,
            b',' => CharClass::Comma,
            b'.' => CharClass::Dot,
            b';' => CharClass::SemiColon,
            b':' => CharClass::Colon,
               _ => CharClass::Other
        }
    }

    pub fn is_reserved(&self) -> bool {
        match self {
            &CharClass::Other => false,
                       _ => true
        }
    }
}