use std::{fs, process::ExitCode};
use toylang;

fn main() -> ExitCode {
    let path = std::env::args().nth(1);

    if path.is_none() {
        eprintln!("Expect file path\nusage toylang <file.toy>");
        return ExitCode::FAILURE;
    }

    let path = path.unwrap();

    if path.starts_with('-') {
        match &path[..] {
            "--version" | "-v" => {
                let version = env!("CARGO_PKG_VERSION");
                println!("v{}", version);
                return ExitCode::SUCCESS;
            }
            _ => {
                eprintln!("cannot find command '{}'", path);
                return ExitCode::FAILURE;
            }
        }
    }

    let content = fs::read_to_string(path);

    match content {
        Ok(source) => toylang::run(source),
        Err(err) => {
            eprintln!("{}", err);
            ExitCode::FAILURE
        }
    }
}
