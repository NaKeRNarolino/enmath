use crate::interpret::Interpreter;
use crate::interpret::scope::{Arw, ArwNew, RuntimeScope};
use crate::parse::Parser;

mod parse;
mod interpret;

fn main() {
    let file = include_str!("../examples/basic.emat");
    
    let x = Parser::new(file.to_string()).gen_ast();
    
    let r = RuntimeScope::new();
    
    dbg!(&x);
    
    dbg!(Interpreter::new(
        x.try_program().unwrap()
    ).evaluate(
        Arw::arw(r)
    ));
}
