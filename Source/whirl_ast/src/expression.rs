use crate::Span;

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
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
    TypeUnion = 10,
}
