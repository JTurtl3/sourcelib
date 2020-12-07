use super::error::{Error, ErrorKind};

use std::{
    fs::File,
    io::Read,
};

pub fn parse(string: &str) -> Result<Vec<Token>, Error> {
    Parser::from(string).parse()
}

pub fn parse_file(path: &str) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let v = parse(content.as_str())?;
    Ok(v)
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub line: usize,
    //todo: pub character: char, the token's char index on the line
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftBrace, RightBrace,
    Str(String),

    EOF,
}


#[derive(Default)]
struct Parser<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Parser<'a> {
    fn from(string: &'a str) -> Self {
        Self { source: string, line: 1, ..Default::default() }
    }

    // Not a reference, will take ownership and drop itself after being called
    fn parse(mut self) -> Result<Vec<Token>, Error>{
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        
        self.tokens.push(Token { line: 0, kind: TokenType::EOF });

        Ok(self.tokens)
    }

    fn scan_token(&mut self) -> Result<(), Error> {
        let c = self.advance();
        match c {
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),

            '"' => self.eat_string()?,

            _ if c == '\n' => self.line += 1,
            _ if c.is_whitespace() => {},

            _ => self.eat_identifier(),
        }
        Ok(())
    }

    fn add_token(&mut self, kind: TokenType) {
        self.tokens.push(Token {
            kind,
            line: self.line
        });
    }

    fn advance(&mut self) -> char {
        self.current += 1;
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
            // todo: parse_escapes
            self.add_token(TokenType::Str(s));
            Ok(())
        }
    }

    fn eat_identifier(&mut self) {
        while !self.peek().is_whitespace() && !self.is_at_end() {
            self.advance();
        }

        let s = self.source[self.start..self.current].to_string();
        self.add_token(TokenType::Str(s));
    }

    fn error(&self, kind: ErrorKind) -> Error {
        Error {
            line: self.line,
            kind
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

//todo
// fn parse_escapes(s: &String) -> String {
//     let mut index = 0;
//     let mut result: String = String::new();
//     while index < s.len() {
//         let c = s.chars().nth(index).unwrap();
//         if c == '\\' {
//             //toodoo
//         }
//         index += 1;
//     }

//     result
// }

// const escape_chars: [char; 3] = ['n', '\\', '"'];

// fn is_valid_escape_char(c: char) -> bool {
//     escape_chars.contains(&c)
// }