pub mod error;
mod parser;
mod tokens;

use std::path::Path;

use nom::{multi::many0, Finish};
use oq3_semantics::{
    asg::{Expr, GateOperand, IndexOperator, Literal, Stmt},
    symbols::SymbolType,
    syntax_to_semantics::parse_source_file,
    types::{ArrayDims, Type},
};
use qcs_core::model::{gates::QuantumGate, span::Span, QuantumCircuit};

use error::{Error, OwnedParserError};
use parser::parse_gate;

pub fn parse_program(filepath: impl AsRef<Path>) -> Result<QuantumCircuit, Error> {
    let path = filepath.as_ref();
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("qasm") => Ok(parse_qasm_program(path)?),
        Some("txt") => Ok(parse_textual_program(path)?),
        _ => Err(Error::UnsupportedFileExtension),
    }
}

fn parse_qasm_program(filename: impl AsRef<Path>) -> Result<QuantumCircuit, OwnedParserError> {
    let source_file = parse_source_file(filename.as_ref(), None::<&[&Path]>);
    let symbols = source_file.symbol_table();
    source_file.program().print_asg_debug_pretty();
    // dbg!(symbols);

    // Find the number of qubits in the program
    let mut n_qubits = None;
    for stm in source_file.program().stmts() {
        if let Stmt::DeclareQuantum(q) = stm {
            let sym = q.name().as_ref().unwrap();
            if let Type::QubitArray(ArrayDims::D1(d)) = symbols[sym].symbol_type() {
                n_qubits = Some(*d);
            }
        }
    }
    let n_qubits = n_qubits.unwrap();

    let mut circ = QuantumCircuit::new(n_qubits);

    // from now on we consider q as the quantum register of dimension n_qubits
    for stm in source_file.program().stmts() {
        if let Stmt::GateCall(g) = stm {
            let gate = &symbols[g.name().as_ref().unwrap()];
            let gate_name = gate.name();
            // let Type::Gate(_, qargs) = gate.symbol_type() else {
            //     continue;
            // };

            let params = g
                .params()
                .unwrap_or_default()
                .iter()
                .map(|p| {
                    if let Expr::Literal(Literal::Float(f)) = p.expression() {
                        f.value().parse().unwrap()
                    } else {
                        0.0
                    }
                })
                .collect::<Vec<_>>();

            let lanes = g
                .qubits()
                .iter()
                .filter_map(|e| {
                    let Expr::GateOperand(GateOperand::IndexedIdentifier(id)) = e.expression()
                    else {
                        return None;
                    };
                    let IndexOperator::ExpressionList(el) = id.indexes().first()? else {
                        return None;
                    };
                    let Expr::Literal(Literal::Int(i)) = el.expressions[0].expression() else {
                        return None;
                    };
                    Some(*i.value() as usize)
                })
                .collect::<Vec<_>>();

            match gate_name {
                "x" => circ.g_x(lanes[0]),
                "y" => circ.g_y(lanes[0]),
                "z" => circ.g_z(lanes[0]),
                "h" => circ.g_h(lanes[0]),
                "p" => circ.g_p(params[0], lanes[0]),
                "s" => circ.g_s(lanes[0]),
                "t" => circ.g_t(lanes[0]),
                "id" => circ.g_id(lanes[0]),
                "CX" | "cx" => circ.g_cx(lanes[0], lanes[1]),
                "cy" => circ.g_cy(lanes[0], lanes[1]),
                "cz" => circ.g_cz(lanes[0], lanes[1]),
                "swap" => circ.g_swap(lanes[0], lanes[1]),
                "ccx" => circ.g_toff(lanes[0], lanes[1], lanes[2]),
                _ => (),
            }
        }
    }

    // println!("circ: {:?}", circ);
    Ok(circ)
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
