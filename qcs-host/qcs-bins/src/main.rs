use std::path::PathBuf;

use clap::Parser;
use qcs_circuit_parser::parse_program;
use qcs_core::{
    executor::CpuExecutor,
    model::{QRegister, Qubit, TensorProduct},
    representations::tensor_networks::{TensorKind, TensorNetwork},
    scheduler::ContractionPlan,
};

#[derive(Debug, Clone, Parser)]
struct Cli {
    input: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let input_txt = std::fs::read_to_string(args.input).unwrap();
    let circuit = parse_program(&input_txt).unwrap();

    let tensor_net = TensorNetwork::from(circuit);
    println!("{}", tensor_net);
    println!("{:?}", tensor_net.contractable());

    // let contr_graph = circuit.clone().into_contraction_graph();
    // let mut contracted_nodes = contr_graph.contract().into_iter();

    // let mut instructions = Vec::new();

    // while let Some(TensorKind::Contraction(contr)) = contracted_nodes.next() {
    //     let mut plan = ContractionPlan::from(*contr);
    //     println!("{}", plan);

    //     let mut ready = plan.fetch_ready();
    //     while !ready.is_empty() {
    //         println!("Ready instructions:");
    //         for instr in ready.iter() {
    //             println!("{}", &instr);
    //         }

    //         plan.set_done(ready.iter().map(|i| i.id));
    //         instructions.extend(ready);
    //         println!("............................................");
    //         ready = plan.fetch_ready();
    //     }
    // }

    // let inr = QRegister::from((0..circuit.n_qubits).map(|_| Qubit::zero()));
    // let circ_eval = circuit.clone().eval();
    // let qstate = circ_eval * inr.clone();
    // // println!("{}", qstate.distr().map(|v| (v * 1e2).round()));
    // println!("{}", qstate.distr());

    // let mut exec = CpuExecutor::new();
    // let blocks = exec.execute(instructions);

    // let eval = blocks.into_iter().fold(None, |acc, block| match acc {
    //     None => Some(block),
    //     Some(acc) => Some(acc.tensor_product(block)),
    // });

    // if let Some(eval) = eval {
    //     let qstate = eval.into_block() * inr;
    //     println!("{}", qstate.distr());
    // }
}
