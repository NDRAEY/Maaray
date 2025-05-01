use crate::tokenizer::{TResult, Token, TokenKind};

#[derive(Debug)]
pub enum LexemKind {
    Ident(String),
    Number(String),
    StringLiteral(String),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Dot,
    Comma,
    Colon,
    Semicolon,
    Equals,
    Or,
    And,
    Less,
    Greater,
    Slash,
    Asterisk,
    Minus,
    Plus,
}

#[derive(Debug)]
pub struct Lexem {
    line: usize,
    column: usize,
    kind: LexemKind
}

pub struct Lexer<T: Iterator> {
    input: T,
}

impl<T: Iterator<Item = TResult<Token>>> Lexer<T> {
    pub fn new(input: T) -> Self {
        Self {
            input
        }
    }

    pub fn next(&mut self) -> Option<TResult<Token>> {
        self.input.next()
    }

    pub fn lex(&mut self) -> Option<TResult<Lexem>> {
        let next = self.next();

        match next {
            None => None,
            Some(token) => {
                match token {
                    Ok(token) => {
                        let kind = match token.kind {
                            TokenKind::Ident(id) => {
                                let ret = LexemKind::Ident(id);
                                ret
                            },
                            TokenKind::StringLiteral(st) => {
                                let ret = LexemKind::StringLiteral(st);
                                ret
                            },
                            TokenKind::Number(nr) => {
                                let ret = LexemKind::Number(nr);
                                ret
                            },
                            TokenKind::Symbol(sym) => {
                                match sym {
                                    '\n' | ' ' => return self.lex(),
                                    '(' => LexemKind::LParen,
                                    ')' => LexemKind::RParen,
                                    '{' => LexemKind::LBrace,
                                    '}' => LexemKind::RBrace,
                                    '=' => LexemKind::Equals,
                                    '|' => LexemKind::Or,
                                    '&' => LexemKind::And,
                                    '<' => LexemKind::Less,
                                    '>' => LexemKind::Greater,
                                    ';' => LexemKind::Semicolon,
                                    '+' => LexemKind::Plus,
                                    '-' => LexemKind::Minus,
                                    '*' => LexemKind::Asterisk,
                                    '/' => LexemKind::Slash,
                                    '.' => LexemKind::Dot,
                                    ',' => LexemKind::Comma,
                                    _ => {
                                        todo!("Not supported yet: {sym:?}");
                                    }
                                }
                            },
                        };

                        Some(Ok(Lexem {
                            line: token.line,
                            column: token.column,
                            kind
                        }))
                    }
                    Err(x) => 
                    {
                        Some(Err(x))
                    },
                }
            }
        }
    }
}

impl<T: Iterator<Item = TResult<Token>>> Iterator for Lexer<T> {
    type Item = TResult<Lexem>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lex()
    }
}