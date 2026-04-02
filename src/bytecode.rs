use crate::ast::{BinaryOperator, Expression, Literal, Statement};
use crate::stdlib;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Instruction {
    Push(Literal),
    LoadVar(String),
    StoreVar(String),
    Add,
    Subtract,
    Multiply,
    Divide,
    Less,
    Greater,
    Equal,
    NotEqual,
    Jump(usize),
    JumpIfFalse(usize),
    CallStd { name: String, argc: usize },
    Print,
    Halt,
}

pub struct BytecodeCompiler {
    instructions: Vec<Instruction>,
}

impl BytecodeCompiler {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn compile_program(mut self, ast: &[Statement]) -> Vec<Instruction> {
        for stmt in ast {
            self.compile_statement(stmt);
        }
        self.instructions.push(Instruction::Halt);
        self.instructions
    }

    fn compile_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::VariableDecl { name, value, .. } | Statement::Assignment { name, value } => {
                self.compile_expression(value);
                self.instructions.push(Instruction::StoreVar(name.clone()));
            }
            Statement::Print(expr) => {
                self.compile_expression(expr);
                self.instructions.push(Instruction::Print);
            }
            Statement::Expression(expr) => {
                self.compile_expression(expr);
            }
            Statement::Block(stmts) => {
                for stmt in stmts {
                    self.compile_statement(stmt);
                }
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.compile_expression(condition);
                let jump_if_false_pos = self.instructions.len();
                self.instructions.push(Instruction::JumpIfFalse(usize::MAX));
                self.compile_statement(then_branch);

                if let Some(else_branch) = else_branch {
                    let jump_over_else_pos = self.instructions.len();
                    self.instructions.push(Instruction::Jump(usize::MAX));
                    let else_start = self.instructions.len();
                    self.patch_jump_if_false(jump_if_false_pos, else_start);
                    self.compile_statement(else_branch);
                    let after_else = self.instructions.len();
                    self.patch_jump(jump_over_else_pos, after_else);
                } else {
                    let after_then = self.instructions.len();
                    self.patch_jump_if_false(jump_if_false_pos, after_then);
                }
            }
            Statement::WhileLoop { condition, body } => {
                let loop_start = self.instructions.len();
                self.compile_expression(condition);
                let jump_if_false_pos = self.instructions.len();
                self.instructions.push(Instruction::JumpIfFalse(usize::MAX));
                self.compile_statement(body);
                self.instructions.push(Instruction::Jump(loop_start));
                let loop_end = self.instructions.len();
                self.patch_jump_if_false(jump_if_false_pos, loop_end);
            }
            Statement::ForLoop {
                init,
                condition,
                increment,
                body,
            } => {
                self.compile_statement(init);
                let loop_start = self.instructions.len();
                self.compile_expression(condition);
                let jump_if_false_pos = self.instructions.len();
                self.instructions.push(Instruction::JumpIfFalse(usize::MAX));
                self.compile_statement(body);
                self.compile_statement(increment);
                self.instructions.push(Instruction::Jump(loop_start));
                let loop_end = self.instructions.len();
                self.patch_jump_if_false(jump_if_false_pos, loop_end);
            }
        }
    }

    fn compile_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Literal(literal) => self.instructions.push(Instruction::Push(literal.clone())),
            Expression::Identifier(name) => self.instructions.push(Instruction::LoadVar(name.clone())),
            Expression::BinaryOp {
                left,
                operator,
                right,
            } => {
                self.compile_expression(left);
                self.compile_expression(right);
                self.instructions.push(match operator {
                    BinaryOperator::Add => Instruction::Add,
                    BinaryOperator::Subtract => Instruction::Subtract,
                    BinaryOperator::Multiply => Instruction::Multiply,
                    BinaryOperator::Divide => Instruction::Divide,
                    BinaryOperator::Less => Instruction::Less,
                    BinaryOperator::Greater => Instruction::Greater,
                    BinaryOperator::Equal => Instruction::Equal,
                    BinaryOperator::NotEqual => Instruction::NotEqual,
                });
            }
            Expression::Call {
                function,
                arguments,
            } => {
                for arg in arguments {
                    self.compile_expression(arg);
                }
                self.instructions.push(Instruction::CallStd {
                    name: function.clone(),
                    argc: arguments.len(),
                });
            }
        }
    }

    fn patch_jump_if_false(&mut self, index: usize, target: usize) {
        if let Some(Instruction::JumpIfFalse(slot)) = self.instructions.get_mut(index) {
            *slot = target;
        }
    }

    fn patch_jump(&mut self, index: usize, target: usize) {
        if let Some(Instruction::Jump(slot)) = self.instructions.get_mut(index) {
            *slot = target;
        }
    }
}

pub struct VirtualMachine {
    stack: Vec<Literal>,
    variables: HashMap<String, Literal>,
    ip: usize,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            variables: HashMap::new(),
            ip: 0,
        }
    }

    pub fn execute(&mut self, instructions: &[Instruction]) -> Result<(), String> {
        println!("\n--- Executing Bytecode VM ---");
        self.ip = 0;

        while self.ip < instructions.len() {
            match &instructions[self.ip] {
                Instruction::Push(value) => self.stack.push(value.clone()),
                Instruction::LoadVar(name) => {
                    let value = self
                        .variables
                        .get(name)
                        .cloned()
                        .ok_or_else(|| format!("Undefined variable '{}'", name))?;
                    self.stack.push(value);
                }
                Instruction::StoreVar(name) => {
                    let value = self.pop()?;
                    self.variables.insert(name.clone(), value);
                }
                Instruction::Add => self.binary_math(|l, r| math_add(&l, &r))?,
                Instruction::Subtract => self.binary_math(|l, r| math_subtract(&l, &r))?,
                Instruction::Multiply => self.binary_math(|l, r| math_multiply(&l, &r))?,
                Instruction::Divide => self.binary_math(|l, r| math_divide(&l, &r))?,
                Instruction::Less => self.binary_math(|l, r| compare_less(&l, &r))?,
                Instruction::Greater => self.binary_math(|l, r| compare_greater(&l, &r))?,
                Instruction::Equal => self.binary_math(|l, r| Ok(Literal::Bool(l == r)))?,
                Instruction::NotEqual => self.binary_math(|l, r| Ok(Literal::Bool(l != r)))?,
                Instruction::Jump(target) => {
                    self.ip = *target;
                    continue;
                }
                Instruction::JumpIfFalse(target) => {
                    let condition = self.pop()?;
                    match condition {
                        Literal::Bool(false) => {
                            self.ip = *target;
                            continue;
                        }
                        Literal::Bool(true) => {}
                        _ => return Err("JumpIfFalse expected a bool".to_string()),
                    }
                }
                Instruction::CallStd { name, argc } => {
                    let mut args = Vec::with_capacity(*argc);
                    for _ in 0..*argc {
                        args.push(self.pop()?);
                    }
                    args.reverse();
                    let result = stdlib::call(name, &args)?;
                    self.stack.push(result);
                }
                Instruction::Print => {
                    let value = self.pop()?;
                    print_literal(&value);
                }
                Instruction::Halt => break,
            }
            self.ip += 1;
        }

        Ok(())
    }

    fn pop(&mut self) -> Result<Literal, String> {
        self.stack
            .pop()
            .ok_or_else(|| "VM stack underflow".to_string())
    }

    fn binary_math<F>(&mut self, f: F) -> Result<(), String>
    where
        F: FnOnce(Literal, Literal) -> Result<Literal, String>,
    {
        let right = self.pop()?;
        let left = self.pop()?;
        let result = f(left, right)?;
        self.stack.push(result);
        Ok(())
    }
}

fn as_f64(value: &Literal) -> Result<f64, String> {
    match value {
        Literal::Int(v) => Ok(*v as f64),
        Literal::Float(v) => Ok(*v),
        _ => Err("Expected numeric value".to_string()),
    }
}

fn math_add(left: &Literal, right: &Literal) -> Result<Literal, String> {
    match (left, right) {
        (Literal::Int(l), Literal::Int(r)) => Ok(Literal::Int(l + r)),
        (Literal::Str(l), Literal::Str(r)) => Ok(Literal::Str(format!("{}{}", l, r))),
        _ => Ok(Literal::Float(as_f64(left)? + as_f64(right)?)),
    }
}

fn math_subtract(left: &Literal, right: &Literal) -> Result<Literal, String> {
    match (left, right) {
        (Literal::Int(l), Literal::Int(r)) => Ok(Literal::Int(l - r)),
        _ => Ok(Literal::Float(as_f64(left)? - as_f64(right)?)),
    }
}

fn math_multiply(left: &Literal, right: &Literal) -> Result<Literal, String> {
    match (left, right) {
        (Literal::Int(l), Literal::Int(r)) => Ok(Literal::Int(l * r)),
        _ => Ok(Literal::Float(as_f64(left)? * as_f64(right)?)),
    }
}

fn math_divide(left: &Literal, right: &Literal) -> Result<Literal, String> {
    match (left, right) {
        (Literal::Int(l), Literal::Int(r)) => Ok(Literal::Int(l / r)),
        _ => Ok(Literal::Float(as_f64(left)? / as_f64(right)?)),
    }
}

fn compare_less(left: &Literal, right: &Literal) -> Result<Literal, String> {
    Ok(Literal::Bool(as_f64(left)? < as_f64(right)?))
}

fn compare_greater(left: &Literal, right: &Literal) -> Result<Literal, String> {
    Ok(Literal::Bool(as_f64(left)? > as_f64(right)?))
}

fn print_literal(value: &Literal) {
    match value {
        Literal::Int(v) => println!("> {}", v),
        Literal::Float(v) => println!("> {}", v),
        Literal::Str(v) => println!("> {}", v),
        Literal::Bool(v) => println!("> {}", v),
    }
}
