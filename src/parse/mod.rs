use lalrpop_util::lalrpop_mod;

mod ast;

lalrpop_mod!(pub grammar, "/parser/grammar.rs");

