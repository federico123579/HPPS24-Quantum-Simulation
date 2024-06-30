use std::{path::PathBuf, time::Duration};

use clap::Parser;
use qcs_circuit_parser::parse_program;
use qcs_core::{
    contractions::{TensorKind, TensorNetwork},
    executor::CpuExecutor,
    model::{gates::QuantumGate, TensorProduct},
    scheduler::ContractionPlan,
};

#[derive(Debug, Clone, Parser)]
struct Cli {
    input: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let circuit = parse_program(args.input.clone()).unwrap();

    // if results.csv does not exist, create it and add the header
    if !std::path::Path::new("results.csv").exists() {
        let mut csv = csv::Writer::from_path("results.csv").unwrap();
        csv.write_record([
            "circuit",
            "contraction_ns",
            "contracted_exe_ns",
            "normal_exe_ns",
        ])
        .unwrap();
    }

    // open the file in append mode
    let csv_file = std::fs::File::options()
        .append(true)
        .open("results.csv")
        .unwrap();
    let mut csv = csv::WriterBuilder::new().from_writer(std::io::BufWriter::new(csv_file));

    let tensor_net = TensorNetwork::from(circuit.clone());
    let start = std::time::Instant::now();
    let contracted_nodes = tensor_net.contract().into_iter();
    let contracted_elapsed = start.elapsed();

    let mut blocks = Vec::new();

    let mut contracted_exe_elapsed = Duration::new(0, 0);
    for node in contracted_nodes {
        match node {
            TensorKind::Contraction(contr) => {
                let plan = ContractionPlan::from(*contr);
                let exec = CpuExecutor::new();
                let start = std::time::Instant::now();
                blocks.extend(exec.execute(plan));
                contracted_exe_elapsed += start.elapsed();
            }
            TensorKind::Gate(g) => blocks.push((*g).spanned_block()),
        }
    }

    let start = std::time::Instant::now();
    let _ = blocks.into_iter().fold(None, |acc, block| match acc {
        None => Some(block),
        Some(acc) => Some(acc.tensor_product(block)),
    });
    contracted_exe_elapsed += start.elapsed();

    let start = std::time::Instant::now();
    let _ = circuit.clone().eval();
    let normal_elapsed = start.elapsed();

    csv.write_record(&[
        args.input.to_string_lossy().to_string(),
        contracted_elapsed.as_nanos().to_string(),
        contracted_exe_elapsed.as_nanos().to_string(),
        normal_elapsed.as_nanos().to_string(),
    ])
    .unwrap();

    println!("{} computed", args.input.to_string_lossy());
}
