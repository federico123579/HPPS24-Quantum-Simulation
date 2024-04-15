use std::ops::Range;

use nom::error::ParseError;
use thiserror::Error;

use crate::parser::Parser;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Text parser error: {0}")]
    TextParserError(#[from] OwnedParserError),
    #[error("Unsupported file extension")]
    UnsupportedFileExtension,
}

#[derive(Debug, Error)]
#[error("{kind}\n in \"{text}\"")]
pub struct OwnedParserError {
    pub kind: ErrorKind,
    pub input: String,
    pub text: String,
}

impl From<ParserError<'_>> for OwnedParserError {
    fn from(err: ParserError) -> Self {
        let source = err.input.0.source().to_string();
        Self {
            kind: err.kind,
            text: source[err.span].to_string(),
            input: source,
        }
    }
}

#[derive(Debug, Error)]
#[error("{kind} at {span:?}: {input:?}")]
pub struct ParserError<'s> {
    pub input: Parser<'s>,
    pub span: Range<usize>,
    pub kind: ErrorKind,
}

impl<'s> ParseError<Parser<'s>> for ParserError<'s> {
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
