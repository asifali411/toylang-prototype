use std::{fs, process::ExitCode};
use toylang;

fn main() -> ExitCode {
    
    let content = fs::read_to_string("test.toy");

    match content {
       Ok(source) => toylang::run(source),
       Err(err) => {
           eprintln!("{}", err);
           ExitCode::FAILURE
       },
    }
}
