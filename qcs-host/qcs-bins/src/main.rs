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

    let mut instructions = Vec::new();

    while let Some(TensorKind::Contraction(contr)) = contracted_nodes.next() {
        let mut plan = ContractionPlan::from(*contr);
        println!("{}", plan);

        let mut ready = plan.fetch_ready();
        while !ready.is_empty() {
            println!("Ready instructions:");
            for instr in ready.iter() {
                println!("{}", &instr);
            }

            plan.set_done(ready.iter().map(|i| i.id));
            instructions.extend(ready);
            println!("............................................");
            ready = plan.fetch_ready();
        }
    }

    let inr = QRegister::from((0..circuit.n_qubits).map(|_| Qubit::zero()));
    let start = std::time::Instant::now();
    let circ_eval = circuit.clone().eval();
    let qstate_1 = circ_eval * inr.clone();
    // println!("{}", qstate.distr().map(|v| (v * 1e2).round()));
    println!("Time: {:?}", start.elapsed());
    println!("{}", qstate_1.distr());

    let mut exec = CpuExecutor::new();
    let start = std::time::Instant::now();
    let blocks = exec.execute(instructions);

    let eval = blocks.into_iter().fold(None, |acc, block| match acc {
        None => Some(block),
        Some(acc) => Some(acc.tensor_product(block)),
    });

    if let Some(eval) = eval {
        let qstate_2 = eval.into_block() * inr;
        println!("Time: {:?}", start.elapsed());
        println!("{}", qstate_2.distr());
        assert!((qstate_1.distr() - qstate_2.distr()).norm() < 1e-6);
    }
}
