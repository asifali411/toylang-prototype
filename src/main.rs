use colored::Colorize;
use std::{
    fs,
    io::{self, Write},
    process::ExitCode,
};
use toylang;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const UNINSTALL_SH: &str = include_str!("../scripts/uninstall.sh");
const UNINSTALL_PS1: &str = include_str!("../scripts/uninstall.ps1");

enum Command {
    Run(String),
    Help,
    Version,
    Uninstall,
}

fn parse_args() -> Result<Command, String> {
    let mut args = std::env::args().skip(1);
    let first = args.next().ok_or_else(|| {
        format!(
            "{}\nusage: {} {}",
            "Expected a file path".red(),
            "toylang".bold(),
            "<file.toy>".cyan()
        )
    })?;

    match first.as_str() {
        "-h" | "--help" => Ok(Command::Help),
        "-v" | "--version" => Ok(Command::Version),
        "-u" | "--uninstall" => Ok(Command::Uninstall),
        _ if first.starts_with('-') => {
            Err(format!("{} '{}'", "unknown flag:".red(), first.yellow()))
        }
        _ => Ok(Command::Run(first)),
    }
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
    println!(
        "    {}, {}  Uninstall toylang from your system",
        "-u".yellow(),
        "--uninstall".yellow()
    );
}

fn confirm(prompt: &str) -> bool {
    print!("{prompt} ");
    let _ = io::stdout().flush();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return false;
    }
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

fn run_uninstaller() -> ExitCode {
    println!(
        "{}",
        "This will remove toylang and its files from your system.".yellow()
    );
    if !confirm(&format!("{} [y/N]:", "Are you sure you want to continue?".bold())) {
        println!("{}", "Uninstall cancelled.".green());
        return ExitCode::SUCCESS;
    }

    let tmp_dir = std::env::temp_dir();

    let (script_path, status) = if cfg!(target_os = "windows") {
        let path = tmp_dir.join("toylang_uninstall.ps1");
        if let Err(err) = fs::write(&path, UNINSTALL_PS1) {
            eprintln!("{} could not write uninstall script: {}", "Error:".red().bold(), err);
            return ExitCode::FAILURE;
        }
        let status = std::process::Command::new("powershell")
            .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-File"])
            .arg(&path)
            .status();
        (path, status)
    } else {
        let path = tmp_dir.join("toylang_uninstall.sh");
        if let Err(err) = fs::write(&path, UNINSTALL_SH) {
            eprintln!("{} could not write uninstall script: {}", "Error:".red().bold(), err);
            return ExitCode::FAILURE;
        }
        let status = std::process::Command::new("sh").arg(&path).status();
        (path, status)
    };

    let result = match status {
        Ok(s) if s.success() => ExitCode::SUCCESS,
        Ok(s) => {
            eprintln!("{} uninstaller exited with {}", "Error:".red().bold(), s);
            ExitCode::FAILURE
        }
        Err(err) => {
            eprintln!("{} failed to run uninstaller: {}", "Error:".red().bold(), err);
            ExitCode::FAILURE
        }
    };

    let _ = fs::remove_file(&script_path);
    result
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
        Command::Uninstall => run_uninstaller(),
        Command::Run(path) => match fs::read_to_string(&path) {
            Ok(source) => toylang::run(source),
            Err(err) => {
                eprintln!("{} could not read '{}': {}", "Error:".red().bold(), path, err);
                ExitCode::FAILURE
            }
        },
    }
}