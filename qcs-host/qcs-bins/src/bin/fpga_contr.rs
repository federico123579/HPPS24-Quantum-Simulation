use std::path::PathBuf;

use clap::Parser;
use qcs_circuit_parser::parse_program;
use qcs_core::{
    contractions::{TensorKind, TensorNetwork},
    fpga_scheduler::FPGAContractionPlan,
    model::gates::QuantumGate,
    op_tree,
};

#[derive(Debug, Clone, Parser)]
struct Cli {
    input: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let circuit = parse_program(args.input).unwrap();

    let tensor_net = TensorNetwork::from(circuit.clone());
    println!("Tensor Network:\n{}", tensor_net);
    let contracted_nodes = tensor_net.contract().into_iter();

    let mut blocks = Vec::new();

    for node in contracted_nodes {
        match node {
            TensorKind::Contraction(contr) => {
                let opt = op_tree::Operation::from_contraction(*contr, false);
                let plan = FPGAContractionPlan::from(opt);
                println!("Contraction plan:\n{}", &plan);
            }
            TensorKind::Gate(g) => blocks.push((*g).spanned_block()),
        }
    }
}
