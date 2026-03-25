#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    // A raw value: 5, 3.14, "Hello", or true
    Literal(Literal),

    // A variable name: my_var
    Identifier(String),

    // A math operation: 5 + 10
    // We use Box<Expression> because an expression can contain another expression
    BinaryOp {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },

    // A function call
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
    Add,      // +
    Subtract, // -
    Multiply, // *
    Divide,   // /
    Equal,    // ==
    NotEqual, // !=
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    // Variable declaration: mut int x = 5;
    VariableDecl {
        is_mutable: bool,
        var_type: String, // "int", "float", etc.
        name: String,
        value: Expression,
    },
    // Print
    Print(Expression),

    // A standalone expression: print(x);
    Expression(Expression),

    // A block of code: { ... }
    Block(Vec<Statement>),
}
