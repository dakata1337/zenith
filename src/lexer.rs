#[derive(Debug, Clone, Copy)]
pub struct Position {
    line: usize,
    column: usize,
}
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

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

#[derive(Debug)]
pub struct EnrichedToken {
    start: Position,
    end: Position,
    token: Token,
}
impl EnrichedToken {
    pub fn get_token(&self) -> &Token {
        &self.token
    }
}
impl std::fmt::Display for EnrichedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let left = format!("{}", self.start);
        let right = format!("{}", self.end);
        write!(f, "{:>6}-{:<6} {:?}", left, right, self.token)
    }
}

pub struct Lexer {
    chars: Vec<char>,
    idx: usize,
    pos: Position,
    last_pos: Position,
}

impl Lexer {
    pub fn new(chars: Vec<char>) -> Self {
        Self {
            chars,
            idx: 0,
            pos: Position { line: 1, column: 1 },
            last_pos: Position { line: 1, column: 1 },
        }
    }

    /// Returns the next character, without moving the lexer index.
    fn peek_next_char(&self) -> Option<char> {
        self.chars.get(self.idx + 1).copied()
    }

    /// Collects and returns all characters until the given condition is met.
    fn collect_until(&mut self, cond: fn(char) -> bool) -> String {
        // TODO: change this to use string slices (&str)
        let mut s = String::new();

        while let Some(ch) = self.chars.get(self.idx) {
            if cond(*ch) {
                self.decrement_idx();
                break;
            }
            s.push(*ch);
            self.increment_idx();
        }

        s
    }

    /// Skip until the given condition is met and returns the number of characters skipped.
    fn skip_until(&mut self, cond: fn(char) -> bool) -> usize {
        let start = self.idx;
        while let Some(ch) = self.chars.get(self.idx) {
            if cond(*ch) {
                self.decrement_idx();
                break;
            }
            self.increment_idx();
        }
        self.idx - start + 1
    }

    fn decrement_idx(&mut self) {
        self.pos = self.last_pos;
        self.idx -= 1;
    }

    fn increment_idx(&mut self) {
        self.last_pos = self.pos;
        // TODO: this doesn't work when '\n' whitespace is at the end of the file.
        // It registers the end position of the token as `line + 1` which doesn't exist.
        match self.chars.get(self.idx) {
            Some('\n') => {
                self.pos.line += 1;
                self.pos.column = 1;
            }
            _ => {
                self.pos.column += 1;
            }
        }
        self.idx += 1;
    }
}

#[inline]
const fn is_identifier(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

#[inline]
#[rustfmt::skip]
const fn is_punctuation(ch: char) -> bool {
    matches!( ch, 
        '!' | '#'  | '$' | '%' | '&' | '(' | ')' | '*' | '+' | '-' |
        ',' | '.'  | '/' | ':' | ';' | '<' | '=' | '>' | '?' | '@' |
        '[' | '\\' | ']' | '^' | '{' | '|' | '}'
    )
}

#[inline]
const fn is_number(ch: char) -> bool {
    matches!(ch, '0'..='9')
}

pub fn lex_code(code: String) -> Vec<EnrichedToken> {
    let mut lexer = Lexer::new(code.chars().collect());
    let mut tokens = vec![];

    while let Some(ch) = lexer.chars.get(lexer.idx) {
        let start_pos = lexer.pos;

        let token: Token = match ch {
            // its a whitespace
            ch if ch.is_whitespace() => {
                let len = lexer.skip_until(|ch| !ch.is_whitespace());
                Token::Whitespace(len)
            }
            // its an identifier
            ch if is_identifier(*ch) => {
                let ident = lexer.collect_until(|ch| !(is_identifier(ch) || is_number(ch)));
                match ident.as_str() {
                    "int" => Token::BuiltinType(BuiltinType::Int),
                    "float" => Token::BuiltinType(BuiltinType::Float),
                    "boolean" => Token::BuiltinType(BuiltinType::Boolean),
                    "string" => Token::BuiltinType(BuiltinType::String),

                    "fn" => Token::Keyword(Keyword::Function),
                    "if" => Token::Keyword(Keyword::If),
                    "else" => Token::Keyword(Keyword::Else),
                    "end" => Token::Keyword(Keyword::End),
                    "return" => Token::Keyword(Keyword::Return),
                    "mut" => Token::Keyword(Keyword::Mutable),
                    "for" => Token::Keyword(Keyword::For),
                    "in" => Token::Keyword(Keyword::In),

                    _ => Token::Ident(ident),
                }
            }
            // its a string
            '"' => {
                lexer.increment_idx();
                let mut s = String::new();
                while let Some(ch) = lexer.chars.get(lexer.idx).copied() {
                    match ch {
                        '\\' => {
                            lexer.increment_idx();
                            if let Some(ch) = lexer.chars.get(lexer.idx) {
                                s.push(*ch);
                            }
                        }
                        '"' => break,
                        _ => {
                            s.push(ch);
                        }
                    }

                    lexer.increment_idx();
                }
                Token::String(s)
            }
            // its a comment
            '/' if lexer.peek_next_char().unwrap_or('/') == '/' => {
                lexer.increment_idx();
                lexer.increment_idx();
                let comment = lexer.collect_until(|c| c == '\n');
                Token::Comment(comment.trim().to_owned())
            }
            // its a number
            ch if is_number(*ch) => {
                // TODO: this has some weird behavior with incorrectly formatted floats
                // For example "6. 9" will be evaluated to `6`, whitespace '9'
                let number_str = lexer.collect_until(|ch| !is_number(ch) && ch != '.');

                if let Ok(n) = number_str.parse::<i64>() {
                    Token::Number(Number::Int(n))
                } else if let Ok(n) = number_str.parse::<f64>() {
                    Token::Number(Number::Float(n))
                } else {
                    unreachable!("how? `{}` not a i64 or f64", number_str);
                }
            }
            // its a punctuation
            ch if is_punctuation(*ch) => {
                // NOTE: i hate this! Still waiting for if-let chains to be stabalized :')
                let next_ch = lexer.peek_next_char();
                let double_matched = if next_ch.is_some() && is_punctuation(next_ch.unwrap()) {
                    // this is safe because we checked it with .is_some()
                    let next_ch = next_ch.unwrap();
                    // because we previously peeked next_ch, we need to advance the idx
                    lexer.idx += 1;

                    match (*ch, next_ch) {
                        (':', '=') => Some(Token::Assign),
                        ('=', '=') => Some(Token::Eq),
                        ('!', '=') => Some(Token::NotEq),
                        ('>', '=') => Some(Token::GreaterThanOrEq),
                        ('<', '=') => Some(Token::LessThanOrEq),
                        ('[', ']') => Some(Token::BuiltinType(BuiltinType::Array)),
                        ('-', '>') => Some(Token::ReturnType),
                        ('+', '=') => Some(Token::RelationalPlus),
                        ('-', '=') => Some(Token::RelationalMinus),
                        ('*', '=') => Some(Token::RelationalMul),
                        ('/', '=') => Some(Token::RelationalDiv),
                        _ => {
                            lexer.idx -= 1;
                            None
                        }
                    }
                } else {
                    None
                };

                if let Some(token) = double_matched {
                    token
                } else {
                    match ch {
                        '!' => Token::Bang,
                        '#' => Token::Shabang,
                        '$' => Token::Dollar,
                        '%' => Token::Percent,
                        '&' => Token::Ampersand,
                        '(' => Token::OpenParen,
                        ')' => Token::CloseParen,
                        '*' => Token::Asterisk,
                        '+' => Token::Plus,
                        '-' => Token::Minus,
                        ',' => Token::Comma,
                        '.' => Token::Period,
                        '/' => Token::Slash,
                        ':' => Token::Colon,
                        ';' => Token::Semicolon,
                        '<' => Token::LessThan,
                        '=' => Token::Equals,
                        '>' => Token::GreaterThan,
                        '?' => Token::QuestionMark,
                        '@' => Token::AtSign,
                        '[' => Token::OpenSquareBracket,
                        '\\' => Token::Backslash,
                        ']' => Token::CloseSquareBracket,
                        '^' => Token::Caret,
                        '{' => Token::OpenCurlyBrace,
                        '|' => Token::VerticalBar,
                        '}' => Token::CloseCurlyBrace,
                        _ => unimplemented!("punctuation: `{}`", ch),
                    }
                }
            }
            _ => panic!("unexpected char: `{}`", ch),
        };

        tokens.push(EnrichedToken {
            start: start_pos,
            end: lexer.pos,
            token,
        });

        lexer.increment_idx();
    }

    if let Some(last_token) = tokens.last() {
        if let Token::Whitespace(_) = last_token.token {
            tokens.pop();
        }
    }

    tokens
}

mod tests {
    #[allow(unused)]
    use super::{Number, Token, Keyword, BuiltinType};

    #[allow(unused)]
    macro_rules! next_token {
        ($a:expr) => {
            $a.next().unwrap().token
        };
    }

    #[test]
    fn lex1() {
        #[rustfmt::skip]
        let code = 
r#"a := 34
b := 35
res := a + b
print(res)"#;

        let tokens = super::lex_code(code.to_owned());
        let mut tokens = tokens.iter();

        assert_eq!(next_token!(tokens), Token::Ident("a".to_owned()));
        assert_eq!(next_token!(tokens), Token::Whitespace(1));
        assert_eq!(next_token!(tokens), Token::Assign);
        assert_eq!(next_token!(tokens), Token::Whitespace(1));
        assert_eq!(next_token!(tokens), Token::Number(Number::Int(34)));
        assert_eq!(next_token!(tokens), Token::Whitespace(1));

        assert_eq!(next_token!(tokens), Token::Ident("b".to_owned()));
        assert_eq!(next_token!(tokens), Token::Whitespace(1));
        assert_eq!(next_token!(tokens), Token::Assign);
        assert_eq!(next_token!(tokens), Token::Whitespace(1));
        assert_eq!(next_token!(tokens), Token::Number(Number::Int(35)));
        assert_eq!(next_token!(tokens), Token::Whitespace(1));

        assert_eq!(next_token!(tokens), Token::Ident("res".to_owned()));
        assert_eq!(next_token!(tokens), Token::Whitespace(1));
        assert_eq!(next_token!(tokens), Token::Assign);
        assert_eq!(next_token!(tokens), Token::Whitespace(1));
        assert_eq!(next_token!(tokens), Token::Ident("a".to_owned()));
        assert_eq!(next_token!(tokens), Token::Whitespace(1));
        assert_eq!(next_token!(tokens), Token::Plus);
        assert_eq!(next_token!(tokens), Token::Whitespace(1));
        assert_eq!(next_token!(tokens), Token::Ident("b".to_owned()));
        assert_eq!(next_token!(tokens), Token::Whitespace(1));

        assert_eq!(next_token!(tokens), Token::Ident("print".to_owned()));
        assert_eq!(next_token!(tokens), Token::OpenParen);
        assert_eq!(next_token!(tokens), Token::Ident("res".to_owned()));
        assert_eq!(next_token!(tokens), Token::CloseParen);

        assert!(tokens.next().is_none());
    }

    #[test]
    fn lex2() {
        #[rustfmt::skip]
        let code = 
r#"mut res := []
res.push(34)
res.push(35)
sum := res[0] + res[1]
"#;

        let tokens = super::lex_code(code.to_owned());
        println!("{:#?}", tokens);
        let mut tokens = tokens.iter();

        assert_eq!(next_token!(tokens), Token::Keyword(Keyword::Mutable));
        assert_eq!(next_token!(tokens), Token::Whitespace(1));
        assert_eq!(next_token!(tokens), Token::Ident("res".to_owned()));
        assert_eq!(next_token!(tokens), Token::Whitespace(1));
        assert_eq!(next_token!(tokens), Token::Assign);
        assert_eq!(next_token!(tokens), Token::Whitespace(1));
        assert_eq!(next_token!(tokens), Token::BuiltinType(BuiltinType::Array));
        assert_eq!(next_token!(tokens), Token::Whitespace(1));

        assert_eq!(next_token!(tokens), Token::Ident("res".to_owned()));
        assert_eq!(next_token!(tokens), Token::Period);
        assert_eq!(next_token!(tokens), Token::Ident("push".to_owned()));
        assert_eq!(next_token!(tokens), Token::OpenParen);
        assert_eq!(next_token!(tokens), Token::Number(Number::Int(34)));
        assert_eq!(next_token!(tokens), Token::CloseParen);
        assert_eq!(next_token!(tokens), Token::Whitespace(1));

        assert_eq!(next_token!(tokens), Token::Ident("res".to_owned()));
        assert_eq!(next_token!(tokens), Token::Period);
        assert_eq!(next_token!(tokens), Token::Ident("push".to_owned()));
        assert_eq!(next_token!(tokens), Token::OpenParen);
        assert_eq!(next_token!(tokens), Token::Number(Number::Int(35)));
        assert_eq!(next_token!(tokens), Token::CloseParen);
        assert_eq!(next_token!(tokens), Token::Whitespace(1));

        assert_eq!(next_token!(tokens), Token::Ident("sum".to_owned()));
        assert_eq!(next_token!(tokens), Token::Whitespace(1));
        assert_eq!(next_token!(tokens), Token::Assign);
        assert_eq!(next_token!(tokens), Token::Whitespace(1));
        assert_eq!(next_token!(tokens), Token::Ident("res".to_owned()));
        assert_eq!(next_token!(tokens), Token::OpenSquareBracket);
        assert_eq!(next_token!(tokens), Token::Number(Number::Int(0)));
        assert_eq!(next_token!(tokens), Token::CloseSquareBracket);
        assert_eq!(next_token!(tokens), Token::Whitespace(1));
        assert_eq!(next_token!(tokens), Token::Plus);
        assert_eq!(next_token!(tokens), Token::Whitespace(1));
        assert_eq!(next_token!(tokens), Token::Ident("res".to_owned()));
        assert_eq!(next_token!(tokens), Token::OpenSquareBracket);
        assert_eq!(next_token!(tokens), Token::Number(Number::Int(1)));
        assert_eq!(next_token!(tokens), Token::CloseSquareBracket);

        assert!(tokens.next().is_none());
    }
}
