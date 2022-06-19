use std::io::{Read, BufRead, BufReader};
use std::io::ErrorKind;
use std::fs::File;
use crate::keyword::*;
use crate::symbol::*;
use crate::char_class::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier(String),
    Number(i32),
    Undefined
}

pub struct Tokenizer {
    reader: BufReader<File>,
    current_byte: u8,
}

pub enum TokenizerError {
    ReachedEOF,
    NoMoreToken,
    UndefinedToken,
    Unrecoverable,
}

impl Tokenizer {
    pub fn new(f: File) -> Self {
        let mut reader = BufReader::new(f);
        let mut byte = [0; 1];
        reader.read_exact(&mut byte);
        Tokenizer {
            reader: reader,
            current_byte: byte,
        }
    }

    pub fn get_next_token(&self) -> Option<Token> {
        b'\n' => {
            self.get_next_token()
        },
        while self.curent_byte.is_ascii_whitespace() {
            self._read_next_byte();
        }
        match CharClass::from_u8(self.current_byte) {
            CharClass::Digit => {
                Some(self._tokenize_number())
            },
            CharClass::Letter => {
                Some(self._tokenize_identifier())
            },
            CharClass::Colon => {
                self._read_next_byte();
                match CharClass::from_u8(self.current_byte) {
                    CharClass::Equal => {
                        self._read_next_byte();
                        Some(Token::Symbol(Symbol::Assign))
                    },
                    _ => {
                        Some(Token::Undefined)
                    }
                }
            },
            CharClass::Lss => {
                self._read_next_byte();
                match CharClass::from_u8(self.current_byte) {
                    CharClass::Equal => {
                        self._read_next_byte();
                        Some(Token::Symbol(Symbol::LssEq))
                    },
                    CharClass::Gtr => {
                        self._read_next_byte();
                        Some(Token::Symbol(Symbol::NotEq))
                    },
                    _ => {
                        Some(Token::Symbol(Symbol::Lss))
                    }
                }
            },
            CharClass::Gtr => {
                self._read_next_byte();
                match CharClass::from_u8(self.current_byte) {
                    CharClass::Equal => {
                        self._read_next_byte();
                        Some(Token::Symbol(Symbol::GtrEq))
                    },
                    _ => {
                        Some(Token::Symbol(Symbol::Gtr))
                    }
                }
            },
            Char::Slash => {
                self._read_next_byte();
                match CharClass::from_u8(self.current_byte) {
                    CharClass::Aster => { /* comment */
                        loop {
                            self._read_until(b'*');
                            self._read_next_byte();
                            if self.current_byte == b'/' {
                                self._read_next_byte();
                                break;
                            }
                        }
                        self.get_next_token()
                    },
                    _ => {
                        Some(Token::Symbol(Symbol::Div))
                    }
                }
            },
            _ => {
                match Symbol::from_u8(self.current_byte) {
                    Ok(sym) => {
                        tokens.push(Token::Symbol(sym));
                    },
                    Err(e) => {
                        panic!("unexpected error occurred while tokenizing: {}", e);
                    }
                }
            },
            Err(e) => {
                match e {
                    ReachedEOF => None, // reached EOF
                    _ => panic!("unrecoverable error occurred while tokenizing."),
                }
            }
        }
    }

    fn _read_next_byte(&self) -> Result<_, TokenizerError> {
        let mut one_byte = [0; 1];
        match reader.read_exact(&mut one_byte) {
            Ok(_) => {
                self.current_byte = one_byte[0];
                Ok()
            },
            Err(ErrorKind::UnexpectedEOF) => {
                Err(ReachedEOF)
            },
            Err(_) => {
                Err(Unrecoverable)
            }
        }
    }

    fn _read_until(&self, b: u8) {
        let mut skip = vec![];
        self._read_until(b, &mut skip).unwrap();
        self.current_byte = skip.last();
    }

    fn _tokenize_number(&self) -> Token {
        let mut digits = vec![self.current_byte];
        loop {
            self._read_next_byte();
            match self.current_byte {
                b'0'..=b'9' => {
                    digits.push(*d);
                },
                _ => {
                    break;
                }
            }
        }

        let num = digits
            .into_iter()
            .map(|d| (d - b'0') as i32)
            .fold(0, |acc, d| 10*acc + d);

        Token::Number(num)
    }

    fn _tokenize_identifier(&self) -> Token {
        let mut chars = vec![self.current_byte];
        loop {
            self._read_next_byte();
            match CharClass::from_u8(self.current_byte) {
                CharClass::Digit | CharClass::Letter => {
                    chars.push(self.current_byte);
                },
                _ => {
                    break;
                }
            }
        }

        let word = std::str::from_utf8(&chars).unwrap();
        match Keyword::try_from(word) {
            Ok(kw) => {
                Token::Keyword(kw)
            },
            Err(_) => {
                Token::Identifier(word.to_string());
            }
        }
    }
/*
    fn _tokenize_symbol(&self) -> Token {

    }

    fn _peek_next_byte(&self) -> Option<u8> {
        match reader.fill_buf() {
            Ok(buf) => {
                Some(buf[0])
            },
            Err(ErrorKind::UnexpectedEOF) => {
                None
            },
            Err(_) => {
                panic!("unrecoverable error occurred while tokenizing.");
            }
        }
    }
    */
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

        // pair list of full path of *.jack and *T.xml files
        let mut filename_pairs_in_out = vec![]; 
        let jack_src_path = Path::new("/workspace/Jack-compiler/jack_compiler/jack");
        for dir in jack_src_path.read_dir().expect("read_dir call failed") {
            if let Ok(dir) = dir {
                for f in dir.path().read_dir().expect("read_dir call failed") {
                    if let Ok(f) = f {
                        if f.path().extension().unwrap() == "jack" {
                            let input_filename = f.path().to_string_lossy().into_owned();
                            let output_filename = dir.path().join(f.path().file_stem().unwrap()).to_string_lossy().into_owned()+"T.xml";
                            filename_pairs_in_out.push((input_filename, output_filename));
                        }
                    }
                }
            }
        }

        // tokenize *.jack, export *T.xml, and compare with *T.xml.org
        for (fin, fout) in filename_pairs_in_out.iter() {
            let input_file = File::open(fin).expect("cannot open input file");
            let mut t = Tokenizer::new(input_file);

            let output_file = File::create(fout).expect("cannot open output file");
            let mut w = BufWriter::<File>::new(output_file);

            // export xml
            writeln!(w, "<tokens>").unwrap();
            'export_xml: loop {
                match t.get_next_token() {
                    Some(t) => {
                        match t {
                            Token::Keyword(kw) => {
                                writeln!(w, "<keyword> {} </keyword>", kw).unwrap();
                            },
                            Token::Symbol(sym) => {
                                writeln!(w, "<symbol> {} </symbol>", sym).unwrap();
                            },
                            Token::Identifier(s) => {
                                writeln!(w, "<identifier> {} </identifier>", s).unwrap();
                            },
                            Token::Number(i) => {
                                writeln!(w, "<number> {} </number>", i).unwrap();
                            },
                            _ => {
                                panic!("undefined symbol");
                            }
                        }
                    },
                    None => { break 'export_xml; }
                }
            }
            writeln!(w, "</tokens>").unwrap();
            w.flush().unwrap();

            // compare two files
            let forg = Path::new(fout).with_extension("xml.org").to_string_lossy().into_owned();
            let diff_status = Command::new("diff").args(["-b", "-u", &fout, &forg]).status().expect("failed to execute process");
            assert!(diff_status.success());
        }
    }
}
