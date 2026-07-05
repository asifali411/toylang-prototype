use colored::Colorize;
use std::{fs, process::ExitCode};
use toylang;

const VERSION: &str = env!("CARGO_PKG_VERSION");

enum Command {
    Run(String),
    Help,
    Version,
}

fn parse_args() -> Result<Command, String> {
    let mut args = std::env::args().skip(1);

    let first = args
        .next()
        .ok_or_else(|| format!(
            "{}\nusage: {} {}",
            "Expect file path".red(),
            "toylang".bold(),
            "<file.toy>".cyan()
        ))?;

    if let Some(flag) = first.strip_prefix('-') {
        return match flag.trim_start_matches('-') {
            "h" | "help" => Ok(Command::Help),
            "v" | "version" => Ok(Command::Version),
            _ => Err(format!(
                "{} '{}'",
                "cannot find command".red(),
                first.yellow()
            )),
        };
    }

    Ok(Command::Run(first))
}

fn print_help() {
    println!(
        "{} {} - run a toylang script\n",
        "toylang".bold().green(),
        "<file.toy>".cyan()
    );
    println!("{}", "USAGE:".bold().underline());
    println!("    {} {}", "toylang".green(), "<file.toy>".cyan());
    println!("    {} {}", "toylang".green(), "[FLAG]".cyan());
    println!();
    println!("{}", "FLAGS:".bold().underline());
    println!(
        "    {}, {}       Print this help message",
        "-h".yellow(),
        "--help".yellow()
    );
    println!(
        "    {}, {}    Print version information",
        "-v".yellow(),
        "--version".yellow()
    );
}

fn main() -> ExitCode {
    let command = match parse_args() {
        Ok(cmd) => cmd,
        Err(msg) => {
            eprintln!("{}", msg);
            return ExitCode::FAILURE;
        }
    };

    match command {
        Command::Help => {
            print_help();
            ExitCode::SUCCESS
        }
        Command::Version => {
            println!("{} {}", "toylang".green().bold(), format!("v{}", VERSION).cyan());
            ExitCode::SUCCESS
        }
        Command::Run(path) => match fs::read_to_string(&path) {
            Ok(source) => toylang::run(source),
            Err(err) => {
                eprintln!("{} {}", "Error:".red().bold(), err);
                ExitCode::FAILURE
            }
        },
    }
}