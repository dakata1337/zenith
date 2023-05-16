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

#[derive(Debug)]
pub enum Number {
    Int(i64),
    Float(f64),
}

#[derive(Debug)]
pub enum Token {
    Ident(String),
    Number(Number),
    String(String),
    Comment(String),
    Whitespace(usize),
    Punctuation(char),
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
        '!' | '#'  | '$' | '%' | '&' | '(' | ')' | '*' | '+' | ',' |
        '-' | '.'  | '/' | ':' | ';' | '<' | '=' | '>' | '?' | '@' |
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
                // TODO: this doesn't allow variables to end with numbers :)
                let ident = lexer.collect_until(|ch| !is_identifier(ch));
                Token::Ident(ident)
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
            '/' if lexer.chars.get(lexer.idx + 1).unwrap_or(&'/') == &'/' => {
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
                // TODO: convert chars to punctuation tokens
                Token::Punctuation(*ch)
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

    tokens
}
