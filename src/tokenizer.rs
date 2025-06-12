pub type TResult<T> = Result<T, TokenizerError>;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Ident(String),
    StringLiteral(String),
    Number(String),
    Symbol(char),
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub(crate) line: usize,
    pub(crate) column: usize,
    pub(crate) kind: TokenKind,
}

#[derive(Debug)]
pub struct TokenizerError {
    line: usize,
    column: usize,
    kind: TokenizerErrorKind,
}

impl TokenizerError {
    pub fn line(&self) -> usize { self.line }
    pub fn column(&self) -> usize { self.column }
}

#[derive(Debug)]
pub enum TokenizerErrorKind {
    UnterminatedString,
    InvalidEscapeSequence,
}

#[derive(Clone)]
pub struct Tokenizer<'a> {
    data: &'a str,
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(data: &'a str) -> Self {
        Self {
            data,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn current_char(&self) -> Option<char> {
        self.data[self.position..].chars().next()
    }

    fn advance(&mut self, count: usize) {
        for _ in 0..count {
            if let Some(ch) = self.current_char() {
                if ch == '\n' {
                    self.line += 1;
                    self.column = 1;
                } else {
                    self.column += 1;
                }
                self.position += ch.len_utf8();
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() && ch != '\n' {
                self.advance(1);
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance(1);
            } else {
                break;
            }
        }
        ident
    }

    fn read_number(&mut self) -> String {
        let mut ident = String::new();
        while let Some(ch) = self.current_char() {
            if ch.is_numeric() || ch == '\'' {
                ident.push(ch);
                self.advance(1);
            } else {
                break;
            }
        }
        ident
    }

    fn read_string(&mut self) -> TResult<String> {
        let mut string = String::new();
        self.advance(1); // Skip opening quote

        loop {
            match self.current_char() {
                Some('"') => {
                    self.advance(1);
                    return Ok(string);
                }
                Some('\\') => {
                    self.advance(1);
                    match self.current_char() {
                        Some('"') => {
                            string.push('"');
                            self.advance(1);
                        }
                        Some('\\') => {
                            string.push('\\');
                            self.advance(1);
                        }
                        Some('n') => {
                            string.push('\n');
                            self.advance(1);
                        }
                        // Add other escape sequences as needed
                        _ => {
                            return Err(TokenizerError {
                                line: self.line,
                                column: self.column,
                                kind: TokenizerErrorKind::InvalidEscapeSequence,
                            });
                        }
                    }
                }
                Some(ch) => {
                    string.push(ch);
                    self.advance(1);
                }
                None => {
                    return Err(TokenizerError {
                        line: self.line,
                        column: self.column,
                        kind: TokenizerErrorKind::UnterminatedString,
                    });
                }
            }
        }
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = TResult<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let ch = self.current_char()?;

        if ch.is_alphabetic() {
            let ident = self.read_identifier();
            Some(Ok(Token {
                line: self.line,
                column: self.column,
                kind: TokenKind::Ident(ident),
            }))
        } else if ch.is_numeric() {
            let ident = self.read_number();
            Some(Ok(Token {
                line: self.line,
                column: self.column,
                kind: TokenKind::Number(ident),
            }))
        } else if ch == '"' {
            Some(self.read_string().map(|a| Token {
                line: self.line,
                column: self.column,
                kind: TokenKind::StringLiteral(a),
            }))
        } else {
            self.advance(1);
            Some(Ok(Token {
                line: self.line,
                column: self.column,
                kind: TokenKind::Symbol(ch),
            }))
        }
    }
}
