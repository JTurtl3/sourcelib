use super::token::TokenKind;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
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
                ErrorKind::IoError(e) => format!("IO Error '{}'", e),
            },
            self.line,
            self.column,
        )
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    UnterminatedString,
    UnexpectedToken(TokenKind),
    NoMatchingRightBrace,
    InvalidEscape(char),
    UnexpectedEOF,
    IoError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self { kind: ErrorKind::IoError(e), line: 0, column: 0 }
    }
}