use std::io::{Read, BufRead, BufReader};
use std::io::ErrorKind;
use std::fs::File;
use crate::keyword::*;
use crate::symbol::*;
use crate::char_class::*;
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier(String),
    Number(i32),
}

pub struct Tokenizer {
    reader: BufReader<File>,
    current_byte: u8,
}

#[derive(Debug)]
pub enum TokenizerError {
    ReachedEOF,
    UndefinedToken,
    CannotReadByte,
    CommentNotTerminated,
    Unrecoverable,
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenizerError::ReachedEOF => {
                write!(f, "Error: Reached EOF")
            },
            TokenizerError::UndefinedToken => {
                write!(f, "Error: Undefined token found")
            },
            TokenizerError::CannotReadByte => {
                write!(f, "Error: Cannot read byte")
            },
            TokenizerError::CommentNotTerminated => {
                write!(f, "Comment Not Terminated")
            },
            TokenizerError::Unrecoverable => {
                write!(f, "Unexpected error occurred")
            }
        }
    }
}

impl Tokenizer {
    pub fn new(f: File) -> Self {
        let mut reader = BufReader::new(f);
        let mut byte = [0; 1];
        reader.read_exact(&mut byte);
        println!("{:?}", byte);
        Tokenizer {
            reader: reader,
            current_byte: byte[0],
        }
    }

    pub fn get_next_token(&mut self) -> Result<Token, TokenizerError> {
        while self.current_byte.is_ascii_whitespace() || self.current_byte == b'\n' {
            self._read_next_byte()?;
        }
        match CharClass::from_u8(self.current_byte) {
            CharClass::Digit => {
                self._tokenize_number()
            },
            CharClass::Letter => {
                self._tokenize_identifier()
            },
            CharClass::Colon => {
                self._read_next_byte()?;
                match CharClass::from_u8(self.current_byte) {
                    CharClass::Equal => {
                        self._read_next_byte();
                        Ok(Token::Symbol(Symbol::Assign))
                    },
                    _ => {
                        Err(TokenizerError::UndefinedToken)
                    }
                }
            },
            CharClass::Lss => {
                self._read_next_byte();
                match CharClass::from_u8(self.current_byte) {
                    CharClass::Equal => {
                        self._read_next_byte();
                        Ok(Token::Symbol(Symbol::LssEq))
                    },
                    CharClass::Gtr => {
                        self._read_next_byte();
                        Ok(Token::Symbol(Symbol::NotEq))
                    },
                    _ => {
                        Ok(Token::Symbol(Symbol::Lss))
                    }
                }
            },
            CharClass::Gtr => {
                self._read_next_byte();
                match CharClass::from_u8(self.current_byte) {
                    CharClass::Equal => {
                        self._read_next_byte();
                        Ok(Token::Symbol(Symbol::GtrEq))
                    },
                    _ => {
                        Ok(Token::Symbol(Symbol::Gtr))
                    }
                }
            },
            CharClass::Slash => {
                self._read_next_byte();
                match CharClass::from_u8(self.current_byte) {
                    CharClass::Aster => { /* comment */
                        loop {
                            match self._read_until(b'*') {
                                Ok(()) => {
                                    self._read_next_byte();
                                    if self.current_byte == b'/' {
                                        self._read_next_byte();
                                        break;
                                    }
                                },
                                Err(e) => {
                                    match e.kind() {
                                        ErrorKind::UnexpectedEof => {
                                            return Err(TokenizerError::CommentNotTerminated)
                                        },
                                        _ => {
                                            return Err(TokenizerError::Unrecoverable)
                                        }
                                    }
                                }
                            }
                        }
                        self.get_next_token() // recursion
                    },
                    _ => {
                        Ok(Token::Symbol(Symbol::Div))
                    }
                }
            },
            cc => {
                match Symbol::try_from(cc) {
                    Ok(sym) => {
                        self._read_next_byte();
                        Ok(Token::Symbol(sym))
                    },
                    Err(_) => {
                        Err(TokenizerError::UndefinedToken)
                    }
                }
            }
        }
    }

    fn _read_next_byte(&mut self) -> Result<(), TokenizerError> {
        let mut byte = [0; 1];
        match self.reader.read_exact(&mut byte) {
            Ok(_) => {
                self.current_byte = byte[0];
                Ok(())
            },
            Err(e) => {
                match e.kind() {
                    ErrorKind::UnexpectedEof => {
                        Err(TokenizerError::ReachedEOF)
                    },
                    _ => {
                        Err(TokenizerError::CannotReadByte)
                    }
                }
            }
        }
    }

    fn _read_until(&mut self, b: u8) -> Result<(), std::io::Error>{
        let mut _skip = vec![];
        self.reader.read_until(b, &mut _skip)?;
        self.current_byte = b;
        Ok(())
    }

    fn _tokenize_number(&mut self) -> Result<Token, TokenizerError> {
        let mut digits = vec![self.current_byte];
        loop {
            match self._read_next_byte() {
                Ok(_) => {
                    match self.current_byte {
                        b'0'..=b'9' => {
                            digits.push(self.current_byte);
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Err(e) => {
                    match e {
                        TokenizerError::ReachedEOF => {
                            break;
                        },
                        _ => {
                            return Err(e);
                        }
                    }
                }
            }
            
        }

        let num = digits
            .into_iter()
            .map(|d| (d - b'0') as i32)
            .fold(0, |acc, d| 10*acc + d);

        Ok(Token::Number(num))
    }

    fn _tokenize_identifier(&mut self) -> Result<Token, TokenizerError> {
        let mut chars = vec![self.current_byte];
        loop {
            match self._read_next_byte() {
                Ok(_) => {
                    match CharClass::from_u8(self.current_byte) {
                        CharClass::Digit | CharClass::Letter => {
                            chars.push(self.current_byte);
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Err(e) => {
                    match e {
                        TokenizerError::ReachedEOF => {
                            break;
                        },
                        _ => {
                            return Err(e);
                        }
                    }
                }
            }
        }

        let word = std::str::from_utf8(&chars).unwrap();
        match Keyword::try_from(word) {
            Ok(kw) => {
                Ok(Token::Keyword(kw))
            },
            Err(_) => {
                Ok(Token::Identifier(word.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_export_xml() {
        use super::*;
        use std::path::Path;
        use std::fs::File;
        use std::io::{BufWriter, Write};
        use std::process::Command;

        // pair list of full path of *.pl0 and *T.xml files
        let mut filename_pairs_in_out = vec![]; 
        let dir = Path::new("/workspace/pl0dash-compiler/pl0dash_compiler/pl0");
        for f in dir.read_dir().expect("read_dir call failed") {
            if let Ok(f) = f {
                if f.path().extension().unwrap() == "pl0" {
                    let input_filename = f.path().to_string_lossy().into_owned();
                    let output_filename = dir.join(f.path().file_stem().unwrap()).to_string_lossy().into_owned()+"T.xml";
                    filename_pairs_in_out.push((input_filename, output_filename));
                }
            }
        }
        // tokenize *.pl0, export *T.xml, and compare with *T.xml.org
        for (fin, fout) in filename_pairs_in_out.iter() {
            let input_file = File::open(fin).expect("cannot open input file");
            let mut t = Tokenizer::new(input_file);

            let output_file = File::create(fout).expect("cannot open output file");
            let mut w = BufWriter::<File>::new(output_file);

            // export xml
            writeln!(w, "<tokens>").unwrap();
            'export_xml: loop {
                match t.get_next_token() {
                    Ok(t) => {
                        match t {
                            Token::Keyword(kw) => {
                                writeln!(w, "<keyword> {:?} </keyword>", kw).unwrap();
                            },
                            Token::Symbol(sym) => {
                                writeln!(w, "<symbol> {:?} </symbol>", sym).unwrap();
                            },
                            Token::Identifier(s) => {
                                writeln!(w, "<identifier> {} </identifier>", s).unwrap();
                            },
                            Token::Number(i) => {
                                writeln!(w, "<number> {} </number>", i).unwrap();
                            },
                        }
                    },
                    Err(e) => {
                        match e {
                            TokenizerError::ReachedEOF => {
                                break 'export_xml;
                            },
                            _ => {
                                panic!("{}", e);
                            }
                        }
                    }
                }
            }
            writeln!(w, "</tokens>").unwrap();
            w.flush().unwrap();

            // compare two files
            //let forg = Path::new(fout).with_extension("xml.org").to_string_lossy().into_owned();
            //let diff_status = Command::new("diff").args(["-b", "-u", &fout, &forg]).status().expect("failed to execute process");
            //assert!(diff_status.success());
        }
    }
}
