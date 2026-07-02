#![allow(warnings)]
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", &self.kind))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    NUM(f64),
    IDENT(String),
    STRING(String),

    IF,
    ELSE,
    VAR,
    TRUE,
    FALSE,
    LOOP,
    FUNC,
    RETURN,
    CLASS,
    INHERIT,
    IN,
    BREAK,
    CONTINUE,

    PLUS,
    MINUS,
    STAR,
    SLASH,
    MOD,
    INC,
    DEC,
    
    PLUS_EQ,
    MINUS_EQ,
    STAR_EQ,
    SLASH_EQ,
    MOD_EQ,
    
    EQUAL,
    NOT,

    LESS,
    LESS_EQ,
    GREAT,
    GREAT_EQ,
    EQ_EQ,
    NOT_EQ,

    OPEN_PAREN,
    CLOSE_PAREN,
    OPEN_BRACE,
    CLOSE_BRACE,
    OPEN_BRACK,
    CLOSE_BRACK,

    DOT,
    COMMA,
    SEMI,
    COLON,
    EOF,
}
