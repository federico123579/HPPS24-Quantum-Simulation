use std::ops::{Deref, DerefMut};

use logos::{Lexer, Logos};
use nom::InputLength;
use qcs_core::model::gates::*;

use crate::{
    error::{Error, ErrorKind},
    tokens::Token,
};

#[derive(Debug, Clone)]
pub struct Parser<'s>(Lexer<'s, Token>);

impl<'s> Deref for Parser<'s> {
    type Target = Lexer<'s, Token>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'s> DerefMut for Parser<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'s> Parser<'s> {
    pub fn new(input: &'s str) -> Self {
        Self(Token::lexer(input))
    }
}

impl InputLength for Parser<'_> {
    fn input_len(&self) -> usize {
        self.0.remainder().len()
    }
}

pub type IResult<'s, T> = nom::IResult<Parser<'s>, T, Error<'s>>;

pub fn parse_gate(input: Parser<'_>) -> IResult<Gate> {
    let (input, token) = expect_next(input)?;
    let (input, kind) = match token {
        Token::GateX => {
            let (input, lanes) = parse_lanes(input)?;
            (input, Gate::PauliX(PauliX::new(lanes[0])))
        }
        Token::GateY => {
            let (input, lanes) = parse_lanes(input)?;
            (input, Gate::PauliY(PauliY::new(lanes[0])))
        }
        Token::GateZ => {
            let (input, lanes) = parse_lanes(input)?;
            (input, Gate::PauliZ(PauliZ::new(lanes[0])))
        }
        Token::GateH => {
            let (input, lanes) = parse_lanes(input)?;
            (input, Gate::Hadamard(Hadamard::new(lanes[0])))
        }
        Token::GateP(p) => {
            let (input, lanes) = parse_lanes(input)?;
            (input, Gate::Phase(Phase::new(p, lanes[0])))
        }
        Token::GateCX => {
            let (input, lanes) = parse_lanes(input)?;
            (input, Gate::CX(CX::new(lanes[0], lanes[1])))
        }
        Token::GateCY => {
            let (input, lanes) = parse_lanes(input)?;
            (input, Gate::CY(CY::new(lanes[0], lanes[1])))
        }
        Token::GateCZ => {
            let (input, lanes) = parse_lanes(input)?;
            (input, Gate::CZ(CZ::new(lanes[0], lanes[1])))
        }
        Token::GateSWAP => {
            let (input, lanes) = parse_lanes(input)?;
            (input, Gate::Swap(Swap::new(lanes[0], lanes[1])))
        }
        Token::GateTOFF => {
            let (input, lanes) = parse_lanes(input)?;
            (
                input,
                Gate::Toffoli(Toffoli::new((lanes[0], lanes[1]), lanes[2])),
            )
        }
        _ => return Err(unexpected(input)),
    };
    Ok((input, kind))
}

fn parse_lanes(input: Parser<'_>) -> IResult<Vec<usize>> {
    let (input, token) = expect_next(input)?;
    let lanes = match token {
        Token::Lanes(lanes) => lanes,
        _ => return Err(unexpected(input)),
    };
    Ok((input, lanes))
}

fn expect_next(mut input: Parser<'_>) -> IResult<Token> {
    match input.next() {
        Some(Ok(token)) => Ok((input, token)),
        Some(Err(_)) => Err(nom::Err::Failure(Error {
            span: input.span(),
            input,
            kind: ErrorKind::InvalidToken,
        })),
        None => Err(nom::Err::Error(Error {
            span: input.span(),
            input,
            kind: ErrorKind::UnexpectedEoF,
        })),
    }
}

#[inline]
fn unexpected(input: Parser<'_>) -> nom::Err<Error> {
    nom::Err::Failure(Error {
        span: input.span(),
        input,
        kind: ErrorKind::UnexpectedToken,
    })
}
