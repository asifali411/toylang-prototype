#![allow(warnings)]

use std::process::ExitCode;

mod lexer;
mod error;
use lexer::Lexer;

pub fn run(source: String) -> ExitCode {
    
    let lexer = Lexer::new(source);
    let result = lexer.tokenize();

    match result {
       Ok(tokens) => {
            
       },
       Err(err) => {
            eprintln!("{}", err);
            return ExitCode::FAILURE;
       }
    }

    ExitCode::SUCCESS
}
