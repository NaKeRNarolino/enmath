use std::collections::VecDeque;
use logos::{Lexer, Logos, Span};

#[derive(Debug, Logos, Clone)]
#[logos(
    skip r"[ \t\f]+"
)]
#[derive(PartialEq)]
pub enum Token {
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,
    #[regex(r"[0-9]+(\.[0-9]+)?")]
    Number,
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("=")]
    Assign,
    #[token(")")]
    RParen,
    #[token("(")]
    LParen,
    #[token("[")]
    RBracket,
    #[token("]")]
    LBracket,
    #[token("\n")]
    Newline,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    EOF
}


#[derive(Clone, Debug)]
pub struct TokenData {
    pub t: Token,
    pub span: Span,
    pub slice: String,
}

impl TokenData {
    pub fn vecdeque(mut lexer: Lexer<Token>) -> VecDeque<TokenData> {
        let mut vd = VecDeque::new();

        while let Some(token) = lexer.next() {
            let tk = token.unwrap();

            vd.push_back(
                TokenData {
                    t: tk,
                    span: lexer.span(),
                    slice: lexer.slice().to_string()
                }
            )
        }

        vd
    }
}