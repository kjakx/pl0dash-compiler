use std::io::{BufWriter, Write};
use std::fs::File;
use crate::tokenizer::*;
use crate::keyword::*;
use crate::symbol::*;
use crate::char_class::*;

#[derive(Copy, Clone, Debug, PartialEq)]
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

    fn get_syntax(&self) -> Syntax {
        self.syntax
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

pub struct SyntaxTree {
    root: SyntaxNode
}

impl SyntaxTree {
    fn new(syntax_node: SyntaxNode) -> Self {
        SyntaxTree {
            syntax_node
        }
    }

    fn get_ref_root(&self) -> &SyntaxNode {
        &root
    }
}

struct SyntaxTreeVisitor<'a> {
    depth: usize,
    current_node: &'a SyntaxNode,
}

impl SyntaxTreeVisitor {
    fn new(node: &'a SyntaxNode) -> Self {
        current_node: node
    }

    fn visit(&mut self) {
        let current_syntax = *current_node.get_syntax();
        self.print_indent();
        println!("<{:?}>", current_syntax);
        self.depth += 1;
        for child in current_node.get_ref_children().iter() {
            self.current_node = *child;
            self.visit();
        }
        self.depth -= 1;
        self.print_indent();
        println!("</{:?}>", current_syntax());
    }

    fn print_indent(&self) {
        for i in 0..self.depth {
            print!(" ");
        }
    }
}

pub struct Parser {
    tokenizer: Tokenizer,
    current_token: Token,
}

impl Parser {
    pub fn new(t: Tokenizer) -> Self {
        Engine {
            tokenizer: t,
            current_token: t.get_next_token(),
        }
    }
    
    pub fn parse(&mut self) -> SyntaxTree {
        SyntaxTree::new(self.parse_program())
    }

    fn parse_program(&mut self) -> SyntaxNode {
        let mut node = SyntaxNode::new(Syntax::Program);
        node.append_child(self.parse_block());
        node.append_child(self.parse_token());
        node
    }

    fn compile_block(&mut self) -> SyntaxNode {
        let mut node = SyntaxNode::new(Syntax::Block);
        loop {
            let child = match self.current_token {
                Token::Keyword(Keyword::Const) => {
                /    self.current_token = self.tokenizer.get_next_token();
                    parse_const_decl()
                },
                Token::Keyword(Keyword::Var) => {
                    self.current_token = self.tokenizer.get_next_token();
                    parse_var_decl()
                },
                Token::Keyword(Keyword::Function) => {
                    self.current_token = self.tokenizer.get_next_token();
                    parse_func_decl()
                },
                _ => {
                    break;
                }
            };
            node.append_child(child);
        }
        node.append_child(parse_statement());
        node
    }
    
    fn parse_const_decl(&mut self) -> SyntaxNode {
        let mut node = SyntaxNode::new(Syntax::ConstDecl);
        node.append_child(SyntaxNode::Token(Token::Keyword(Keyword::Const))); // const
        loop {
            node.append_child(self.parse_token()); // ident
            node.append_child(self.parse_token()); // =
            node.append_child(self.parse_token()); // number
            if let Token::Symbol(Symbol::Comma) = self.current_token {
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
        node.append_child(SyntaxNode::Token(Token::Keyword(Keyword::Var))); // var
        loop {
            node.append_child(self.parse_token()); // ident
            if let Token::Symbol(Symbol::Comma) = self.current_token {
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
        node.append_child(SyntaxNode::Token(Token::Keyword(Keyword::Function))); // function
        node.append_child(self.parse_token()); // ident
        node.append_child(self.parse_token()); // '('
        while let Token::Identifier(_) = self.current_token {
            node.append_child(self.parse_token()); // ident
            if let Token::Symbol(Symbol::Comma) = self.current_token {
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
                    if let Token::Symbol(Symbol::Semicolon) = self.current_token {
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
            Token::Keyword(Keyword::Return) => {
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
        if let Token::Keyword(Keyword::Odd) = self.current_token {
            node.append_child(self.parse_token()); // odd
            node.append_child(self.parse_expression());
        } else {
            node.append_child(self.parse_expression());
            node.append_child(self.parse_token()); // bool op.
            node.append_child(self.parse_expression());
        }
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
            }
        }
    }

    fn parse_factor(&mut self) -> SyntaxNode {
        let mut node = SyntaxNode::new(Syntax::Factor);
        match self.current_token {
            Token::Identifier => {
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
            Token::Number => {
                node.append_child(self.parse_token()); // number
            },
            Token::Symbol => {
                node.append_child(self.parse_token()); // '('
                node.append_child(self.parse_expression());
                node.append_child(self.parse_token()); // ')'
            }
        }
        node
    }

    fn parse_token(&mut self) -> SyntaxNode {
        let mut node = SyntaxNode::new(Syntax::Token(self.current_token.clone()));
        self.current_token = self.tokenizer.get_next_token();
        node
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_no_expression_case() {
        use super::*;
        use std::path::Path;
        use std::fs::File;
        use std::io::{BufWriter, Write};
        use std::process::Command;
        use crate::tokenizer::*;

        // pair list of full path of *.jack and *.xml files
        let mut filename_pairs_in_out = vec![]; 
        let jack_src_path = Path::new("/workspace/Jack-compiler/jack_compiler/jack/ExpressionLessSquare");
        for f in jack_src_path.read_dir().expect("read_dir call failed") {
            if let Ok(f) = f {
                if f.path().extension().unwrap() == "jack" {
                    let input_filename = f.path().to_string_lossy().into_owned();
                    let output_filename = f.path().with_extension("xml").to_string_lossy().into_owned();
                    filename_pairs_in_out.push((input_filename, output_filename));
                }
            }
        }

        // compile *.jack, export *.xml, and compare with *.xml.org
        for (fin, fout) in filename_pairs_in_out.iter() {
            // tokenize
            let input_file = File::open(fin).expect("cannot open input file");
            let mut t = Tokenizer::new(input_file);
            
            // compile
            let output_file = File::create(fout).expect("cannot open output file");
            let mut e = Engine::new(t, output_file);
            e.compile();

            // compare two files
            let forg = Path::new(fout).with_extension("xml.org").to_string_lossy().into_owned();
            let diff_status = Command::new("diff").args(["-b", "-u", "-w", &fout, &forg]).status().expect("failed to execute process");
            assert!(diff_status.success());
        }
    }

    #[test]
    fn test_expression_case() {
        use super::*;
        use std::path::Path;
        use std::fs::File;
        use std::io::{BufWriter, Write};
        use std::process::Command;
        use crate::tokenizer::*;

        // pair list of full path of *.jack and *.xml files
        let mut filename_pairs_in_out = vec![]; 
        let square_path = Path::new("/workspace/Jack-compiler/jack_compiler/jack/Square");
        let array_test_path = Path::new("/workspace/Jack-compiler/jack_compiler/jack/ArrayTest");
        for d in [square_path, array_test_path].into_iter() {
            for f in d.read_dir().expect("read_dir call failed") {
                if let Ok(f) = f {
                    if f.path().extension().unwrap() == "jack" {
                        let input_filename = f.path().to_string_lossy().into_owned();
                        let output_filename = f.path().with_extension("xml").to_string_lossy().into_owned();
                        filename_pairs_in_out.push((input_filename, output_filename));
                    }
                }
            }
        }

        // compile *.jack, export *.xml, and compare with *.xml.org
        for (fin, fout) in filename_pairs_in_out.iter() {
            // tokenize
            let input_file = File::open(fin).expect("cannot open input file");
            let mut t = Tokenizer::new(input_file);
            
            // compile
            let output_file = File::create(fout).expect("cannot open output file");
            let mut e = Engine::new(t, output_file);
            e.compile();

            // compare two files
            let forg = Path::new(fout).with_extension("xml.org").to_string_lossy().into_owned();
            let diff_status = Command::new("diff").args(["-b", "-u", "-w", &fout, &forg]).status().expect("failed to execute process");
            assert!(diff_status.success());
        }
    }
}