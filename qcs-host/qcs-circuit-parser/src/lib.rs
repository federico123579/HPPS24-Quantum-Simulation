mod error;
mod parser;
mod tokens;

use nom::{multi::many0, Finish};
use qcs_core::model::QuantumCircuit;

pub use error::Error;
use parser::parse_gate;

pub fn parse_program(input: &str) -> Result<QuantumCircuit, Error> {
    // parse the input and retrieve the gates
    let input = parser::Parser::new(input);
    let (_, gates) = many0(parse_gate)(input).finish()?;

    let lanes_num = gates.iter().fold(0, |acc, g| g.lanes().end.max(acc));
    let mut circ = QuantumCircuit::new(lanes_num);
    gates.into_iter().for_each(|g| circ.push_gate(g));
    Ok(circ)
}
