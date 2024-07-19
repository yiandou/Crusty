pub enum Expression {
    Identifier(String),
    Literal(Literal),
    Unary(UnaryExpression),
    Multiplicative(MultiplicativeExpression),
    Additive(AdditiveExpression),
    Relational(RelationalExpression),
    Equality(EqualityExpression),
    Logical(LogicalExpression),
}

pub enum Literal {
    Numeric(String),
    String(String),
    Boolean(bool),
}

pub struct UnaryExpression {
    operator: String,
    expression: Box<Expression>,
}

pub struct MultiplicativeExpression {
    left: Box<Expression>,
    operator: String,
    right: Box<Expression>,
}

pub struct AdditiveExpression {
    left: Box<Expression>,
    operator: String,
    right: Box<Expression>,
}

pub struct RelationalExpression {
    left: Box<Expression>,
    operator: String,
    right: Box<Expression>,
}

pub struct EqualityExpression {
    left: Box<Expression>,
    operator: String,
    right: Box<Expression>,
}

pub struct LogicalExpression {
    left: Box<Expression>,
    operator: String,
    right: Box<Expression>,
}