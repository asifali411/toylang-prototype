use std::{fs, process::ExitCode};
use toylang;

fn main() -> ExitCode {
    
    let path = std::env::args().nth(1);

    if path.is_none() {
        eprintln!("Expect file path\nusage toylang <file.toy>");
        return ExitCode::FAILURE;
    }

    let content = fs::read_to_string(path.unwrap());

    match content {
       Ok(source) => toylang::run(source),
       Err(err) => {
           eprintln!("{}", err);
           ExitCode::FAILURE
       },
    }
}
