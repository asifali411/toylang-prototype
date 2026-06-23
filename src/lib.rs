use std::process::ExitCode;

mod lexer;

use lexer::Lexer;

pub fn run(source: String) -> ExitCode {
    
    let lexer = Lexer::new(source);

    ExitCode::SUCCESS
}
