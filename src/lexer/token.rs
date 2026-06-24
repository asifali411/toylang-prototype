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
    FLOAT(f32),
    INT(i64),
    IDENT(String),

    PRINT,
    VAR,
    TRUE,
    FALSE,

    PLUS,
    MINUS,
    STAR,
    SLASH,

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

    SEMI,
    EOF,
}
