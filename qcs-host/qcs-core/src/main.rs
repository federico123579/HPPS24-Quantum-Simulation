use either::Either;

use crate::{
    model::{QRegister, QuantumCircuit, Qubit},
    scheduler::ContractionPlan,
};

mod contractions;
mod model;
mod scheduler;

fn main() {
    // let inr = QRegister::from([Qubit::one(), Qubit::new(0.0.into(), (-1.0).into())]);
    // let mut circ = QuantumCircuit::new(2);
    // circ.g_x(1);
    // circ.g_h(0);
    // circ.g_h(1);
    // circ.g_cxd(0..2);
    // circ.g_h(0);
    // circ.g_h(1);

    let mut circ = QuantumCircuit::new(3);
    circ.g_x(0);
    circ.g_h(0);
    circ.g_h(1);
    circ.g_x(2);
    circ.g_cxu(0..2);
    circ.g_z(0);
    circ.g_swap(1..3);
    circ.g_h(1);
    circ.g_z(2);

    // let circ_eval = circ.clone().eval();
    let contr_graph = circ.into_contraction_graph();
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
