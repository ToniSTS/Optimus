use crate::ast::{BinaryOperator, Expression, Literal, Statement};
use crate::lexer::Token;
use chumsky::prelude::*;

pub fn parser() -> impl Parser<Token, Vec<Statement>, Error = Simple<Token>> {
    // 1. Mathematical Expressions (Recursive to handle parentheses like `(5 + 2) * 3`)
    let expr = recursive(|expr| {
        let val = select! {
            Token::Integer(n) => Expression::Literal(Literal::Int(n)),
            Token::Float(s) => Expression::Literal(Literal::Float(s.parse().unwrap_or(0.0))),
            Token::String(s) => Expression::Literal(Literal::Str(s)),
            Token::True => Expression::Literal(Literal::Bool(true)),
            Token::False => Expression::Literal(Literal::Bool(false)),
            Token::Identifier(name) => Expression::Identifier(name),
        };

        // An 'atom' is the smallest piece: a number, variable, or an expression in parentheses
        let atom = val.or(expr.delimited_by(just(Token::LParen), just(Token::RParen)));

        // Layer 1: High Priority (*, /)
        let op_mul_div = just(Token::Asterisk)
            .to(BinaryOperator::Multiply)
            .or(just(Token::Slash).to(BinaryOperator::Divide));

        let product =
            atom.clone()
                .then(op_mul_div.then(atom).repeated())
                .foldl(|left, (op, right)| Expression::BinaryOp {
                    left: Box::new(left),
                    operator: op,
                    right: Box::new(right),
                });

        // Layer 2: Low Priority (+, -)
        let op_add_sub = just(Token::Plus)
            .to(BinaryOperator::Add)
            .or(just(Token::Minus).to(BinaryOperator::Subtract));

        let sum = product
            .clone()
            .then(op_add_sub.then(product).repeated())
            .foldl(|left, (op, right)| Expression::BinaryOp {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            });

        sum // Return the fully layered expression
    });

    // 2. Variable Declaration Parser (Now uses 'expr')
    let var_decl = just(Token::Mut)
        .or_not()
        .then(select! {
            Token::FloatType => "float".to_string(),
            Token::IntType => "int".to_string(),
            Token::StringType => "string".to_string(),
            Token::BoolType => "bool".to_string(),
        })
        .then(select! { Token::Identifier(name) => name })
        .then_ignore(just(Token::Assign))
        .then(expr.clone()) // We changed this from val to expr!
        .then_ignore(just(Token::Semicolon))
        .map(
            |(((opt_mut, v_type), name), value)| Statement::VariableDecl {
                is_mutable: opt_mut.is_some(),
                var_type: v_type,
                name,
                value,
            },
        );

    // 3. Print Statement Parser (Now uses 'expr')
    let print_stmt = just(Token::Print)
        .ignore_then(expr.delimited_by(just(Token::LParen), just(Token::RParen)))
        .then_ignore(just(Token::Semicolon))
        .map(Statement::Print);

    // Combine them and allow them to repeat
    var_decl.or(print_stmt).repeated().then_ignore(end())
}
