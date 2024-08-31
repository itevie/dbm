use super::lexer::{Location, LogicalOperator};

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Block(Block),
    Number(Number),
    StringNode(StringNode),
    Call(Call),
    Member(Member),
    Logical(Logical),
    IfBlock(IfBlock),
    VariableDeclaration(VariableDeclaration),
}

impl Expression {
    pub fn get_location(&self) -> Location {
        match self.clone() {
            Expression::Identifier(v) => v.location,
            Expression::Number(v) => v.location,
            Expression::Block(v) => v.location,
            Expression::Call(v) => v.location,
            Expression::Logical(v) => v.location,
            Expression::Member(v) => v.location,
            Expression::StringNode(v) => v.location,
            Expression::IfBlock(v) => v.location,
            Expression::VariableDeclaration(v) => v.location,
        }
    }
}

// ----- Specials -----
#[derive(Debug, Clone)]
pub struct Block {
    pub nodes: Vec<Expression>,
    pub location: Location,
}

// ----- Statements -----
#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub name: Identifier,
    pub value: Box<Expression>,
    pub location: Location,
}

// ----- Expressions -----
#[derive(Debug, Clone)]
pub struct Call {
    pub callee: Box<Expression>,
    pub args: Vec<Expression>,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub struct Member {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub struct Logical {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: LogicalOperator,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub struct IfBlock {
    pub test: Box<Expression>,
    pub success: Block,
    pub alternate: Option<Box<Expression>>,
    pub location: Location,
}

// ----- Literals -----

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub struct Number {
    pub value: f64,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub struct StringNode {
    pub value: String,
    pub location: Location,
}
