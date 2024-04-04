use nalgebra::Complex;

use crate::model::{Block, KnownGates, QRegister, Qubit};

mod dag;
mod model;

fn main() {
    let a_block = Block::from(KnownGates::PauliX);
    let b_block = Block::from(KnownGates::Identity);
    let c_block = a_block.tensor_product(&b_block);
    println!("Block A: {}", a_block);
    println!("Block B: {}", b_block);
    println!("Block C: {}", c_block);

    let phi = Qubit::new(Complex::i(), 0.0.into());
    let psi = Qubit::new(0.0.into(), (-1.0).into());
    let a_reg = QRegister::from([phi.clone()]);
    let b_reg = QRegister::from([psi.clone()]);
    let c_reg = QRegister::from([phi, psi]);
    println!("Initial A: {}", a_reg);
    println!("Initial B: {}", b_reg);
    println!("Initial C: {}", c_reg);

    let a_reg = a_block * a_reg;
    let b_reg = b_block * b_reg;
    let c_reg = c_block * c_reg;
    println!("A After gate: {}", a_reg);
    println!("B After gate: {}", b_reg);
    println!("C After gate: {}", c_reg);
}
