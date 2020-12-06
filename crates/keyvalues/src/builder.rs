use crate::parser::{Token, TokenType};
use crate::error::{Error, ErrorKind};
use crate::keyvalues::KeyValues;

// Construct KeyValues from a vec of tokens
pub fn build_keyvalues(tokens: &Vec<Token>) -> Result<KeyValues, Error> {
    Builder::from(tokens).build()
}

struct Builder<'a> {
    tokens: &'a Vec<Token>,
    result: KeyValues,
    current: usize,
}

impl<'a> Builder<'a> {
    fn from(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, result: KeyValues::new(), current: 0 }
    }

    // What a mess!
    // As far as I know, it works fine
    // but 1. it's a disaster
    // and 2. if parsing a subkey fails, it will panic.
    fn build(mut self) -> Result<KeyValues, Error> {
        while !self.is_at_end() {
            let t = self.advance();
            match &t.kind {
                TokenType::Str(s) => {
                    let key = s.clone();

                    let t = self.advance();
                    match &t.kind {
                        TokenType::Str(value) => {
                            self.result.add_value(&key, &value);
                        },
                        TokenType::LeftBrace => {
                            if let Some(index) = self.find_matching_brace() {
                                self.result.add_subkey(&key, &Builder::from(&self.tokens[self.current..index].to_vec()).build().expect("oh fiddlesticks"));
                                self.current = index+1;
                            } else {
                                return Err(unclosed_brace_err(t));
                            }
                            
                        },
                        TokenType::RightBrace => return Err(unexpected_token_err(t.clone())),

                        TokenType::EOF => return Err(unexpected_eof()),
                    }
                },

                TokenType::EOF => {}

                _ => return Err(unexpected_token_err(t.clone())),
            }

        }
        Ok(self.result)
    }

    fn find_matching_brace(&self) -> Option<usize> {
        let mut depth = 0;
        for (i, v) in self.tokens[self.current..].iter().enumerate() {
            if v.kind == TokenType::RightBrace {
                if depth == 0 {
                    return Some(i + self.current);
                } else {
                    depth -= 1;
                }
            } else if v.kind == TokenType::LeftBrace {
                depth += 1;
            }
        }
        None
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn advance(&mut self) -> Token {
        self.current += 1;
        self.tokens[self.current - 1].clone()
    }
}

fn unexpected_token_err(t: Token) -> Error {
    Error {
        kind: ErrorKind::UnexpectedToken(t.kind),
        line: t.line
    }
}

fn unclosed_brace_err(t: Token) -> Error {
    Error {
        kind: ErrorKind::NoMatchingRightBrace,
        line: t.line
    }
}

fn unexpected_eof() -> Error {
    Error {
        kind: ErrorKind::UnexpectedEOF,
        line: 0,
    }
}