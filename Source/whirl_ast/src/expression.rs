use crate::{Block, GenericParameter, Parameter, Span, Type};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    StringLiteral(WhirlString),
    NumberLiteral(WhirlNumber),
    BooleanLiteral(WhirlBoolean),
    NewExpr(Box<NewExpr>),
    ThisExpr(ThisExpr),
    CallExpr(Box<CallExpr>),
    FnExpr(Box<FunctionExpr>),
    IfExpr(Box<IfExpression>),
    ArrayExpr(ArrayExpr),
    AccessExpr(Box<AccessExpr>),
    IndexExpr(Box<IndexExpr>),
    BinaryExpr(Box<BinaryExpr>),
    AssignmentExpr(Box<AssignmentExpr>),
    UnaryExpr(Box<UnaryExpr>),
    LogicExpr(Box<LogicExpr>),
    BlockExpr(Block),
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
pub struct WhirlBoolean {
    pub value: bool,
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
pub struct NewExpr {
    pub value: Expression,
    pub span: Span,
}

#[derive(PartialEq, Debug)]
pub struct CallExpr {
    pub caller: Expression,
    pub arguments: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct FunctionExpr {
    pub generic_params: Option<Vec<GenericParameter>>,
    pub params: Vec<Parameter>,
    pub return_type: Type,
    pub body: Expression,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct IfExpression {
    pub condition: Expression,
    pub consequent: Block,
    pub alternate: Option<Else>,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct Else {
    pub expression: Expression,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct ArrayExpr {
    pub elements: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct AccessExpr {
    pub object: Expression,
    pub property: Expression,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct IndexExpr {
    pub object: Expression,
    pub index: Expression,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpr {
    pub left: Expression,
    pub operator: BinOperator,
    pub right: Expression,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct AssignmentExpr {
    pub left: Expression,
    pub operator: AssignOperator,
    pub right: Expression,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub operand: Expression,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct LogicExpr {
    pub left: Expression,
    pub operator: LogicOperator,
    pub right: Expression,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct ThisExpr {
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinOperator {
    Multiply,            // a * b
    Divide,              // a / b
    PowerOf,             // a ^ b
    BitAnd,              // a & b
    BitOr,               // a | b
    Is,                  // a is b
    Equals,              // a == b
    NotEquals,           // a != b
    Remainder,           // a % b
    Add,                 // a + b
    Subtract,            // a - b
    Range,               // a..b
    LessThan,            // a < b
    GreaterThan,         // a > b
    LessThanOrEquals,    // a <= b
    GreaterThanOrEquals, // a >= b
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Negation,        // !a
    NegationLiteral, // not a
    Plus,            // +a
    Minus,           // -a
}

#[derive(Debug, PartialEq)]
pub enum LogicOperator {
    And,        // a && b
    AndLiteral, // a and b
    Or,         // a || b
    OrLiteral,  // a or b
}

#[derive(Debug, PartialEq)]
pub enum AssignOperator {
    Assign,      // a = b
    PlusAssign,  // a += b
    MinusAssign, // a -= b
}

/// Chart for expression precedence in Whirl.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum ExpressionPrecedence {
    Access = 1,                    // a.b
    Index = 2,                     // a[b]
    Call = 3,                      // a(b)
    New = 4,                       // new a
    Negation = 5,                  // !a, not a
    UnaryPlusOrMinus = 6,          // +a, -a
    Range = 7,                     // a..b
    PowerOf = 8,                   // a ^ b
    MultiplyDivideOrRemainder = 9, // a * b, a / b, a % b
    AddOrSubtract = 10,            // a + b, a - b
    BitShift = 11,                 // a << b, a >> b
    Ordering = 12,                 // a > b, a < b, a >= b, a <= b
    Equality = 13,                 // a == b, a != b,
    ReferentialEquality = 14,      // a is b
    BitLogic = 15,                 // a | b, a & b
    Logic = 16,                    // a || b, a && b, a and b, a or b
    Assignment = 17,               // a = b, a += b, a -= b,
    TypeUnion = 18,                // A | B
    Pseudo = 99,                   // placeholder operator.
}

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Expression::Identifier(i) => i.span,
            Expression::StringLiteral(s) => s.span,
            Expression::NumberLiteral(n) => n.span,
            Expression::CallExpr(c) => c.span,
            Expression::FnExpr(f) => f.span,
            Expression::BlockExpr(b) => b.span,
            Expression::IfExpr(i) => i.span,
            Expression::ArrayExpr(a) => a.span,
            Expression::IndexExpr(i) => i.span,
            Expression::BinaryExpr(b) => b.span,
            Expression::AssignmentExpr(a) => a.span,
            Expression::UnaryExpr(u) => u.span,
            Expression::LogicExpr(l) => l.span,
            Expression::AccessExpr(a) => a.span,
            Expression::BooleanLiteral(b) => b.span,
            Expression::ThisExpr(t) => t.span,
            Expression::NewExpr(n) => n.span,
        }
    }

    pub(crate) fn set_start(&mut self, start: [u32; 2]) {
        match self {
            Expression::Identifier(i) => i.span.start = start,
            Expression::StringLiteral(s) => s.span.start = start,
            Expression::NumberLiteral(n) => n.span.start = start,
            Expression::CallExpr(c) => c.span.start = start,
            Expression::FnExpr(f) => f.span.start = start,
            Expression::BlockExpr(b) => b.span.start = start,
            Expression::IfExpr(i) => i.span.start = start,
            Expression::ArrayExpr(a) => a.span.start = start,
            Expression::IndexExpr(i) => i.span.start = start,
            Expression::BinaryExpr(b) => b.span.start = start,
            Expression::AssignmentExpr(a) => a.span.start = start,
            Expression::UnaryExpr(u) => u.span.start = start,
            Expression::LogicExpr(l) => l.span.start = start,
            Expression::AccessExpr(a) => a.span.start = start,
            Expression::BooleanLiteral(b) => b.span.start = start,
            Expression::ThisExpr(t) => t.span.start = start,
            Expression::NewExpr(n) => n.span.start = start,
        }
    }
}
