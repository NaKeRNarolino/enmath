use std::collections::HashMap;
use std::os::unix::raw::off_t;
use std::sync::{Arc, RwLock};
use crate::interpret::value::RuntimeValue;
use crate::parse::ast::ASTNode;

pub type Arw<T> = Arc<RwLock<T>>;

pub trait ArwNew<T> {
    fn arw(value: T) -> Arw<T>;
}

impl<T> ArwNew<T> for Arw<T> {
    fn arw(value: T) -> Arw<T> {
        Arc::new(RwLock::new(value))
    }
}

#[derive(Clone)]
pub struct Variable {
    pub value: RuntimeValue,
    pub is_const: bool
}

pub type Variables = HashMap<String, Variable>;

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub body: Box<ASTNode>,
    pub args: Vec<String>
}

pub struct RuntimeScope {
    parent: Option<Arw<RuntimeScope>>,
    variables: Variables
}

#[derive(Debug)]
pub enum VariableError {
    DoesNotExist(String),
    CannotAssignConstants(String),
    VariableShadowingNotSupported(String)
}

impl RuntimeScope {
    pub fn new() -> Self {
        Self::general_new(None)
    }

    pub fn general_new(parent: Option<Arw<RuntimeScope>>) -> Self {
        Self {
            parent,
            variables: HashMap::new(),
        }
    }

    pub fn new_parent(parent: Arw<RuntimeScope>) -> Self {
        Self::general_new(Some(parent))
    }

    pub fn variables(&self) -> Variables {
        self.variables.clone()
    }

    pub fn get_variable(&self, key: &String) -> Option<Variable> {
        self.resolve_variable(key)
    }

    pub fn set_variable_or_const(&mut self, key: &String, value: RuntimeValue, is_const_declaration: bool) -> Result<(), VariableError> {
        if self.exists(key) {
            self.variables.insert(key.clone(), Variable {
                value, is_const: is_const_declaration
            });

            Ok(())
        } else {
            if self.is_constant(key) {
                Err(VariableError::CannotAssignConstants(key.clone()))
            } else {
                self.variables.insert(key.clone(), Variable {
                    value, is_const: false
                });

                Ok(())
            }
        }
    }

    pub fn get_var_or_const(&self, key: &String) -> Result<Variable, VariableError> {
        match self.get_variable(key) {
            None => Err(VariableError::DoesNotExist(key.clone())),
            Some(v) => Ok(v)
        }
    }

    pub fn exists(&self, key: &String) -> bool {
        if self.variables.contains_key(key) {
            true
        } else {
            if let Some(v) = &self.parent {
                v.read().unwrap().exists(key)
            } else {
                false
            }
        }
    }

    pub fn is_constant(&self, key: &String) -> bool {
        match self.get_variable(key) {
            None => false,
            Some(v) => v.is_const
        }
    }

    fn resolve_variable(&self, key: &String) -> Option<Variable> {
        if self.variables.contains_key(key) {
            self.variables.get(key).cloned()
        } else {
            if let Some(v) = &self.parent {
                v.read().unwrap().resolve_variable(key)
            } else {
                None
            }
        }
    }
}