use crate::{Block, GenericParameter, Parameter, Span, Type};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    StringLiteral(WhirlString),
    NumberLiteral(WhirlNumber),
    CallExpression(Box<CallExpression>),
    FunctionExpression(Box<FunctionExpression>),
    Block(Block),
}

#[derive(Debug, PartialEq)]
pub struct WhirlString {
    pub value: String,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct WhirlNumber {
    pub value: Number,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}

#[derive(PartialEq, Debug, Default)]
pub enum Number {
    Binary(String),
    Octal(String),
    Hexadecimal(String),
    Decimal(String),
    #[default]
    None,
}

#[derive(PartialEq, Debug)]
pub struct CallExpression {
    pub caller: Expression,
    pub arguments: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct FunctionExpression {
    pub generic_params: Option<Vec<GenericParameter>>,
    pub params: Vec<Parameter>,
    pub return_type: Type,
    pub body: Expression,
    pub span: Span,
}

/// Chart for expression precedence in Whirl.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum ExpressionPrecedence {
    Access = 1,           // a.b
    Call = 2,             // a(b)
    New = 3,              // new a
    PowerOf = 4,          // a ^ b
    MultiplyOrDivide = 5, // a * b, a / b
    AddOrSubtract = 6,    // a + b, a - b
    BitLogic = 7,         // a | b, a & b
    Logic = 8,            // a || b, a && b
    Equality = 9,         // a == b, a != b
    TypeUnion = 10,       // A | B
    Pseudo = 99,          // placeholder operator.
}

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Expression::Identifier(i) => i.span,
            Expression::StringLiteral(s) => s.span,
            Expression::NumberLiteral(n) => n.span,
            Expression::CallExpression(c) => c.span,
            Expression::FunctionExpression(f) => f.span,
            Expression::Block(b) => b.span,
        }
    }

    pub(crate) fn set_start(&mut self, start: [u32; 2]) {
        match self {
            Expression::Identifier(i) => i.span.start = start,
            Expression::StringLiteral(s) => s.span.start = start,
            Expression::NumberLiteral(n) => n.span.start = start,
            Expression::CallExpression(c) => c.span.start = start,
            Expression::FunctionExpression(f) => f.span.start = start,
            Expression::Block(b) => b.span.start = start,
        }
    }
}
