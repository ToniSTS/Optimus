mod ast;
mod lexer;
mod parser;

use chumsky::Parser;
use lexer::Token;
use logos::Logos;

fn main() {
    // Our ultimate math test for Optimus
    let source_code = r#"
        mut float pi = 3.14159;
        float radius = 5.0;
        
        mut float area = pi * radius * radius;
        
        float perimeter = 2.0 * pi * radius + 10.0;
        
        print(area);
    "#;

    println!("--- Optimus Compiler: Lexing & Parsing ---");
    println!("Source Code:\n{}\n", source_code);

    // 1. Lexing (Text -> Tokens)
    let tokens: Vec<_> = Token::lexer(source_code).filter_map(|t| t.ok()).collect();

    // 2. Parsing (Tokens -> AST Tree)
    let (ast, errors) = parser::parser().parse_recovery(tokens);

    if errors.is_empty() {
        if let Some(tree) = ast {
            println!("--- Optimus AST Successfully Built ---");
            println!("{:#?}", tree);
        }
    } else {
        println!("Parsing Errors found:");
        for err in errors {
            println!("{:?}", err);
        }
    }
}
