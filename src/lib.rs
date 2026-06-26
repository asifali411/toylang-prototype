#![allow(warnings)]
use std::process::ExitCode;

mod errors;
mod interpreter;
mod lexer;
mod parser;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use errors::lang_error::LangError;

pub fn run(source: String) -> ExitCode {
    match try_run(source) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            e.display();
            ExitCode::FAILURE
        }
    }
}

fn try_run(source: String) -> Result<(), LangError> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let statements = Parser::new(&tokens).parse()?;
    let mut interpreter = Interpreter::new();
    for statement in statements {
        interpreter.execute(&statement)?;
    }
    Ok(())
}