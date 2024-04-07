use either::Either;

use crate::{
    model::{QRegister, QuantumCircuit, Qubit},
    scheduler::ContractionPlan,
};

mod contractions;
mod model;
mod scheduler;
mod tree;

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
    let mut contr_graph = circ.into_contraction_graph();
    contr_graph.contract();
    println!("{}", contr_graph);

    for i in contr_graph.items() {
        if let Either::Left(contr) = i.0 {
            let plan = ContractionPlan::from(*contr);
            println!("{}", plan);
        }
    }

    // let qstate = circ_eval * inr;
    // println!("{}", qstate.distr().map(|v| (v * 1e2).round()));
}
