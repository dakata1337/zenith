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

    /// Returns the position of the lexer (line and column).
    #[inline]
    pub fn get_position(&self) -> Position {
        self.pos
    }

    /// Returns the current character at the lexer index.
    #[inline]
    pub fn get_current_char(&self) -> Option<char> {
        self.chars.get(self.idx).copied()
    }

    /// Returns the next character, without moving the lexer index.
    #[inline]
    pub fn peek_next_char(&self) -> Option<char> {
        self.chars.get(self.idx + 1).copied()
    }

    /// Collects and returns all characters until the given condition is met.
    pub fn collect_until(&mut self, cond: fn(char) -> bool) -> String {
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
    pub fn skip_until(&mut self, cond: fn(char) -> bool) -> usize {
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

    pub fn decrement_idx(&mut self) {
        self.pos = self.last_pos;
        self.idx -= 1;
    }

    pub fn increment_idx(&mut self) {
        self.last_pos = self.pos;
        // NOTE: this doesn't work when '\n' whitespace is at the end of the file.
        // It registers the end position of the token as `line + 1` which doesn't exist.
        // HACK: fixed by just removing the last whitespace token in `crate::tokenize_code`
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

