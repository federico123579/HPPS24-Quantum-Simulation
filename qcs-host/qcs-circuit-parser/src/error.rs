use nom::error::ParseError;

use crate::parser::Parser;

#[derive(Debug)]
pub struct Error<'s> {
    pub input: Parser<'s>,
    pub span: std::ops::Range<usize>,
    pub kind: ErrorKind,
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
