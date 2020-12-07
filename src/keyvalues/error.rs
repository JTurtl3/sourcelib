use super::parser::TokenType;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub line: usize,
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} on line {}", match &self.kind {
                ErrorKind::UnterminatedString => format!("Unterminated string"),
                ErrorKind::InvalidEscape(c) => format!("Invalid escape sequence '\\{}'", c),
                ErrorKind::UnexpectedToken(t) => format!("Unexpected {}", match t {
                    TokenType::LeftBrace => "{",
                    TokenType::RightBrace => "}",
                    TokenType::Str(s) => s.as_str(),
                    TokenType::EOF => "EOF",
                }),
                ErrorKind::NoMatchingRightBrace => format!("No matching }}"),
                ErrorKind::UnexpectedEOF => format!("Unexpected End of File"),
            },
            self.line
        )
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    UnterminatedString,
    UnexpectedToken(TokenType),
    NoMatchingRightBrace,
    InvalidEscape(char),
    UnexpectedEOF,
}