use crate::model::{QRegister, QuantumCircuit, Qubit};

mod contractions;
mod model;
mod tree;

fn main() {
    let inr = QRegister::from([Qubit::one(), Qubit::new(0.0.into(), (-1.0).into())]);
    let mut circ = QuantumCircuit::new(2);
    circ.g_x(1);
    circ.g_h(0);
    circ.g_h(1);
    circ.g_cxd(0..2);
    circ.g_h(0);
    circ.g_h(1);

    let circ_eval = circ.clone().eval();
    let contr_graph = circ.into_contraction_graph();
    println!("{}", contr_graph);

    let qstate = circ_eval * inr;
    // println!("{}", qstate.distr().map(|v| (v * 1e2).round()));
}
