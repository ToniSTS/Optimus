mod analyzer;
mod ast;
mod lexer;
mod parser;

use chumsky::Parser;
use lexer::Token;
use logos::Logos;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("========================================");
        println!("Optimus Compiler & Complexity Analyzer");
        println!("========================================");
        println!("Usage: cargo run -- <file.op>");
        process::exit(1);
    }

    let filename = &args[1];

    if !filename.ends_with(".op") {
        eprintln!("Error: Optimus files must use the .op extension.");
        process::exit(1);
    }

    let source_code = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => {
            eprintln!("Error: Could not find or read file '{}'", filename);
            process::exit(1);
        }
    };

    println!("Analyzing: {}", filename);
    println!("----------------------------------------");

    let tokens: Vec<_> = Token::lexer(&source_code).filter_map(|t| t.ok()).collect();
    let (ast, errors) = parser::parser().parse_recovery(tokens);

    if errors.is_empty() {
        if let Some(tree) = ast {
            println!("Syntax Validated!");

            let mut analyzer = analyzer::Analyzer::new();
            analyzer.analyze(&tree);
        }
    } else {
        println!("Syntax Errors Found:");
        for err in errors {
            println!("{:?}", err);
        }
    }
}
