use std::path::PathBuf;

use clap::Parser;
use qcs_circuit_parser::parse_program;
use qcs_core::{scheduler::ContractionPlan, Either};

#[derive(Debug, Clone, Parser)]
struct Cli {
    input: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let input_txt = std::fs::read_to_string(args.input).unwrap();
    let circuit = parse_program(&input_txt).unwrap();

    // let inr = QRegister::from([Qubit::one(), Qubit::new(0.0.into(), (-1.0).into())]);
    // let circ_eval = circ.clone().eval();
    let contr_graph = circuit.into_contraction_graph();
    let contracted_nodes = contr_graph.contract();

    for i in contracted_nodes {
        if let Either::Left(contr) = i.0 {
            let mut plan = ContractionPlan::from(*contr);
            println!("{}", plan);

            let mut ready = plan.fetch_ready();
            while !ready.is_empty() {
                println!("Ready instructions:");
                for instr in ready.iter() {
                    println!("{}", instr);
                }

                plan.set_done(ready.iter().map(|i| i.id));
                println!("............................................");
                ready = plan.fetch_ready();
            }
        }
    }

    // let qstate = circ_eval * inr;
    // println!("{}", qstate.distr().map(|v| (v * 1e2).round()));
}
