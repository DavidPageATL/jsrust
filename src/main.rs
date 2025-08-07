use std::env;
use std::fs;

mod lexer;
mod parser;
mod ast;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <javascript_file>", args[0]);
        std::process::exit(1);
    }
    
    let filename = &args[1];
    
    match fs::read_to_string(filename) {
        Ok(source) => {
            println!("Parsing JavaScript file: {}", filename);
            println!("Source code:\n{}\n", source);
            
            // Tokenize the source code
            let mut lexer = Lexer::new(&source);
            let tokens = lexer.tokenize();
            
            println!("Tokens:");
            for token in &tokens {
                println!("  {:?}", token);
            }
            println!();
            
            // Parse the tokens into an AST
            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Ok(ast) => {
                    println!("Abstract Syntax Tree:");
                    println!("{:#?}", ast);
                }
                Err(err) => {
                    eprintln!("Parse error: {}", err);
                    std::process::exit(1);
                }
            }
        }
        Err(err) => {
            eprintln!("Error reading file '{}': {}", filename, err);
            std::process::exit(1);
        }
    }
}
