use crate::{error::Error, lexer::token::Token};

pub struct Lexer {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Self {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            column: 0,
            tokens: Vec::new(),
        }
    }

    pub fn tokenize(&self) -> Result<Vec<Token>, String> {
        Err(Error::syntax_error("Unexpected Error occured", self.line, self.column))
    }
}
