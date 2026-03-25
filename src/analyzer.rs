use crate::ast::{Expression, Statement};

pub struct Analyzer {
    time_cost: usize,
    space_cost: usize,
    current_depth: usize,
    max_depth: usize,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            time_cost: 0,
            space_cost: 0,
            current_depth: 0,
            max_depth: 0,
        }
    }

    pub fn analyze(&mut self, ast: &[Statement]) {
        for stmt in ast {
            self.visit_statement(stmt);
        }
        self.print_report();
    }

    fn visit_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::VariableDecl { value, .. } => {
                self.space_cost += 1;
                self.time_cost += 1;
                self.visit_expression(value);
            }
            Statement::Assignment { value, .. } => {
                self.time_cost += 1;
                self.visit_expression(value);
            }
            Statement::Print(expr) => {
                self.time_cost += 1;
                self.visit_expression(expr);
            }
            Statement::Expression(expr) => {
                self.visit_expression(expr);
            }
            Statement::Block(stmts) => {
                for s in stmts {
                    self.visit_statement(s);
                }
            }
            Statement::ForLoop {
                init,
                condition,
                increment,
                body,
            } => {
                // Initialize loop variable
                self.visit_statement(init);

                // Track loop depth
                self.current_depth += 1;
                if self.current_depth > self.max_depth {
                    self.max_depth = self.current_depth;
                }

                // Analyze loop contents
                self.visit_expression(condition);
                self.visit_statement(increment);
                self.visit_statement(body);

                self.current_depth -= 1;
            }
        }
    }

    fn visit_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::BinaryOp { left, right, .. } => {
                self.time_cost += 1;
                self.visit_expression(left);
                self.visit_expression(right);
            }
            Expression::Call { arguments, .. } => {
                self.time_cost += 1;
                for arg in arguments {
                    self.visit_expression(arg);
                }
            }
            Expression::Literal(_) | Expression::Identifier(_) => {}
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
        println!("Space Complexity: O(1) [Constant Space]");
        println!("Total Variables Allocated: {}", self.space_cost);
        println!("========================================\n");
    }
}
