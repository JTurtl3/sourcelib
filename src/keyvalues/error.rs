use super::token::TokenKind;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
    pub line: usize,
    pub column: usize,
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} on line {}, column {}", match &self.kind {
                ErrorKind::UnterminatedString => format!("Unterminated string"),
                ErrorKind::InvalidEscape(c) => format!("Invalid escape sequence '\\{}'", c),
                ErrorKind::UnexpectedToken(t) => format!("Unexpected {}", match t {
                    TokenKind::LeftBrace => "{",
                    TokenKind::RightBrace => "}",
                    TokenKind::Str(s) => s.as_str(),
                    TokenKind::EOF => "EOF",
                }),
                ErrorKind::NoMatchingRightBrace => format!("No matching }}"),
                ErrorKind::UnexpectedEOF => format!("Unexpected End of File"),
            },
            self.line,
            self.column,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    UnterminatedString,
    UnexpectedToken(TokenKind),
    NoMatchingRightBrace,
    InvalidEscape(char),
    UnexpectedEOF,
}
