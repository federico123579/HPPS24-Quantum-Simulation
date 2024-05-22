use std::path::{Path, PathBuf};

use clap::Parser;
use qcs_circuit_parser::parse_program;
use qcs_core::{
    contractions::{TensorKind, TensorNetwork},
    model::gates::{Gate, QuantumGate},
};
use rusqlite::{Connection, Result};

#[derive(Debug, Clone, Parser)]
struct Cli {
    input: Vec<PathBuf>,
}

const SCHEMA: &str = include_str!("../../../schema.sql");

fn main() -> Result<()> {
    let args = Cli::parse();
    let conn = Connection::open("db.sqlite")?;

    conn.execute_batch(SCHEMA)?;

    for input in args.input {
        let program_id = conn.insert_program(&input)?;
        let circuit = parse_program(input).unwrap();
        let tensor_net = TensorNetwork::from(circuit.clone());
        let contracted_nodes: std::vec::IntoIter<TensorKind> = tensor_net.contract().into_iter();
        for node in contracted_nodes {
            conn.insert_contraction(node, program_id)?;
        }
    }

    Ok(())
}

trait QuantumDB {
    fn insert_gate(&self, gate: Gate) -> Result<i64>;
    fn insert_contraction(&self, contr: TensorKind, program_id: i64) -> Result<i64>;
    fn insert_program(&self, program: &Path) -> Result<i64>;
}

impl QuantumDB for Connection {
    fn insert_gate(&self, gate: Gate) -> Result<i64> {
        let name = gate.to_string();
        // check first if the gate is already in the database
        let mut stmt = self.prepare("SELECT id FROM gates WHERE name = ?1")?;
        let mut rows = stmt.query_map([&name], |row| row.get::<usize, i64>(0))?;
        if let Some(id) = rows.next() {
            return id;
        }
        // if not, insert the gate
        let rank = gate.rank();
        // from block to matrix to vector of f64 to vector of bytes
        let data = gate
            .block()
            .into_matrix()
            .into_iter()
            .flat_map(|x| {
                let mut bytes = Vec::new();
                bytes.extend(x.re.to_le_bytes());
                bytes.extend(x.im.to_le_bytes());
                bytes
            })
            .collect::<Vec<u8>>();
        self.execute(
            "INSERT INTO gates (name, rank, data) VALUES (?1, ?2, ?3)",
            (&name, &rank, &data),
        )?;
        let id = self.last_insert_rowid();
        Ok(id)
    }

    fn insert_contraction(&self, contr: TensorKind, program_id: i64) -> Result<i64> {
        match contr {
            TensorKind::Gate(gate) => {
                let span = gate.span();
                let gate_id = self.insert_gate(*gate)?;
                self.execute(
                    "INSERT INTO contractions (program_id, span, kind, gate_id) VALUES (?1, ?2, ?3, ?4)",
                    (&program_id, &span.to_string(), "G", &gate_id),
                )?;
                let id = self.last_insert_rowid();
                Ok(id)
            }
            TensorKind::Contraction(contr) => {
                let left_id = self.insert_contraction(contr.lhs, program_id)?;
                let right_id = self.insert_contraction(contr.rhs, program_id)?;
                let span = contr.span;
                self.execute(
                    "INSERT INTO contractions (program_id, span, kind, left_id, right_id) VALUES (?1, ?2, ?3, ?4, ?5)",
                    (&program_id, &span.to_string(), "C", &left_id, &right_id),
                )?;
                let id = self.last_insert_rowid();
                Ok(id)
            }
        }
    }

    fn insert_program(&self, program: &Path) -> Result<i64> {
        let filename = program.file_name().unwrap().to_str().unwrap();
        let text = std::fs::read_to_string(program).unwrap();
        self.execute(
            "INSERT INTO programs (filename, text) VALUES (?1, ?2)",
            (&filename, &text),
        )?;
        let id = self.last_insert_rowid();
        Ok(id)
    }
}
