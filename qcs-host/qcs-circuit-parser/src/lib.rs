pub mod error;
mod openqasm;
mod parser;
mod tokens;

use std::path::Path;

use nom::{multi::many0, Finish};
use qcs_core::model::{gates::QuantumGate, span::Span, QuantumCircuit};

use error::{Error, OwnedParserError};
use openqasm::parse_qasm_program;
use parser::parse_gate;

pub fn parse_program(filepath: impl AsRef<Path>) -> Result<QuantumCircuit, Error> {
    let path = filepath.as_ref();
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("qasm") => Ok(parse_qasm_program(path)?),
        Some("txt") => Ok(parse_textual_program(path)?),
        _ => Err(Error::UnsupportedFileExtension),
    }
}

fn parse_textual_program(filename: impl AsRef<Path>) -> Result<QuantumCircuit, OwnedParserError> {
    let input = std::fs::read_to_string(filename).unwrap();
    // parse the input and retrieve the gates
    let input = parser::Parser::new(&input);
    let (_, gates) = many0(parse_gate)(input)
        .finish()
        .map_err(Into::<OwnedParserError>::into)?;

    let total_span = gates
        .iter()
        .fold(Span::range(0..0), |acc, g| acc.full_join(&g.span()));
    let mut circ = QuantumCircuit::new(total_span.span_len());
    gates.into_iter().for_each(|g| circ.push_gate(g));
    Ok(circ)
}
