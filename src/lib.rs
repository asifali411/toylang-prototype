#![allow(warnings)]

use std::process::ExitCode;

mod errors;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

pub fn run(source: String) -> ExitCode {
    let mut lexer = Lexer::new(source);
    let result = lexer.tokenize();

    match result {
        Ok(tokens) => {
            // println!("{:?}", tokens);

            let mut parser = Parser::new(&tokens);
            match parser.parse() {
                Ok(expr) => println!("{:#?}", expr),
                Err(e) => {
                    e.display();
                    return ExitCode::FAILURE;
                }
            }
        }
        Err(e) => {
            e.display();
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
