use crate::ast::{BinaryOperator, Expression, Literal, Statement};
use std::collections::HashMap;

pub struct Analyzer {
    time_cost: usize,
    space_cost: usize,
    current_depth: usize,
    max_depth: usize,
    memory: HashMap<String, Literal>,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            time_cost: 0,
            space_cost: 0,
            current_depth: 0,
            max_depth: 0,
            memory: HashMap::new(),
        }
    }

    pub fn analyze(&mut self, ast: &[Statement]) {
        println!("\n--- Executing Optimus Script ---");
        for stmt in ast {
            self.execute_statement(stmt);
        }
        self.print_report();
    }

    fn execute_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::VariableDecl { name, value, .. } => {
                self.space_cost += 1;
                self.time_cost += 1;
                let val = self.evaluate_expression(value);
                self.memory.insert(name.clone(), val);
            }
            Statement::Assignment { name, value } => {
                self.time_cost += 1;
                let val = self.evaluate_expression(value);
                self.memory.insert(name.clone(), val);
            }
            Statement::Print(expr) => {
                self.time_cost += 1;
                let val = self.evaluate_expression(expr);
                print!("> ");
                self.print_literal(&val);
            }
            Statement::Block(stmts) => {
                for s in stmts {
                    self.execute_statement(s);
                }
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.time_cost += 1;
                let cond = self.evaluate_expression(condition);
                if let Literal::Bool(true) = cond {
                    self.execute_statement(then_branch);
                } else if let Some(else_b) = else_branch {
                    self.execute_statement(else_b);
                }
            }
            Statement::WhileLoop { condition, body } => {
                self.current_depth += 1;
                if self.current_depth > self.max_depth {
                    self.max_depth = self.current_depth;
                }
                loop {
                    let cond = self.evaluate_expression(condition);
                    if let Literal::Bool(true) = cond {
                        self.execute_statement(body);
                    } else {
                        break;
                    }
                }
                self.current_depth -= 1;
            }
            Statement::ForLoop {
                init,
                condition,
                increment,
                body,
            } => {
                self.current_depth += 1;
                if self.current_depth > self.max_depth {
                    self.max_depth = self.current_depth;
                }
                self.execute_statement(init);
                loop {
                    if let Literal::Bool(true) = self.evaluate_expression(condition) {
                        self.execute_statement(body);
                        self.execute_statement(increment);
                    } else {
                        break;
                    }
                }
                self.current_depth -= 1;
            }
            _ => {}
        }
    }

    fn evaluate_expression(&mut self, expr: &Expression) -> Literal {
        match expr {
            Expression::Literal(l) => l.clone(),
            Expression::Identifier(name) => {
                self.memory.get(name).cloned().unwrap_or(Literal::Int(0))
            }
            Expression::BinaryOp {
                left,
                operator,
                right,
            } => {
                self.time_cost += 1;
                let l = self.evaluate_expression(left);
                let r = self.evaluate_expression(right);
                self.eval_math(operator, &l, &r)
            }
            _ => Literal::Int(0),
        }
    }

    fn eval_math(&self, op: &BinaryOperator, left: &Literal, right: &Literal) -> Literal {
        match (left, right) {
            (Literal::Int(l), Literal::Int(r)) => match op {
                BinaryOperator::Add => Literal::Int(l + r),
                BinaryOperator::Subtract => Literal::Int(l - r),
                BinaryOperator::Multiply => Literal::Int(l * r),
                BinaryOperator::Divide => Literal::Int(l / r),
                BinaryOperator::Less => Literal::Bool(l < r),
                BinaryOperator::Greater => Literal::Bool(l > r),
                BinaryOperator::Equal => Literal::Bool(l == r),
                BinaryOperator::NotEqual => Literal::Bool(l != r),
            },
            _ => Literal::Bool(false),
        }
    }

    fn print_literal(&self, l: &Literal) {
        match l {
            Literal::Int(i) => println!("{}", i),
            Literal::Float(f) => println!("{}", f),
            Literal::Str(s) => println!("{}", s),
            Literal::Bool(b) => println!("{}", b),
        }
    }

    fn print_report(&self) {
        println!("\n========================================");
        println!("BIG-O COMPLEXITY REPORT");
        println!("========================================");
        let time = match self.max_depth {
            0 => "O(1)",
            1 => "O(N)",
            2 => "O(N^2)",
            _ => "O(N^X)",
        };
        println!("Time Complexity:  {}", time);
        println!("Space Complexity: O(1)");
        println!("========================================\n");
    }
}
