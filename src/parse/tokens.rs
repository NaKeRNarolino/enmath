pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

impl ___ToTriple for Spanned<Tokens, usize, LexicalError> {
    fn to_triple(self) -> Result<(usize, Tokens, usize), ParseError<usize, Tokens, String>> {
        match self {
            Ok(v) => {
                Ok(v)
            }
            Err(e) => {
                panic!()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct LexicalError;

use std::iter::Peekable;
use std::str::{CharIndices, Chars};
use lalrpop_util::ParseError;
use crate::parse::grammar::___ToTriple;

pub struct Lexer<'input> {
    chars: Peekable<Chars<'input>>,
    pub location: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
            location: 0,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(' ') | Some('\t') = self.chars.peek() {
            self.chars.next();
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tokens, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        let start_pos = self.location;

        let mut chars = self.chars.clone();

        match chars.next() {
            Some('\n') => {
                self.chars.next();
                self.location += 1;
                Some(Ok((start_pos, Tokens::Newline, start_pos + 1)))
            }
            Some(c) if c.is_ascii_alphabetic() => {
                let mut ident = String::new();
                ident.push(c);
                while let Some(c) = chars.peek() {
                    if c.is_ascii_alphanumeric() || *c == '_' {
                        ident.push(*c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                self.chars = chars;
                self.location += ident.len();
                Some(Ok((start_pos, Tokens::Ident(ident.clone()), start_pos + ident.len())))
            }
            Some(c) if c.is_ascii_digit() => {
                let mut num_str = String::new();
                num_str.push(c);
                while let Some(c) = chars.peek() {
                    if c.is_ascii_digit() || *c == '.' {
                        num_str.push(*c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                self.chars = chars;
                self.location += num_str.len();
                match num_str.parse::<f64>() {
                    Ok(num) => Some(Ok((start_pos, Tokens::Number(num), start_pos + num_str.len()))),
                    Err(_) => Some(Err(LexicalError)),
                }
            }
            Some('=') => {
                self.chars.next();
                self.location += 1;
                Some(Ok((start_pos, Tokens::Assign, start_pos + 1)))
            }
            Some('+') => {
                self.chars.next();
                self.location += 1;
                Some(Ok((start_pos, Tokens::Add, start_pos + 1)))
            }
            Some('-') => {
                self.chars.next();
                self.location += 1;
                Some(Ok((start_pos, Tokens::Sub, start_pos + 1)))
            }
            Some('*') => {
                self.chars.next();
                self.location += 1;
                Some(Ok((start_pos, Tokens::Mul, start_pos + 1)))
            }
            Some('/') => {
                self.chars.next();
                self.location += 1;
                Some(Ok((start_pos, Tokens::Div, start_pos + 1)))
            }
            Some('(') => {
                self.chars.next();
                self.location += 1;
                Some(Ok((start_pos, Tokens::LParen, start_pos + 1)))
            }
            Some(')') => {
                self.chars.next();
                self.location += 1;
                Some(Ok((start_pos, Tokens::RParen, start_pos + 1)))
            },
            Some('[') => {
                self.chars.next();
                self.location += 1;
                Some(Ok((start_pos, Tokens::LBracket, start_pos + 1)))
            }
            Some(']') => {
                self.chars.next();
                self.location += 1;
                Some(Ok((start_pos, Tokens::RBracket, start_pos + 1)))
            },
            Some(',') => {
                self.chars.next();
                self.location += 1;
                Some(Ok((start_pos, Tokens::Comma, start_pos + 1)))
            },
            Some('.') => {
                self.chars.next();
                self.location += 1;
                Some(Ok((start_pos, Tokens::Dot, start_pos + 1)))
            },
            Some(_) => {
                self.chars.next();
                self.location += 1;
                Some(Err(LexicalError))
            }
            None => {
                if self.location == start_pos {
                    None
                } else {
                    Some(Ok((start_pos, Tokens::EOF, start_pos)))
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Tokens {
    Ident(String),
    Number(f64),
    Add, Sub, Mul, Div, Assign,
    RParen, LParen,
    RBracket, LBracket,
    Newline,
    Comma, Dot,
    EOF
}