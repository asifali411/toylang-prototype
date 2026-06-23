#![allow(warnings)]
use std::process::ExitCode;

mod errors;
mod interpreter;
mod lexer;
mod parser;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

pub fn run(source: String) -> ExitCode {
    let mut lexer = Lexer::new(source);
    let result = lexer.tokenize();

    match result {
        Ok(tokens) => {
            //println!("{:?}", tokens);

            let mut parser = Parser::new(&tokens);
            match parser.parse() {
                Ok(expr) => {
                    //println!("{:#?}", expr);

                    let mut interpreter = Interpreter::new();
                    let result = interpreter.eval_expression(&expr);

                    match result {
                        Ok(res) => {
                            println!("{:?}", res);
                        }
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
        }
        Err(e) => {
            e.display();
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
