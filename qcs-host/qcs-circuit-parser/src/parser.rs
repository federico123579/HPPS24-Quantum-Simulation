use std::ops::{Deref, DerefMut, Range};

use logos::{Lexer, Logos};
use nom::InputLength;
use qcs_core::model::{gates::*, GateOnLanes};

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

pub fn parse_gate(input: Parser<'_>) -> IResult<GateOnLanes> {
    let (input, gate) = parse_gate_kind(input)?;
    let (input, lanes) = match gate {
        Gate::PauliX(_)
        | Gate::PauliY(_)
        | Gate::PauliZ(_)
        | Gate::Hadamard(_)
        | Gate::Phase(_)
        | Gate::Pi8(_) => {
            let (input, lane) = parse_lane(input)?;
            (input, lane..lane + 1)
        }
        Gate::CNOTup(_) | Gate::ConZ(_) | Gate::Swap(_) | Gate::Toffoli(_) => {
            let (input, range) = parse_lanes_range(input)?;
            (input, range)
        }
        _ => return Err(unexpected(input)),
    };
    Ok((input, GateOnLanes { gate, lanes }))
}

fn parse_gate_kind(input: Parser<'_>) -> IResult<Gate> {
    let (input, token) = expect_next(input)?;
    let kind = match token {
        Token::GateX => Gate::PauliX(PauliX),
        Token::GateY => Gate::PauliY(PauliY),
        Token::GateZ => Gate::PauliZ(PauliZ),
        Token::GateH => Gate::Hadamard(Hadamard),
        Token::GateS => Gate::Phase(Phase),
        Token::GateT => Gate::Pi8(Pi8),
        Token::GateCNOT => Gate::CNOTup(CNOTup),
        Token::GateCZED => Gate::ConZ(ConZ),
        Token::GateSWAP => Gate::Swap(Swap),
        Token::GateTOFF => Gate::Toffoli(Toffoli),
        _ => return Err(unexpected(input)),
    };
    Ok((input, kind))
}

fn parse_lane(input: Parser<'_>) -> IResult<usize> {
    let (input, token) = expect_next(input)?;
    let lane = match token {
        Token::Lane(lane) => lane,
        _ => return Err(unexpected(input)),
    };
    Ok((input, lane))
}

fn parse_lanes_range(input: Parser<'_>) -> IResult<Range<usize>> {
    let (input, token) = expect_next(input)?;
    let range = match token {
        Token::LanesRange(range) => range,
        _ => return Err(unexpected(input)),
    };
    Ok((input, range))
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
