use std::{
    path::{Path, PathBuf},
    time::Duration,
    vec::IntoIter,
};

use clap::Parser;
use qcs_circuit_parser::parse_program;
use qcs_core::{
    contractions::{TensorKind, TensorNetwork},
    cpu_scheduler::CPUContractionPlan,
    executor::CpuExecutor,
    model::{
        blocks::Block,
        gates::{Gate, QuantumGate},
        QRegister, Qubit, TensorProduct,
    },
};
use rusqlite::{Connection, Result};
use tracing::{debug, info, instrument, trace};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[derive(Debug, Clone, Parser)]
struct Cli {
    /// Input QASM files
    #[clap(required = true)]
    input: Vec<PathBuf>,

    /// Output SQLite database
    #[clap(short, long, default_value = "db.sqlite")]
    output: String,

    /// Skip inserting contractions into the database
    #[clap(short, long, default_value = "false")]
    skip_insert: bool,
}

const SCHEMA: &str = include_str!("../../../schema.sql");

fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    debug!("Starting export");

    let args = Cli::parse();
    let conn = Connection::open(args.output)?;
    debug!("Connected to database");

    conn.execute_batch(SCHEMA)?;

    for input in args.input {
        let _outer_span = tracing::info_span!("", ?input).entered();
        let program_id = conn.insert_program(&input)?;
        let circuit = parse_program(input).unwrap();

        let inputs: Vec<QRegister> = vec![
            vec![Qubit::zero(); circuit.n_qubits].into(),
            vec![Qubit::one(); circuit.n_qubits].into(),
            vec![
                Qubit::new((1.0 / 2.0_f64.sqrt()).into(), (1.0 / 2.0_f64.sqrt()).into());
                circuit.n_qubits
            ]
            .into(),
        ];

        let tensor_net = TensorNetwork::from(circuit.clone());
        debug!("Starting building tensor network");
        let start = std::time::Instant::now();
        let contracted_nodes: IntoIter<TensorKind> = tensor_net.contract().into_iter();
        let tree_building_time = start.elapsed();
        info!(
            time_us = tree_building_time.as_micros(),
            "Finished building tensor network"
        );

        if !args.skip_insert {
            debug!("Inserting contractions into database");
            let start = std::time::Instant::now();
            for node in contracted_nodes.clone() {
                conn.insert_contraction(node, program_id)?;
            }
            let insertion_time = start.elapsed();
            info!(
                time_us = insertion_time.as_micros(),
                "Finished inserting contractions"
            );
        }

        let mut blocks = Vec::new();
        let mut cpu_time = Duration::new(0, 0);
        for node in contracted_nodes {
            match node {
                TensorKind::Contraction(contr) => {
                    let plan = CPUContractionPlan::from(*contr);
                    let exec = CpuExecutor::new();
                    debug!("Starting contraction");
                    let start = std::time::Instant::now();
                    blocks.extend(exec.execute(plan));
                    cpu_time += start.elapsed();
                    info!(time_us = cpu_time.as_micros(), "Finished contraction");
                }
                TensorKind::Gate(g) => blocks.push((*g).spanned_block()),
            }
        }

        // update the program with the contraction time
        debug!("Updating program with contraction time");
        conn.update_contraction_time(
            program_id,
            cpu_time.as_micros() as u64,
            tree_building_time.as_micros() as u64,
        )?;

        debug!("Combining individual blocks into a single block");
        let eval = blocks.into_iter().fold(None, |acc, block| match acc {
            None => Some(block),
            Some(acc) => Some(acc.tensor_product(block)),
        });

        debug!("Starting measuring experiment time (input -> output)");
        if let Some(eval) = eval {
            let block = eval.into_block();
            let outputs = inputs
                .iter()
                .map(|input| &block * input.to_owned())
                .collect::<Vec<_>>();
            for (input, output) in inputs.into_iter().zip(outputs) {
                conn.insert_experiment(input, output, program_id)?;
            }
        }

        info!("Finished export");
    }

    Ok(())
}

trait IntoByte {
    fn into_bytes(self) -> Vec<u8>;
}

impl IntoByte for Block {
    fn into_bytes(self) -> Vec<u8> {
        self.into_matrix()
            .into_iter()
            .flat_map(|x| {
                let mut bytes = Vec::new();
                bytes.extend(x.re.to_le_bytes());
                bytes.extend(x.im.to_le_bytes());
                bytes
            })
            .collect::<Vec<u8>>()
    }
}

impl IntoByte for QRegister {
    fn into_bytes(self) -> Vec<u8> {
        self.qubits
            .into_iter()
            .flat_map(|x| {
                let mut bytes = Vec::new();
                bytes.extend(x.re.to_le_bytes());
                bytes.extend(x.im.to_le_bytes());
                bytes
            })
            .collect::<Vec<u8>>()
    }
}

trait QuantumDB {
    fn insert_gate(&self, gate: Gate) -> Result<i64>;
    fn insert_contraction(&self, contr: TensorKind, program_id: i64) -> Result<i64>;
    fn insert_program(&self, program: &Path) -> Result<i64>;
    fn update_contraction_time(
        &self,
        id: i64,
        contraction_time_ms: u64,
        tree_building_time_ms: u64,
    ) -> Result<()>;
    fn insert_experiment(
        &self,
        input: QRegister,
        output: QRegister,
        program_id: i64,
    ) -> Result<i64>;
}

impl QuantumDB for Connection {
    #[instrument(skip(self), level = "debug", fields(gate = %gate.to_string(), rank = gate.rank()))]
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
        let data = gate.block().into_bytes();
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
                trace!(program_id, "Inserted contraction");
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
                trace!(program_id, "Inserted contraction");
                Ok(id)
            }
        }
    }

    #[instrument(skip(self), level = "debug")]
    fn insert_program(&self, program: &Path) -> Result<i64> {
        let filename = program.file_name().unwrap().to_str().unwrap();
        let text = std::fs::read_to_string(program).unwrap();
        self.execute(
            "INSERT INTO programs (filename, text) VALUES (?1, ?2)",
            (&filename, &text),
        )?;
        trace!("Inserted program");
        let id = self.last_insert_rowid();
        Ok(id)
    }

    #[instrument(skip(self), level = "debug")]
    fn update_contraction_time(
        &self,
        id: i64,
        contraction_time_us: u64,
        tree_building_time_us: u64,
    ) -> Result<()> {
        self.execute(
            "UPDATE programs SET contraction_cpu_time_us = ?1, tree_building_time_us = ?2 WHERE id = ?3",
            (&contraction_time_us, &tree_building_time_us, &id),
        )?;
        trace!("Updated program with contraction time");
        Ok(())
    }

    #[instrument(skip(self), level = "debug", fields(input = %input.qubits.len(), output = %output.qubits.len()))]
    fn insert_experiment(
        &self,
        input: QRegister,
        output: QRegister,
        program_id: i64,
    ) -> Result<i64> {
        let input = input.into_bytes();
        let output = output.into_bytes();
        self.execute(
            "INSERT INTO experiments (program_id, input_vector, output_vector) VALUES (?1, ?2, ?3)",
            (&program_id, &input, &output),
        )?;
        trace!("Inserted experiment");
        let id = self.last_insert_rowid();
        Ok(id)
    }
}
