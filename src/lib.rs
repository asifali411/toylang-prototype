#![allow(warnings)]

use std::process::ExitCode;

mod error;
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

           let mut parser = Parser::new(tokens);
           let result = parser.parse();

           match result {
                Ok(ast) => {
                    println!("{:?}", ast);
                },
                Err(err) => {
                    eprintln!("{}", err);
                    return ExitCode::FAILURE;
                }
           }
       },
       Err(err) => {
            eprintln!("{}", err);
            return ExitCode::FAILURE;
       }
    }

    ExitCode::SUCCESS
}
