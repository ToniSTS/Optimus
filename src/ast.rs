#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    BinaryOp {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Call {
        function: String,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    Less,
    Greater,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VariableDecl {
        is_mutable: bool,
        var_type: String,
        name: String,
        value: Expression,
    },
    Assignment {
        name: String,
        value: Expression,
    },
    Print(Expression),
    Expression(Expression),
    Block(Vec<Statement>),

    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },

    WhileLoop {
        condition: Expression,
        body: Box<Statement>,
    },


    ForLoop {
        init: Box<Statement>,
        condition: Expression,
        increment: Box<Statement>,
        body: Box<Statement>,
    },
}
