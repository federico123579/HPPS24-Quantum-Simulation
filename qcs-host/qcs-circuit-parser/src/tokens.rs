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

    #[regex("P\\([0-9]+.?[0-9]*\\)", parse_phase)]
    GateP(f64),

    #[token("CX")]
    GateCX,

    #[token("CY")]
    GateCY,

    #[token("CZ")]
    GateCZ,

    #[token("SWAP")]
    GateSWAP,

    #[token("CCX")]
    GateTOFF,

    #[regex("\\[[0-9]+ *(, *[0-9]+)*\\]", parse_lanes)]
    Lanes(Vec<usize>),
}

/// Parse phase from P(0.5) to 0.5
fn parse_phase(s: &mut logos::Lexer<'_, Token>) -> f64 {
    s.slice()[2..s.slice().len() - 1].parse().unwrap()
}

/// Parse from [2,1] to vec![2, 1]
fn parse_lanes(s: &mut logos::Lexer<'_, Token>) -> Vec<usize> {
    s.slice()[1..s.slice().len() - 1]
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}
