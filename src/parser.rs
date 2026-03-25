use crate::ast::{BinaryOperator, Expression, Literal, Statement};
use crate::lexer::Token;
use chumsky::prelude::*;

pub fn parser() -> impl Parser<Token, Vec<Statement>, Error = Simple<Token>> {
    // Parse mathematical expressions and values
    let expr = recursive(|expr| {
        let val = select! {
            Token::Integer(n) => Expression::Literal(Literal::Int(n)),
            Token::Float(s) => Expression::Literal(Literal::Float(s.parse().unwrap_or(0.0))),
            Token::String(s) => Expression::Literal(Literal::Str(s)),
            Token::True => Expression::Literal(Literal::Bool(true)),
            Token::False => Expression::Literal(Literal::Bool(false)),
            Token::Identifier(name) => Expression::Identifier(name),
        };

        let atom = val.or(expr.delimited_by(just(Token::LParen), just(Token::RParen)));

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

        let op_add_sub = just(Token::Plus)
            .to(BinaryOperator::Add)
            .or(just(Token::Minus).to(BinaryOperator::Subtract));

        // Comparison operators for loop conditions
        let op_compare = just(Token::Less)
            .to(BinaryOperator::Less)
            .or(just(Token::Greater).to(BinaryOperator::Greater))
            .or(just(Token::Equals).to(BinaryOperator::Equal));

        let sum = product
            .clone()
            .then(op_add_sub.then(product).repeated())
            .foldl(|left, (op, right)| Expression::BinaryOp {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            });

        sum.clone()
            .then(op_compare.then(sum).repeated())
            .foldl(|left, (op, right)| Expression::BinaryOp {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            })
    });

    // Parse statements
    let stmt = recursive(|stmt| {
        // Base declaration (no semicolon)
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
            .then(expr.clone())
            .map(
                |(((opt_mut, v_type), name), value)| Statement::VariableDecl {
                    is_mutable: opt_mut.is_some(),
                    var_type: v_type,
                    name,
                    value,
                },
            );

        // Base assignment (no semicolon)
        let assignment = select! { Token::Identifier(name) => name }
            .then_ignore(just(Token::Assign))
            .then(expr.clone())
            .map(|(name, value)| Statement::Assignment { name, value });

        // Standalone statements require semicolons
        let var_decl_stmt = var_decl.clone().then_ignore(just(Token::Semicolon));
        let assignment_stmt = assignment.clone().then_ignore(just(Token::Semicolon));

        let print_stmt = just(Token::Print)
            .ignore_then(
                expr.clone()
                    .delimited_by(just(Token::LParen), just(Token::RParen)),
            )
            .then_ignore(just(Token::Semicolon))
            .map(Statement::Print);

        let block = stmt
            .clone()
            .repeated()
            .delimited_by(just(Token::LBrace), just(Token::RBrace))
            .map(Statement::Block);

        // Java-style For Loop
        let for_loop = just(Token::For)
            .ignore_then(just(Token::LParen))
            .ignore_then(var_decl_stmt.clone().or(assignment_stmt.clone())) // Semicolon included
            .then(expr.clone().then_ignore(just(Token::Semicolon))) // Semicolon required
            .then(assignment.clone()) // No semicolon
            .then_ignore(just(Token::RParen))
            .then(block.clone())
            .map(
                |(((init, condition), increment), body)| Statement::ForLoop {
                    init: Box::new(init),
                    condition,
                    increment: Box::new(increment),
                    body: Box::new(body),
                },
            );

        var_decl_stmt
            .or(assignment_stmt)
            .or(print_stmt)
            .or(for_loop)
            .or(block)
    });

    stmt.repeated().then_ignore(end())
}
