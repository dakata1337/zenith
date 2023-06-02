#[derive(Debug, PartialEq)]
pub enum Number {
    Int(i64),
    Float(f64),
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Function,
    If,
    Else,
    End,
    Return,
    Mutable,
    For,
    In,
}

#[derive(Debug, PartialEq)]
pub enum BuiltinType {
    Int,
    Float,
    Boolean,
    Array,
    String,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Number(Number),
    String(String),
    Comment(String),
    Whitespace(usize),

    Keyword(Keyword),
    BuiltinType(BuiltinType),
    ReturnType, // "->"

    Assign,          // ":="
    Eq,              // "=="
    NotEq,           // "!="
    GreaterThanOrEq, // ">="
    LessThanOrEq,    // "<="
    RelationalPlus,  // "+="
    RelationalMinus, // "-="
    RelationalMul,   // "*="
    RelationalDiv,   // "/="

    Bang,               // '!'
    Shabang,            // '#'
    Dollar,             // '$'
    Percent,            // '%'
    Ampersand,          // '&'
    OpenParen,          // '('
    CloseParen,         // ')'
    Asterisk,           // '*'
    Plus,               // '+'
    Minus,              // '-'
    Comma,              // ','
    Period,             // '.'
    Slash,              // '/'
    Colon,              // ':'
    Semicolon,          // ';'
    LessThan,           // '<'
    Equals,             // '='
    GreaterThan,        // '>'
    QuestionMark,       // '?'
    AtSign,             // '@'
    OpenSquareBracket,  // '['
    Backslash,          // '\'
    CloseSquareBracket, // ']'
    Caret,              // '^'
    OpenCurlyBrace,     // '{'
    VerticalBar,        // '|'
    CloseCurlyBrace,    // '}'
}

pub use crate::lexer::Position;

#[derive(Debug)]
pub struct EnrichedToken {
    start: Position,
    end: Position,
    token: Token,
}
impl EnrichedToken {
    pub fn new(start: Position, end: Position, token: Token) -> Self {
        Self { start, end, token }
    }
    pub fn strip_token(&self) -> &Token {
        &self.token
    }
    pub fn get_position(&self) -> (&Position, &Position) {
        (&self.start, &self.end)
    }
}
impl std::fmt::Display for EnrichedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let left = format!("{}", self.start);
        let right = format!("{}", self.end);
        write!(f, "{:>6}-{:<6} {:?}", left, right, self.token)
    }
}
