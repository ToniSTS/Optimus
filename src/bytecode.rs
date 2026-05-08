use crate::ast::{BinaryOperator, Expression, Literal, Statement};

#[derive(Debug, Clone)]
pub enum Instruction {
    PushInt(i64),
    PushFloat(f64),
    PushBool(bool),
    PushString(String),
    PushNull,

    LoadVar(String),
    StoreVar(String),

    Add,
    Subtract,
    Multiply,
    Divide,

    Equal,
    NotEqual,
    Less,
    Greater,

    Print,
    Pop,

    BeginLoop,
    EndLoop,

    Call(String, usize),
    Return,
}

pub fn compile_program(statements: &[Statement]) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for stmt in statements {
        compile_statement(stmt, &mut instructions);
    }

    instructions
}

fn compile_statement(stmt: &Statement, instructions: &mut Vec<Instruction>) {
    match stmt {
        Statement::VariableDecl { name, value, .. } => {
            compile_expression(value, instructions);
            instructions.push(Instruction::StoreVar(name.clone()));
        }

        Statement::Assignment { value, .. } => {
            compile_expression(value, instructions);
        }

        Statement::Print(expr) => {
            compile_expression(expr, instructions);
            instructions.push(Instruction::Print);
        }

        Statement::Expression(expr) => {
            compile_expression(expr, instructions);
            instructions.push(Instruction::Pop);
        }

        Statement::Block(stmts) => {
            for stmt in stmts {
                compile_statement(stmt, instructions);
            }
        }

        Statement::WhileLoop { condition, body } => {
            instructions.push(Instruction::BeginLoop);
            compile_expression(condition, instructions);
            compile_statement(body, instructions);
            instructions.push(Instruction::EndLoop);
        }

        Statement::ForLoop {
            init,
            condition,
            increment,
            body,
        } => {
            instructions.push(Instruction::BeginLoop);
            compile_statement(init, instructions);
            compile_expression(condition, instructions);
            compile_statement(body, instructions);
            compile_statement(increment, instructions);
            instructions.push(Instruction::EndLoop);
        }

        Statement::Return(value) => {
            if let Some(expr) = value {
                compile_expression(expr, instructions);
            }
            instructions.push(Instruction::Return);
        }

        Statement::If {
            condition,
            then_branch,
            else_branch,
        } => {
            compile_expression(condition, instructions);
            compile_statement(then_branch, instructions);

            if let Some(else_stmt) = else_branch {
                compile_statement(else_stmt, instructions);
            }
        }

        Statement::FunctionDecl(_) => {}

        Statement::ClassDecl { .. } => {}

        Statement::ModuleDecl { .. } => {}

        Statement::Import(_) => {}
    }
}

fn compile_expression(expr: &Expression, instructions: &mut Vec<Instruction>) {
    match expr {
        Expression::Literal(lit) => match lit {
            Literal::Int(n) => instructions.push(Instruction::PushInt(*n)),
            Literal::Float(n) => instructions.push(Instruction::PushFloat(*n)),
            Literal::Bool(b) => instructions.push(Instruction::PushBool(*b)),
            Literal::Str(s) => instructions.push(Instruction::PushString(s.clone())),
            Literal::Null => instructions.push(Instruction::PushNull),
        },

        Expression::Identifier(name) => {
            instructions.push(Instruction::LoadVar(name.clone()));
        }

        Expression::BinaryOp {
            left,
            operator,
            right,
        } => {
            compile_expression(left, instructions);
            compile_expression(right, instructions);

            match operator {
                BinaryOperator::Add => instructions.push(Instruction::Add),
                BinaryOperator::Subtract => instructions.push(Instruction::Subtract),
                BinaryOperator::Multiply => instructions.push(Instruction::Multiply),
                BinaryOperator::Divide => instructions.push(Instruction::Divide),
                BinaryOperator::Equal => instructions.push(Instruction::Equal),
                BinaryOperator::NotEqual => instructions.push(Instruction::NotEqual),
                BinaryOperator::Less => instructions.push(Instruction::Less),
                BinaryOperator::Greater => instructions.push(Instruction::Greater),
            }
        }

        Expression::Call { callee, arguments } => {
            for arg in arguments {
                compile_expression(arg, instructions);
            }

            match callee.as_ref() {
                Expression::Identifier(name) => {
                    instructions.push(Instruction::Call(name.clone(), arguments.len()));
                }
                _ => {
                    instructions.push(Instruction::Call("<complex-call>".to_string(), arguments.len()));
                }
            }
        }

        Expression::MemberAccess { object, member } => {
            compile_expression(object, instructions);
            instructions.push(Instruction::LoadVar(member.clone()));
        }

        Expression::New {
            class_name,
            arguments,
        } => {
            for arg in arguments {
                compile_expression(arg, instructions);
            }

            instructions.push(Instruction::Call(format!("new {}", class_name), arguments.len()));
        }

        Expression::UnaryOp { expr, .. } => {
            compile_expression(expr, instructions);
        }
    }
}

pub fn print_bytecode(instructions: &[Instruction]) {
    for (index, instruction) in instructions.iter().enumerate() {
        println!("{:04}: {:?}", index, instruction);
    }
}
