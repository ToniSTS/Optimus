use crate::ast::{BinaryOperator, Expression, Literal, Statement};
use std::collections::HashMap;

pub struct Analyzer {
    time_cost: usize,
    space_cost: usize,
    current_depth: usize,
    max_depth: usize,
    memory: HashMap<String, Literal>, // The Interpreter's Memory Bank
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
                self.memory.insert(name.clone(), val); // Store in memory
            }
            Statement::Assignment { name, value } => {
                self.time_cost += 1;
                let val = self.evaluate_expression(value);
                self.memory.insert(name.clone(), val); // Update memory
            }
            Statement::Print(expr) => {
                self.time_cost += 1;
                let val = self.evaluate_expression(expr);
                print!("> ");
                Self::print_literal(&val);
            }
            Statement::Expression(expr) => {
                self.evaluate_expression(expr);
            }
            Statement::Block(stmts) => {
                for s in stmts {
                    self.execute_statement(s);
                }
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

                // 1. Initialize the loop variable
                self.execute_statement(init);

                // 2. Loop execution
                loop {
                    let cond = self.evaluate_expression(condition);
                    let should_run = match cond {
                        Literal::Bool(b) => b,
                        _ => false, // Fallback if condition isn't a boolean
                    };

                    if !should_run {
                        break;
                    }

                    self.execute_statement(body);
                    self.execute_statement(increment);
                }

                self.current_depth -= 1;
            }
        }
    }

    fn evaluate_expression(&mut self, expr: &Expression) -> Literal {
        match expr {
            Expression::Literal(l) => l.clone(),
            Expression::Identifier(name) => {
                // Fetch from memory, default to 0 if not found
                self.memory.get(name).cloned().unwrap_or(Literal::Int(0))
            }
            Expression::BinaryOp {
                left,
                operator,
                right,
            } => {
                self.time_cost += 1; // Each math operation takes time
                let l_val = self.evaluate_expression(left);
                let r_val = self.evaluate_expression(right);
                self.eval_math(operator, &l_val, &r_val)
            }
            Expression::Call { .. } => {
                self.time_cost += 1;
                Literal::Int(0) // Placeholder for functions
            }
        }
    }

    // A helper to handle the actual mathematics
    fn eval_math(&self, op: &BinaryOperator, left: &Literal, right: &Literal) -> Literal {
        match (left, right) {
            // Integer Math
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
            // Floating Point Math
            _ => {
                let l_val = match left {
                    Literal::Int(i) => *i as f64,
                    Literal::Float(f) => *f,
                    _ => 0.0,
                };
                let r_val = match right {
                    Literal::Int(i) => *i as f64,
                    Literal::Float(f) => *f,
                    _ => 0.0,
                };
                match op {
                    BinaryOperator::Add => Literal::Float(l_val + r_val),
                    BinaryOperator::Subtract => Literal::Float(l_val - r_val),
                    BinaryOperator::Multiply => Literal::Float(l_val * r_val),
                    BinaryOperator::Divide => Literal::Float(l_val / r_val),
                    BinaryOperator::Less => Literal::Bool(l_val < r_val),
                    BinaryOperator::Greater => Literal::Bool(l_val > r_val),
                    BinaryOperator::Equal => Literal::Bool(l_val == r_val),
                    BinaryOperator::NotEqual => Literal::Bool(l_val != r_val),
                }
            }
        }
    }

    fn print_literal(l: &Literal) {
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

        let time_complexity = match self.max_depth {
            0 => "O(1) [Constant Time]",
            1 => "O(N) [Linear Time]",
            2 => "O(N^2) [Quadratic Time]",
            3 => "O(N^3) [Cubic Time]",
            _ => "O(N^X) [Polynomial Time]",
        };

        println!("Time Complexity:  {}", time_complexity);
        println!("  -> Dynamic Operations Executed: {}", self.time_cost);
        println!("Space Complexity: O(1) [Constant Space]");
        println!("  -> Variables Allocated in Memory: {}", self.space_cost);
        println!("========================================\n");
    }
}
