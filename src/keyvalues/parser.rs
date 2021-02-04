use super::{
    error::*,
    token::*,
    KeyValues,
};

// Construct KeyValues from a vec of tokens
pub fn parse_keyvalues(tokens: &Vec<Token>) -> Result<KeyValues, Error> {
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

    fn build(mut self) -> Result<KeyValues, Error> {
        while !self.is_at_end() {
            let t = self.advance();
            match &t.kind {
                TokenKind::Str(s) => {
                    let key = s.clone();

                    let t = self.advance();
                    match &t.kind {
                        TokenKind::Str(value) => {
                            self.result.add_value(&key, &value);
                        },
                        TokenKind::LeftBrace => {
                            self.parse_subkey(&key, t)?;
                        },
                        TokenKind::RightBrace => return Err(unexpected_token_err(t.clone())),

                        TokenKind::EOF => return Err(unexpected_eof(t)),
                    }
                },

                TokenKind::EOF => {}

                _ => return Err(unexpected_token_err(t.clone())),
            }

        }
        Ok(self.result)
    }

    fn parse_subkey(&mut self, key: &str, start_brace: Token) -> Result<(), Error> {
        if let Some(index) = self.find_matching_brace() {
            match &Builder::from(&self.tokens[self.current..index].to_vec()).build() {
                Ok(subkey) => {
                    self.result.add_subkey(&key, subkey);
                },
                
                Err(e) => return Err(e.clone()),
            }
            self.current = index+1;
            Ok(())
        } else {
            return Err(unclosed_brace_err(start_brace));
        }
    }

    fn find_matching_brace(&self) -> Option<usize> {
        let mut depth = 0;
        for (i, v) in self.tokens[self.current..].iter().enumerate() {
            if v.kind == TokenKind::RightBrace {
                if depth == 0 {
                    return Some(i + self.current);
                } else {
                    depth -= 1;
                }
            } else if v.kind == TokenKind::LeftBrace {
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
        line: t.line,
        column: t.column,
    }
}

fn unclosed_brace_err(t: Token) -> Error {
    Error {
        kind: ErrorKind::NoMatchingRightBrace,
        line: t.line,
        column: t.column,
    }
}

fn unexpected_eof(t: Token) -> Error {
    Error {
        kind: ErrorKind::UnexpectedEOF,
        line: t.line,
        column: t.column,
    }
}
