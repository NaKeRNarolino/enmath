use lalrpop_util::lalrpop_mod;

pub mod ast;
pub mod tokens;

lalrpop_mod!(pub grammar, "/parse/grammar.rs");

