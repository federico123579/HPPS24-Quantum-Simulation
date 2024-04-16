use std::path::PathBuf;

use clap::Parser;
use qcs_circuit_parser::parse_program;
use qcs_core::{
    contractions::{TensorKind, TensorNetwork},
    executor::CpuExecutor,
    model::{QRegister, Qubit, TensorProduct},
    scheduler::ContractionPlan,
};

#[derive(Debug, Clone, Parser)]
struct Cli {
    input: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let circuit = parse_program(args.input).unwrap();

    let tensor_net = TensorNetwork::from(circuit.clone());
    let mut contracted_nodes = tensor_net.contract().into_iter();

    let mut blocks = Vec::new();

    while let Some(TensorKind::Contraction(contr)) = contracted_nodes.next() {
        let plan = ContractionPlan::from(*contr);
        println!("{}", &plan);
        let exec = CpuExecutor::new();
        let start = std::time::Instant::now();
        blocks.extend(exec.execute(plan));
        println!("Time: {:?}", start.elapsed());
    }

    let eval = blocks.into_iter().fold(None, |acc, block| match acc {
        None => Some(block),
        Some(acc) => Some(acc.tensor_product(block)),
    });

    let inr = QRegister::from((0..circuit.n_qubits).map(|_| Qubit::zero()));
    if let Some(eval) = eval {
        let qstate_2 = eval.into_block() * inr;
        println!("{}", qstate_2.distr());
    }
}
