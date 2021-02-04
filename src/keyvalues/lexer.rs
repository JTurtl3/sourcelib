use super::error::{Error, ErrorKind};
use super::token::{Token, TokenKind};

use std::{
    fs::File,
    io::Read,
};

pub fn tokenize(string: &str) -> Result<Vec<Token>, Error> {
    Lexer::from(string).tokenize()
}

pub fn tokenize_file(path: &str) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let v = tokenize(content.as_str())?;
    Ok(v)
}

// todo: This is an awful transcribed lexer from Java
// Make it... rustier
#[derive(Default)]
struct Lexer<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    fn from(string: &'a str) -> Self {
        Self { source: string, line: 1, ..Default::default() }
    }

    // Not a reference, will take ownership and drop itself after being called
    fn tokenize(mut self) -> Result<Vec<Token>, Error>{
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        
        self.tokens.push(Token { line: self.tokens[self.tokens.len()-1].line, column: 1, kind: TokenKind::EOF });

        Ok(self.tokens)
    }

    fn scan_token(&mut self) -> Result<(), Error> {
        let c = self.advance();
        match c {
            '{' => self.add_token(TokenKind::LeftBrace),
            '}' => self.add_token(TokenKind::RightBrace),

            '"' => self.eat_string()?,

            _ if c == '\n' => { self.line += 1; self.column = 1; },
            _ if c.is_whitespace() => {},

            _ => self.eat_identifier(),
        }
        Ok(())
    }

    fn add_token(&mut self, kind: TokenKind) {
        self.tokens.push(Token {
            kind,
            line: self.line,
            column: self.column
        });
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.column += 1;
        self.source.chars().nth(self.current-1).unwrap() // no reason why this unwrap should fail
    }

    fn peek(&self) -> char {
        if !self.is_at_end() {
            self.source.chars().nth(self.current).unwrap()
        } else {
            '\0'
        }
    }

    fn eat_string(&mut self) -> Result<(), Error> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1; }
            self.advance();
        }

        if self.is_at_end() {
            Err(self.error(ErrorKind::UnterminatedString))
        } else {
            self.advance(); // last "
            let s = self.source[self.start+1..self.current-1].to_string();
            // todo: escape characters
            self.add_token(TokenKind::Str(s));
            Ok(())
        }
    }

    fn eat_identifier(&mut self) {
        while !self.peek().is_whitespace() && !self.is_at_end() {
            self.advance();
        }

        let s = self.source[self.start..self.current].to_string();
        self.add_token(TokenKind::Str(s));
    }

    fn error(&self, kind: ErrorKind) -> Error {
        Error {
            line: self.line,
            column: self.column,
            kind
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
