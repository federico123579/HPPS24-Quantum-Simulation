use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use qcs_circuit_parser::parse_program;
use qcs_core::{
    contractions::{TensorKind, TensorNetwork},
    cpu_scheduler::CPUContractionPlan,
    executor::CpuExecutor,
    model::{blocks::Block, gates::QuantumGate, QRegister, QuantumCircuit, Qubit, TensorProduct},
};

macro_rules! test_circuit {
    ($name:ident, $filename:expr) => {
        #[test]
        fn $name() -> Result<()> {
            let circ = parse_program(circuit_dir()?.join($filename)).unwrap();
            check(&circ, &zero_register(circ.n_qubits))
                .context("Failed to check for zero register")?;
            check(&circ, &one_register(circ.n_qubits))
                .context("Failed to check for one register")?;
            Ok(())
        }
    };
    () => {};
}

test_circuit!(q2_00, "q2-00.txt");
test_circuit!(q2_01, "q2-01.qasm");
test_circuit!(q2_02, "q2-02.qasm");
test_circuit!(q3_00, "q3-00.txt");
test_circuit!(q3_01, "q3-01.txt");
test_circuit!(q3_02, "q3-02.txt");
test_circuit!(unlinked_disjointed_spans, "q3-03.qasm");
test_circuit!(q5_00, "q5-00.txt");
test_circuit!(q5_01, "q5-01.txt");
test_circuit!(full_adder, "full-adder.txt");
test_circuit!(full_adder_qasm, "full-adder.qasm");
test_circuit!(quantum_fourier_transform, "qft.qasm");

fn zero_register(n_qubits: usize) -> QRegister {
    QRegister::from((0..n_qubits).map(|_| Qubit::zero()))
}

fn one_register(n_qubits: usize) -> QRegister {
    QRegister::from((0..n_qubits).map(|_| Qubit::one()))
}

fn check(circuit: &QuantumCircuit, input_register: &QRegister) -> Result<()> {
    let base_eval = circuit.clone().eval();
    let contract_eval = contract(circuit).context("Failed to contract")?;

    let base_output = base_eval * input_register.to_owned();
    let contract_output = contract_eval * input_register.to_owned();

    println!("Base distribution: {}", base_output.distr());
    println!("Contracted distribution: {}", contract_output.distr());

    assert!((base_output.distr() - contract_output.distr()).norm() < 1e-6);
    Ok(())
}

fn contract(circuit: &QuantumCircuit) -> Result<Block> {
    let tensor_net = TensorNetwork::from(circuit.clone());
    let contracted_nodes = tensor_net.contract().into_iter();

    let mut blocks = Vec::new();

    for node in contracted_nodes {
        match node {
            TensorKind::Contraction(contr) => {
                let mut plan = CPUContractionPlan::from(*contr);
                let plan_clone = plan.clone();
                println!("{}", plan);

                let mut ready = plan.fetch_ready();
                while !ready.is_empty() {
                    println!("Ready instructions:");
                    for instr in ready.iter() {
                        println!("{}", &instr);
                    }

                    plan.set_done(ready.iter().map(|i| i.id));
                    println!("............................................");
                    ready = plan.fetch_ready();
                }

                let exec = CpuExecutor::new();
                let start = std::time::Instant::now();
                blocks.extend(exec.execute(plan_clone));
                println!("Time: {:?}", start.elapsed());
            }
            TensorKind::Gate(g) => blocks.push((*g).spanned_block()),
        }
    }

    let eval = blocks.into_iter().fold(None, |acc, block| match acc {
        None => Some(block),
        Some(acc) => Some(acc.tensor_product(block)),
    });

    eval.map(|sb| sb.into_block())
        .ok_or(anyhow::anyhow!("Nothing to contract"))
}

fn circuit_dir() -> Result<PathBuf> {
    Ok(Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("circuits"))
}
