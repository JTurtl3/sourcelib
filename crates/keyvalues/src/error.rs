#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub line: usize,
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} on line {}", match self.kind {
                ErrorKind::UnterminatedString => format!("Unterminated string"),
                ErrorKind::InvalidEscape(c) => format!("Invalid escape sequence '\\{}'", c),
            },
            self.line
        )
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    UnterminatedString,
    InvalidEscape(char),
}