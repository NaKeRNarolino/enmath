use std::collections::VecDeque;
use std::iter::Peekable;
use logos::{Lexer, Logos};
use crate::parse::ast::{ASTNode, Operation};
use crate::parse::tokens::{Token, TokenData};

pub mod ast;
pub mod tokens;

pub struct Parser {
    tokens: VecDeque<TokenData>
}

pub trait Boxed<T> {
    fn boxed(&self) -> Box<T>;
}

impl<T: Clone> Boxed<T> for T {
    fn boxed(&self) -> Box<T> {
        Box::new(self.clone())
    }
}

impl Parser {
    pub fn new(src: String) -> Self {
        let tk = TokenData::vecdeque(Token::lexer(&src));

        dbg!(&tk);
        Self { tokens: tk }
    }

    fn eof() -> TokenData { 
        TokenData {
            t: Token::EOF,
            span: 0..0,
            slice: "__EOF__".to_string(),
        }
    }
    
    fn curr(&mut self) -> TokenData {
        self.tokens.front().unwrap_or(&Self::eof()).clone()
    }

    fn go(&mut self) -> TokenData {
        self.tokens.pop_front().unwrap()
    }

    fn peek(&mut self) -> TokenData {
        self.tokens.get(1).unwrap_or(&Self::eof()).clone()
    }

    fn complete(&mut self) -> bool {
        self.tokens.is_empty()
    }

    pub fn gen_ast(&mut self) -> ASTNode {
        let mut nodes: Vec<ASTNode> = vec![];

        while !self.complete() {
            nodes.push(self.parse_expr())
        }

        ASTNode::Program(nodes)
    }

    pub fn expect(&mut self, token: Token, reason: impl Into<String>) -> TokenData {
        let c = self.curr();

        if c.t != token {
            panic!("Expected token {:?}, got {:?}: {}", token, c.t, &reason.into())
        } else {
            c
        }
    }

    fn parse_expr(&mut self) -> ASTNode {
        self.parse_add_expr()
    }

    fn parse_add_expr(&mut self) -> ASTNode {
        let mut left = self.parse_mul_expr();
        let operator = self.curr();

        while operator.t == Token::Add || operator.t == Token::Sub {
            let op = self.curr();

            if op.t != Token::Add && op.t != Token::Sub {
                break;
            }
            self.go();

            let right = self.parse_mul_expr();

            let operand = if operator.t == Token::Add { Operation::Add } else { Operation::Sub };

            left = ASTNode::BinaryExpression {
                left: left.boxed(),
                right: right.boxed(),
                operation: operand
            }
        }

        left
    }

    fn parse_mul_expr(&mut self) -> ASTNode {
        let mut left = self.parse_atom();

        let operator = self.curr();

        while operator.t == Token::Mul || operator.t == Token::Div {
            let op = self.curr();

            if op.t != Token::Mul && op.t != Token::Div {
                break;
            }
            self.go();

            let right = self.parse_atom();

            let operand = if operator.t == Token::Mul { Operation::Mul } else { Operation::Div };

            left = ASTNode::BinaryExpression {
                left: left.boxed(),
                right: right.boxed(),
                operation: operand
            }
        }

        left
    }

    fn parse_function_call_or_decl(&mut self) -> ASTNode {
        dbg!(self.curr());
        if self.curr().t == Token::Ident && self.peek().t == Token::LParen {
            let ident = self.go();
            dbg!(&ident);
            let (args, are_all_ident) = self.parse_function_args();

            if are_all_ident && self.curr().t == Token::Assign {
                self.go();

                let expr = self.parse_expr();

                ASTNode::FunctionDefinition {
                    name: ident.slice.clone(),
                    args: args.iter().map(|x| if let ASTNode::Identifier(t) = x { t.clone() } else { unreachable!() }).collect(),
                    body: expr.boxed()
                }
            } else {
                ASTNode::FunctionCall {
                    name: ident.slice.clone(),
                    args
                }
            }
        } else {
            unreachable!()
        }
    }

    fn parse_function_args(&mut self) -> (Vec<ASTNode>, bool) {
        self.go(); // (

        let mut are_all_ident = true;

        if self.curr().t == Token::RParen {
            self.go();
            return (Vec::new(), true)
        }

        let mut args: Vec<ASTNode> = vec![self.parse_expr()];

        let mut tk = self.go();

        while tk.t == Token::Comma && !self.complete() {
            let expr = self.parse_expr();

            if let ASTNode::Identifier(k) = &expr {} else {
                are_all_ident = false;
            }

            args.push(expr);

            tk = self.go();
        }

        if tk.t != Token::RParen {
            self.expect(
                Token::RParen,
                "Expected a ')'."
            );
        }

        (args, are_all_ident)
    }

    fn parse_atom(&mut self) -> ASTNode {
        let tk = self.curr();

        match tk.t {
            Token::Ident => {
                let ident = ASTNode::Identifier(tk.slice.clone());

                dbg!(self.peek());
                if self.peek().t == Token::LParen {
                    self.parse_function_call_or_decl()
                } else {
                    self.go();
                    ident
                }
            },
            Token::Newline => {
                self.go();
                ASTNode::NumericLiteral(-1.0)
            },
            Token::Number => {
                self.go();
                ASTNode::NumericLiteral(tk.slice.parse::<f64>().unwrap())
            },
            Token::LParen => {
                self.go();
                let e = self.parse_expr();

                self.expect(Token::RParen, "Expected a ')'.");

                e
            },
            _ => panic!("Unexpected token {:?}", tk)
        }
    }
}