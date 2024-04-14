use std::error::Error as StdError;

use nom::error::ParseError;

use crate::parser::Parser;

#[derive(Debug)]
pub struct Error<'s> {
    pub input: Parser<'s>,
    pub span: std::ops::Range<usize>,
    pub kind: ErrorKind,
}

impl StdError for Error<'_> {}

impl std::fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{kind} at {span:?}: {input:?}",
            kind = self.kind,
            span = self.span,
            input = self.input
        )
    }
}

impl<'s> ParseError<Parser<'s>> for Error<'s> {
    fn from_error_kind(input: Parser<'s>, kind: nom::error::ErrorKind) -> Self {
        Self {
            span: input.span(),
            input,
            kind: ErrorKind::NomError(kind),
        }
    }

    fn append(_: Parser<'_>, _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidToken,
    UnexpectedToken,
    UnexpectedEoF,
    NomError(nom::error::ErrorKind),
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::InvalidToken => write!(f, "Invalid token"),
            ErrorKind::UnexpectedToken => write!(f, "Unexpected token"),
            ErrorKind::UnexpectedEoF => write!(f, "Unexpected end of file"),
            ErrorKind::NomError(kind) => write!(f, "Nom error: {:?}", kind),
        }
    }
}
