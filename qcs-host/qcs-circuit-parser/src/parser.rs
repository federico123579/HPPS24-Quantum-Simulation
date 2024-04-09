use std::ops::{Deref, DerefMut};

use logos::{Lexer, Logos};
use nom::InputLength;
use qcs_core::{model::gates::*, utils::GateSpan};

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

pub fn parse_gate(input: Parser<'_>) -> IResult<CircuitGate> {
    let (input, gate) = parse_gate_kind(input)?;
    let (input, span) = if gate.is_rank_one() {
        parse_lane(input)?
    } else {
        parse_lanes_range(input)?
    };
    Ok((input, CircuitGate { kind: gate, span }))
}

fn parse_gate_kind(input: Parser<'_>) -> IResult<GateKind> {
    let (input, token) = expect_next(input)?;
    let kind = match token {
        Token::GateX => GateKind::PauliX(PauliX),
        Token::GateY => GateKind::PauliY(PauliY),
        Token::GateZ => GateKind::PauliZ(PauliZ),
        Token::GateH => GateKind::Hadamard(Hadamard),
        Token::GateS => GateKind::Phase(Phase),
        Token::GateT => GateKind::Pi8(Pi8),
        Token::GateCNOT => GateKind::CNOTup(CNOTup),
        Token::GateCZED => GateKind::ConZ(ConZ),
        Token::GateSWAP => GateKind::Swap(Swap),
        Token::GateTOFF => GateKind::Toffoli(Toffoli),
        _ => return Err(unexpected(input)),
    };
    Ok((input, kind))
}

fn parse_lane(input: Parser<'_>) -> IResult<GateSpan> {
    let (input, token) = expect_next(input)?;
    let lane = match token {
        Token::Lane(lane) => lane,
        _ => return Err(unexpected(input)),
    };
    Ok((input, GateSpan::single(lane)))
}

fn parse_lanes_range(input: Parser<'_>) -> IResult<GateSpan> {
    let (input, token) = expect_next(input)?;
    let range = match token {
        Token::LanesRange(range) => range,
        _ => return Err(unexpected(input)),
    };
    Ok((input, GateSpan::from(range)))
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
