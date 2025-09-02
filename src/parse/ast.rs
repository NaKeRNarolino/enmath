#[derive(Debug)]
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
    )
}

impl ASTNode {
    pub fn try_string(&self) -> Result<String, ()> {
        if let Self::Identifier(v) = self {
            Ok(v.to_string())
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
pub enum AssignmentProp {
    Identifier(String),
    Function
}

#[derive(Debug)]
pub enum Operation {
    Add,
    Sub,
    Div,
    Mul,
    Assign
}