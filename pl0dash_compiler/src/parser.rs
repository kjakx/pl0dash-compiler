use std::io::{BufWriter, Write};
use std::fs::File;
use crate::tokenizer::*;
use crate::keyword::*;
use crate::symbol::*;
use crate::char_class::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Syntax {
    Program,
    Block,
    ConstDecl,
    VarDecl,
    FuncDecl,
    Statement,
    Condition,
    Expression,
    Term,
    Factor,
    Token(Token)
}

#[derive(Clone, Debug, PartialEq)]
pub struct SyntaxNode {
    syntax: Syntax,
    children: Vec<SyntaxNode>,
}

impl SyntaxNode {
    fn new(syntax: Syntax) -> Self {
        SyntaxNode {
            syntax,
            children: vec![],
        }
    }

    fn get_ref_syntax(&self) -> &Syntax {
        &self.syntax
    }

    fn append_child(&mut self, child: SyntaxNode) {
        self.children.push(child);
    }

    fn has_child(&self) -> bool {
        !self.children.is_empty()
    }

    fn get_ref_children(&self) -> &Vec<SyntaxNode> {
        &self.children
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SyntaxTree {
    root: SyntaxNode
}

impl SyntaxTree {
    fn new(root: SyntaxNode) -> Self {
        SyntaxTree {
            root
        }
    }

    fn get_ref_root(&self) -> &SyntaxNode {
        &self.root
    }
}

pub struct Parser {
    tokenizer: Tokenizer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut t: Tokenizer) -> Self {
        let token = t.get_next_token().unwrap();
        Parser {
            tokenizer: t,
            current_token: token,
        }
    }
    
    pub fn parse(&mut self) -> SyntaxTree {
        println!("parsing...");
        SyntaxTree::new(self.parse_program())
    }

    fn parse_program(&mut self) -> SyntaxNode {
        let mut node = SyntaxNode::new(Syntax::Program);
        node.append_child(self.parse_block());
        node.append_child(self.parse_token());
        node
    }

    fn parse_block(&mut self) -> SyntaxNode {
        let mut node = SyntaxNode::new(Syntax::Block);
        loop {
            let child = match self.current_token {
                Token::Keyword(Keyword::Const) => {
                    self.current_token = self.tokenizer.get_next_token().unwrap();
                    self.parse_const_decl()
                },
                Token::Keyword(Keyword::Var) => {
                    self.current_token = self.tokenizer.get_next_token().unwrap();
                    self.parse_var_decl()
                },
                Token::Keyword(Keyword::Func) => {
                    self.current_token = self.tokenizer.get_next_token().unwrap();
                    self.parse_func_decl()
                },
                _ => {
                    break;
                }
            };
            node.append_child(child);
        }
        node.append_child(self.parse_statement());
        node
    }
    
    fn parse_const_decl(&mut self) -> SyntaxNode {
        let mut node = SyntaxNode::new(Syntax::ConstDecl);
        node.append_child(SyntaxNode::new(Syntax::Token(Token::Keyword(Keyword::Const)))); // const
        loop {
            node.append_child(self.parse_token()); // ident
            node.append_child(self.parse_token()); // =
            node.append_child(self.parse_token()); // number
            if Token::Symbol(Symbol::Comma) == self.current_token {
                node.append_child(self.parse_token()); // ,
            } else {
                break;
            }
        }
        node.append_child(self.parse_token()); // ;
        node
    }

    fn parse_var_decl(&mut self) -> SyntaxNode {
        let mut node = SyntaxNode::new(Syntax::VarDecl);
        node.append_child(SyntaxNode::new(Syntax::Token(Token::Keyword(Keyword::Var)))); // var
        loop {
            node.append_child(self.parse_token()); // ident
            if Token::Symbol(Symbol::Comma) == self.current_token {
                node.append_child(self.parse_token()); // ,
            } else {
                break;
            }
        }
        node.append_child(self.parse_token()); // ;
        node
    }

    fn parse_func_decl(&mut self) -> SyntaxNode {
        let mut node = SyntaxNode::new(Syntax::FuncDecl);
        node.append_child(SyntaxNode::new(Syntax::Token(Token::Keyword(Keyword::Func)))); // function
        node.append_child(self.parse_token()); // ident
        node.append_child(self.parse_token()); // '('
        while let Token::Identifier(_) = self.current_token {
            node.append_child(self.parse_token()); // ident
            if Token::Symbol(Symbol::Comma) == self.current_token {
                node.append_child(self.parse_token()); // ,
            } else {
                break;
            }
        }
        node.append_child(self.parse_token()); // ')'
        node.append_child(self.parse_block());
        node.append_child(self.parse_token()); // ;
        node
    }

    fn parse_statement(&mut self) -> SyntaxNode {
        let mut node = SyntaxNode::new(Syntax::Statement);
        match self.current_token {
            Token::Identifier(_) => {
                node.append_child(self.parse_token()); // ident
                node.append_child(self.parse_token()); // :=
                node.append_child(self.parse_expression());
            },
            Token::Keyword(Keyword::Begin) => {
                node.append_child(self.parse_token()); // begin
                loop {
                    node.append_child(self.parse_statement());
                    if Token::Symbol(Symbol::SemiColon) == self.current_token {
                        node.append_child(self.parse_token()); // ;
                    } else {
                        break;
                    }
                }
                node.append_child(self.parse_token()); // end
            },
            Token::Keyword(Keyword::If) => {
                node.append_child(self.parse_token()); // if
                node.append_child(self.parse_condition());
                node.append_child(self.parse_token()); // then
                node.append_child(self.parse_statement());
            },
            Token::Keyword(Keyword::While) => {
                node.append_child(self.parse_token()); // while
                node.append_child(self.parse_condition());
                node.append_child(self.parse_token()); // do
                node.append_child(self.parse_statement());
            },
            Token::Keyword(Keyword::Ret) => {
                node.append_child(self.parse_token()); // return
                node.append_child(self.parse_expression());
            },
            Token::Keyword(Keyword::Write) => {
                node.append_child(self.parse_token()); // write
                node.append_child(self.parse_expression());
            },
            Token::Keyword(Keyword::WriteLn) => {
                node.append_child(self.parse_token()); // writeln
            },
            _ => {
                // nothing to do
            }
        }
        node
    }

    fn parse_condition(&mut self) -> SyntaxNode {
        let mut node = SyntaxNode::new(Syntax::Condition);
        if Token::Keyword(Keyword::Odd) == self.current_token {
            node.append_child(self.parse_token()); // odd
            node.append_child(self.parse_expression());
        } else {
            node.append_child(self.parse_expression());
            node.append_child(self.parse_token()); // bool op.
            node.append_child(self.parse_expression());
        }
        node
    }

    fn parse_expression(&mut self) -> SyntaxNode {
        let mut node = SyntaxNode::new(Syntax::Expression);
        if let Token::Symbol(sym) = self.current_token {
            match sym {
                Symbol::Plus | Symbol::Minus => {
                    node.append_child(self.parse_token()); // + or -
                },
                _ => {
                    // nothing to do
                }
            }
        }
        node.append_child(self.parse_term());
        loop {
            if let Token::Symbol(sym) = self.current_token {
                match sym {
                    Symbol::Plus | Symbol::Minus => {
                        node.append_child(self.parse_token()); // + or -
                        node.append_child(self.parse_term());
                    },
                    _ => {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        node
    }

    fn parse_term(&mut self) -> SyntaxNode {
        let mut node = SyntaxNode::new(Syntax::Term);
        node.append_child(self.parse_factor());
        loop {
            if let Token::Symbol(sym) = self.current_token {
                match sym {
                    Symbol::Mult | Symbol::Div => {
                        node.append_child(self.parse_token()); // * or /
                        node.append_child(self.parse_factor());
                    },
                    _ => {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        node
    }

    fn parse_factor(&mut self) -> SyntaxNode {
        let mut node = SyntaxNode::new(Syntax::Factor);
        match self.current_token {
            Token::Identifier(_) => {
                node.append_child(self.parse_token()); // ident
                if Token::Symbol(Symbol::Lparen) == self.current_token {
                    node.append_child(self.parse_token()); // '('
                    if Token::Symbol(Symbol::Rparen) != self.current_token {
                        loop {
                            node.append_child(self.parse_expression());
                            if Token::Symbol(Symbol::Comma) == self.current_token {
                                node.append_child(self.parse_token()); // ,
                            } else {
                                break;
                            }
                        }
                    }
                    node.append_child(self.parse_token()); // ')'
                }
            },
            Token::Number(_) => {
                node.append_child(self.parse_token()); // number
            },
            Token::Symbol(_) => {
                node.append_child(self.parse_token()); // '('
                node.append_child(self.parse_expression());
                node.append_child(self.parse_token()); // ')'
            },
            _ => {
                panic!("syntax error");
            }
        }
        node
    }

    fn parse_token(&mut self) -> SyntaxNode {
        let mut node = SyntaxNode::new(Syntax::Token(self.current_token.clone()));
        self.current_token = self.tokenizer.get_next_token().unwrap();
        node
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
        use super::*;
        use std::path::Path;
        use std::fs::File;
        use std::io::{BufWriter, Write};
        use std::process::Command;
        use crate::tokenizer::*;

        // pair list of full path of *.pl0
        let mut filenames_input = vec![]; 
        let src_path = Path::new("/workspace/pl0dash-compiler/pl0dash_compiler/pl0/");
        for f in src_path.read_dir().expect("read_dir call failed") {
            if let Ok(f) = f {
                if f.path().extension().unwrap() == "pl0" {
                    let input_filename = f.path().to_string_lossy().into_owned();
                    filenames_input.push(input_filename);
                }
            }
        }

        // parse *.pl0 and show the syntax trees
        for fin in filenames_input.iter() {
            // tokenize
            let input_file = File::open(fin).expect("cannot open input file");
            let mut t = Tokenizer::new(input_file);
            
            // parse
            let mut p = Parser::new(t);
            let syn_tree = p.parse();
            println!("parse finished. printing syn_tree...");
            println!("{:?}", syn_tree);

            // compare two files
            //let forg = Path::new(fout).with_extension("xml.org").to_string_lossy().into_owned();
            //let diff_status = Command::new("diff").args(["-b", "-u", "-w", &fout, &forg]).status().expect("failed to execute process");
            //assert!(diff_status.success());
        }
    }
}