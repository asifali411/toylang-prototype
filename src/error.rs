use colored::*;

pub struct Error;

impl Error {

    fn create_error(error_type: &str, message: &str, line: usize, column: usize) -> String {
        let error_type = format!("{}:", error_type).red();
        let loc = format!(" at line: {}, col: {} ", line, column).black().on_green();

        format!("{}\n{}\n{}\n", error_type, message, loc)
    }

    fn create_error_without_loc(error_type: &str, message: &str) -> String {
        let error_type = format!("{}:", error_type).red();
        format!("{}\n{}", error_type, message)
    }

    //--------------------------------------------------------------------------------

    pub fn syntax_error(message: &str, line: usize, column: usize) -> String {
        Error::create_error("SyntaxError", message, line, column)
    }

    pub fn lexical_error(message: &str, line: usize, column: usize) -> String {
        Error::create_error("Lexical Error", message, line, column)
    }

    pub fn parse_error(message: &str, line: usize, column: usize) -> String {
        Error::create_error("Parse Error", message, line, column)
    }

    pub fn runtime_error(message: &str) -> String {
        Error::create_error_without_loc("Runtime Error", message)
    }

    pub fn semantic_error(message: &str, line: usize, column: usize) -> String {
        Error::create_error("Parse Error", message, line, column)
    }

    pub fn io_error(message: &str) -> String {
        Error::create_error_without_loc("IO Error", message)
    }
}
