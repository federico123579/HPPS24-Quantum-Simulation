use crate::model::{Block, GateKind, QRegister, Qubit};

mod contractions;
mod model;
mod tree;

fn main() {
    let inr = QRegister::from([Qubit::one(), Qubit::new(0.0.into(), (-1.0).into())]);

    let i_block = Block::from(GateKind::Identity);
    let x_block = Block::from(GateKind::PauliX);
    let h_block = Block::from(GateKind::Hadamard);
    let cnot_block = Block::from(GateKind::CNOTup);

    let t1 = i_block.tensor_product(&x_block);
    let t2 = h_block.tensor_product(&h_block);
    let t3 = cnot_block;
    let t4 = h_block.tensor_product(&i_block);

    let qstate = t1 * t2 * t3 * t4 * inr;

    println!("{}", qstate);
}
