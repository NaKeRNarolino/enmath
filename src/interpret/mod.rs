use std::io::ErrorKind::Interrupted;
use crate::interpret::scope::{Arw, ArwNew, Function};
use crate::interpret::value::RuntimeValue;
use crate::parse::ast::{ASTNode, AssignmentProp, Operation};

pub mod scope;
mod value;

pub struct Interpreter {
    program: Vec<ASTNode>
}

type RuntimeScope = Arw<scope::RuntimeScope>;

impl Interpreter {
    pub fn new(program: Vec<ASTNode>) -> Self {
        Self {
            program
        }
    }

    pub fn evaluate(&self, scope: RuntimeScope) -> RuntimeValue {
        let program = self.program.clone();
        let mut last = RuntimeValue::Null;

        for node in program {
            last = self.eval(node, scope.clone())
        }

        last
    }

    pub fn eval(&self, node: ASTNode, scope: RuntimeScope) -> RuntimeValue {
        match node {
            ASTNode::BinaryExpression { left, right, operation } => {
                self.eval_binary_expression(left, right, operation, scope)
            }
            ASTNode::NumericLiteral(num) => { RuntimeValue::Number(num) }
            ASTNode::Identifier(ident) => {
                self.eval_identifier(ident, scope)
            }
            ASTNode::AssignmentProp(assignment_prop) => {
                RuntimeValue::AssignmentProp(assignment_prop)
            }
            ASTNode::Program(program) => {
                Self::new(program).evaluate(scope)
            },
            ASTNode::FunctionCall { name, args } => { 
                self.eval_fn_call(name, args, scope)
            },
            ASTNode::FunctionDefinition { name, args, body } => {
                scope.write().unwrap().set_variable_or_const(
                    &name, RuntimeValue::Function(
                        Function {
                            name: name.clone(),
                            args,
                            body
                        }
                    ), true
                ).unwrap();
                RuntimeValue::Null
            }
        }
    }

    pub fn eval_identifier(&self, ident: String, scope: RuntimeScope) -> RuntimeValue {
        scope.read().unwrap().get_var_or_const(&ident).unwrap().value
    }

    fn eval_binary_expression(&self, left: Box<ASTNode>, right: Box<ASTNode>, operation: Operation, scope: RuntimeScope) -> RuntimeValue {
        let l = self.eval((*left).clone(), scope.clone());
        let r = self.eval((*right).clone(), scope.clone());
        
        match operation {
            Operation::Add => {
                l + r
            }
            Operation::Sub => {
                l - r
            }
            Operation::Div => {
                l / r
            }
            Operation::Mul => {
                l * r
            }
            Operation::Assign => {
                if let RuntimeValue::AssignmentProp(prop) = l {
                    self.eval_assignment(prop, r, (*right).clone(), scope.clone())
                }
                
                RuntimeValue::Null
            }
        }
    }
    
    fn eval_assignment(&self, prop: AssignmentProp, value: RuntimeValue, raw_value: ASTNode, scope: RuntimeScope) {
        match prop {
            AssignmentProp::Identifier(ident) => {
                scope.write().unwrap().set_variable_or_const(
                    &ident, value, false
                ).unwrap()
            }
            _ => {}
        }
    }
    
    fn eval_fn_call(&self, name: String, args: Vec<ASTNode>, scope: RuntimeScope) -> RuntimeValue {
        match scope.read().unwrap().get_variable(&name).unwrap().value {
            RuntimeValue::Function(fun) => {
                let program = vec![(*fun.body).clone()];

                let mut nscope = scope::RuntimeScope::new_parent(scope.clone());


                for i in 0..fun.args.len() {
                    let name = fun.args[i].clone();
                    let arg = self.eval(args[i].clone(), scope.clone());
                    
                    nscope.set_variable_or_const(
                        &name,
                        arg,
                        true
                    ).unwrap()
                }
                
                let ev = Interpreter::new(program).evaluate(Arw::arw(nscope));
                
                ev
            }
            _ => {
                panic!("Not a function.")
            }
        }
    }
}