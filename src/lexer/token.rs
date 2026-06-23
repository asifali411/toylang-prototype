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
        match &self.kind {
            TokenKind::FLOAT(val) => write!(f, "FLOAT({})", val),
            TokenKind::INT(val) => write!(f, "INT({})", val),
            TokenKind::PLUS => write!(f, "PLUS"),
            TokenKind::MINUS => write!(f, "MINUS"),
            TokenKind::STAR => write!(f, "STAR"),
            TokenKind::SLASH => write!(f, "SLASH"),
            TokenKind::EQUAL => write!(f, "EQUAL"),
            TokenKind::NOT => write!(f, "NOT"),
            TokenKind::LESS => write!(f, "LESS"),
            TokenKind::LESS_EQ => write!(f, "LESS_EQ"),
            TokenKind::GREAT => write!(f, "GREAT"),
            TokenKind::GREAT_EQ => write!(f, "GREAT_EQ"),
            TokenKind::EQ_EQ => write!(f, "EQ_EQ"),
            TokenKind::NOT_EQ => write!(f, "NOT_EQ"),
            TokenKind::OPEN_PAREN => write!(f, "OPEN_BRACE"),
            TokenKind::CLOSE_PAREN => write!(f, "CLOSE_BRACE"),
            TokenKind::EOF => write!(f, "EOF"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    FLOAT(f32),
    INT(i64),

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

    EOF,
}
