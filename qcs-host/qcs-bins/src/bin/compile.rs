use std::path::PathBuf;

use clap::Parser;
use qcs_circuit_parser::parse_program;
use qcs_core::{
    compiler::BinaryFile,
    contractions::{TensorKind, TensorNetwork},
    executor::{CpuExecutor, InstructionLike},
    model::{blocks::BlockLike, gates::QuantumGate, QRegister, Qubit, TensorProduct},
    op_tree,
    scheduler::OperationPlan,
};

#[derive(Debug, Clone, Parser)]
struct Cli {
    input: PathBuf,
    output: PathBuf,
}

fn compile_plan(mut plan: OperationPlan, output: PathBuf) {
    let mut bfile = BinaryFile::new(output).unwrap();
    while !plan.is_empty() {
        let instructions = plan.fetch_ready();
        let mut dones = Vec::new();
        for instruction in instructions {
            bfile.add_operation_instruction(&instruction).unwrap();
            dones.push(instruction.id());
        }
        plan.set_done(dones);
    }
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
                let plan = OperationPlan::from(opt);
                println!("Contraction plan:\n{}", &plan);
                compile_plan(plan.clone(), args.output.clone());
                println!("Compiled to binary file {}", args.output.display());
                let exec = CpuExecutor::new();
                let start = std::time::Instant::now();
                blocks.extend(exec.execute(plan));
                println!("CPU Execution Time: {:?}", start.elapsed());
            }
            TensorKind::Gate(g) => blocks.push((*g).block()),
        }
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
