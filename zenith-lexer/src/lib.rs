mod lexer;
use lexer::Lexer;

pub mod types;
use types::{BuiltinType, EnrichedToken, Keyword, Number, Token};

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
    ch.is_ascii_digit()
}

pub fn tokenize_code(code: &str) -> Vec<EnrichedToken> {
    let mut lexer = Lexer::new(code.chars().collect());
    let mut tokens = vec![];

    while let Some(ch) = lexer.get_current_char() {
        let start_pos = lexer.get_position();

        let token: Token = match ch {
            // its a whitespace
            ch if ch.is_whitespace() => {
                let len = lexer.skip_until(|ch| !ch.is_whitespace());
                Token::Whitespace(len)
            }
            // its an identifier
            ch if is_identifier(ch) => {
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
                while let Some(ch) = lexer.get_current_char() {
                    match ch {
                        '\\' => {
                            lexer.increment_idx();
                            if let Some(ch) = lexer.get_current_char() {
                                s.push(ch);
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
            ch if is_number(ch) => {
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
            ch if is_punctuation(ch) => {
                let double_matched = match lexer.peek_next_char() {
                    Some(next_ch) if is_punctuation(next_ch) => {
                        // because we previously peeked next_ch, we need to advance the idx
                        lexer.increment_idx();

                        match (ch, next_ch) {
                            ('[', ']') => Some(Token::BuiltinType(BuiltinType::Array)),

                            (':', '=') => Some(Token::Assign),
                            ('=', '=') => Some(Token::Eq),
                            ('!', '=') => Some(Token::NotEq),
                            ('>', '=') => Some(Token::GreaterThanOrEq),
                            ('<', '=') => Some(Token::LessThanOrEq),
                            ('-', '>') => Some(Token::ReturnType),
                            ('+', '=') => Some(Token::RelationalPlus),
                            ('-', '=') => Some(Token::RelationalMinus),
                            ('*', '=') => Some(Token::RelationalMul),
                            ('/', '=') => Some(Token::RelationalDiv),
                            _ => {
                                lexer.decrement_idx();
                                None
                            }
                        }
                    }
                    _ => None,
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

        tokens.push(EnrichedToken::new(start_pos, lexer.get_position(), token));
        lexer.increment_idx();
    }

    if let Some(last_token) = tokens.last() {
        if let Token::Whitespace(_) = last_token.strip_token() {
            tokens.pop();
        }
    }

    tokens
}

mod tests {
    #[allow(unused)]
    use super::{tokenize_code, BuiltinType, Keyword, Number, Token};

    #[allow(unused)]
    macro_rules! next_token {
        ($a:expr) => {
            *$a.next().unwrap().strip_token()
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

        let tokens = tokenize_code(code);
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

        let tokens = tokenize_code(code);
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
