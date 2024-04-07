use std::ops::Range;

use logos::Logos;

#[derive(Debug, Clone, PartialEq, Logos)]
#[logos(skip r"[ \t\n\f|]+")]
pub enum Token {
    #[token("X")]
    GateX,

    #[token("Y")]
    GateY,

    #[token("Z")]
    GateZ,

    #[token("H")]
    GateH,

    #[token("S")]
    GateS,

    #[token("T")]
    GateT,

    #[token("CNOT")]
    GateCNOT,

    #[token("CZED")]
    GateCZED,

    #[token("SWAP")]
    GateSWAP,

    #[token("TOFF")]
    GateTOFF,

    #[regex("\\[[0-9]+\\]", parse_lane)]
    Lane(usize),

    #[regex("\\[[0-9]+:[0-9]+\\]", parse_lanes_range)]
    LanesRange(Range<usize>),
}

/// Parse from [0] to 0
fn parse_lane(s: &mut logos::Lexer<'_, Token>) -> usize {
    s.slice()[1..s.slice().len() - 1].parse().unwrap()
}

/// Parse from [0:1] to 0..2
fn parse_lanes_range(s: &mut logos::Lexer<'_, Token>) -> Range<usize> {
    let slice = s.slice();
    let mut split = slice[1..slice.len() - 1].split(':');
    let start = split.next().unwrap().parse().unwrap();
    let end: usize = split.next().unwrap().parse().unwrap();
    start..end + 1
}
