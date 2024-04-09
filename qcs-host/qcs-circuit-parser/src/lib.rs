mod error;
mod parser;
mod tokens;

use nom::{multi::many0, Finish};
use qcs_core::{model::QuantumCircuit, utils::GateSpan};

pub use error::Error;
use parser::parse_gate;

pub fn parse_program(input: &str) -> Result<QuantumCircuit, Error> {
    // parse the input and retrieve the gates
    let input = parser::Parser::new(input);
    let (_, gates) = many0(parse_gate)(input).finish()?;

    let total_span = gates
        .iter()
        .fold(GateSpan::range(0..0), |acc, g| acc.full_join(g.span()));
    let mut circ = QuantumCircuit::new(total_span.span_len());
    gates.into_iter().for_each(|g| circ.push_gate(g));
    Ok(circ)
}
