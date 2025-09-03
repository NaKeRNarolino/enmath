#[derive(Debug, Clone)]
pub enum ASTNode {
    BinaryExpression {
        left: Box<ASTNode>,
        right: Box<ASTNode>,
        operation: Operation,
    },
    NumericLiteral(f64), // essentially the only literal, so is not made into a subtype
    Identifier(String),
    AssignmentProp(
        AssignmentProp
    ),
    Program(Vec<ASTNode>),
    FunctionCall {
        name: String,
        args: Vec<ASTNode>,
    },
    FunctionDefinition {
        name: String,
        args: Vec<String>,
        body: Box<ASTNode>,
    }
}

impl ASTNode {
    pub fn try_string(&self) -> Option<String> {
        if let Self::Identifier(v) = self {
            Some(v.to_string())
        } else {
            None
        }
    }
    
    pub fn try_program(&self) -> Option<Vec<ASTNode>> {
        if let Self::Program(v) = self {
            Some(v.clone())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum AssignmentProp {
    Identifier(String),
    Function(String, Vec<String>)
}

#[derive(Debug, Clone)]
pub enum Operation {
    Add,
    Sub,
    Div,
    Mul,
    Assign
}