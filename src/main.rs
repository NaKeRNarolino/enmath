use crate::interpret::Interpreter;
use crate::interpret::scope::{Arw, ArwNew, RuntimeScope};
use crate::parse::tokens::Lexer;

mod parse;
mod interpret;

fn main() {
    let file = include_str!("../examples/basic.emat");
    
    let x = parse::grammar::ProgramParser::new()
        .parse(
            Lexer::new(file)
        ).unwrap();
    
    let r = RuntimeScope::new();
    
    dbg!(&x);
    
    dbg!(Interpreter::new(
        x.try_program().unwrap()
    ).evaluate(
        Arw::arw(r)
    ));
}
