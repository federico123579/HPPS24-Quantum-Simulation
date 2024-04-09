pub mod gates;

use std::ops::{Mul, Range};

use nalgebra::{Complex, DMatrix, DVector, Vector2};

use crate::{
    model::gates::{
        CNOTdown, CNOTup, ConZ, Hadamard, Identity, PauliX, PauliY, PauliZ, Phase, Pi8, Swap,
        Toffoli,
    },
    representations::contraction_graph::ContractionGraph,
    utils::GateSpan,
};

use self::gates::{CircuitGate, QuantumGate};

// @@@@@@@@@@@@
// @@ Qubits @@
// @@@@@@@@@@@@

#[derive(Debug, Clone, PartialEq)]
pub struct Qubit {
    amplitudes: Vector2<Complex<f64>>,
}

impl Qubit {
    pub fn new(alpha: Complex<f64>, beta: Complex<f64>) -> Self {
        Qubit {
            amplitudes: Vector2::new(alpha, beta),
        }
    }

    pub fn zero() -> Self {
        Qubit {
            amplitudes: Vector2::new(Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)),
        }
    }

    pub fn one() -> Self {
        Qubit {
            amplitudes: Vector2::new(Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)),
        }
    }

    pub fn distr(&self) -> Vector2<f64> {
        self.amplitudes.map(|x| x.norm_sqr())
    }
}

impl std::fmt::Display for Qubit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.amplitudes)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QRegister {
    qubits: DVector<Complex<f64>>,
}

impl QRegister {
    pub fn distr(&self) -> DVector<f64> {
        let iter = self.qubits.iter().map(|x| x.norm_sqr());
        DVector::from_iterator(self.qubits.len(), iter)
    }
}

impl std::fmt::Display for QRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.qubits)
    }
}

impl From<Qubit> for QRegister {
    fn from(qubit: Qubit) -> Self {
        QRegister {
            qubits: DVector::from_row_slice(qubit.amplitudes.data.as_slice()),
        }
    }
}

impl<B: IntoIterator<Item = Qubit>> From<B> for QRegister {
    fn from(qubits: B) -> Self {
        QRegister {
            qubits: qubits
                .into_iter()
                .map(|q| q.amplitudes)
                .fold(DVector::identity(1), |acc, q| acc.kronecker(&q)),
        }
    }
}

// @@@@@@@@@@@@@
// @@ Circuit @@
// @@@@@@@@@@@@@

#[derive(Debug, Clone)]
pub struct QuantumCircuit {
    pub n_qubits: usize,
    pub gates: Vec<CircuitGate>,
}

impl QuantumCircuit {
    pub fn new(n_qubits: usize) -> Self {
        QuantumCircuit {
            n_qubits,
            gates: Vec::new(),
        }
    }

    pub fn push_gate(&mut self, gate: CircuitGate) {
        assert!(gate.span.end() <= self.n_qubits);
        self.gates.push(gate);
    }

    pub fn g_id(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(CircuitGate::at(Identity, qix));
    }

    pub fn g_x(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(CircuitGate::at(PauliX, qix));
    }

    pub fn g_y(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(CircuitGate::at(PauliY, qix));
    }

    pub fn g_z(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(CircuitGate::at(PauliZ, qix));
    }

    pub fn g_h(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(CircuitGate::at(Hadamard, qix));
    }

    pub fn g_s(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(CircuitGate::at(Phase, qix));
    }

    pub fn g_t(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(CircuitGate::at(Pi8, qix));
    }

    pub fn g_cxu(&mut self, qixr: Range<usize>) {
        assert!(qixr.end <= self.n_qubits);
        self.gates.push(CircuitGate::new(CNOTup, qixr));
    }

    pub fn g_cxd(&mut self, qixr: Range<usize>) {
        assert!(qixr.end <= self.n_qubits);
        self.gates.push(CircuitGate::new(CNOTdown, qixr));
    }

    pub fn g_cz(&mut self, qixr: Range<usize>) {
        assert!(qixr.end <= self.n_qubits);
        self.gates.push(CircuitGate::new(ConZ, qixr));
    }

    pub fn g_swap(&mut self, qixr: Range<usize>) {
        assert!(qixr.end <= self.n_qubits);
        self.gates.push(CircuitGate::new(Swap, qixr));
    }

    pub fn g_toff(&mut self, qixr: Range<usize>) {
        assert!(qixr.end <= self.n_qubits);
        self.gates.push(CircuitGate::new(Toffoli, qixr));
    }

    pub fn eval(self) -> Block {
        let Self { n_qubits, gates } = self;
        let mut circuit = (0..n_qubits).fold(Block::one(), |acc, _| acc.tensor_product(Identity));
        for (gate, qrange) in gates.into_iter().map(|g| g.deconstruct()) {
            let mut gate_block = gate.block();
            let mut new_block = Block::one();
            for _ in 0..qrange.start() {
                new_block = new_block.tensor_product(Identity);
            }
            new_block = new_block.tensor_product(gate_block);
            for _ in qrange.end()..self.n_qubits {
                new_block = new_block.tensor_product(Identity);
            }
            gate_block = new_block;
            circuit = &circuit * &gate_block;
        }
        circuit
    }

    pub fn into_contraction_graph(self) -> ContractionGraph {
        self.into()
    }
}

// @@@@@@@@@@@@@@@@@
// @@ COMPUTATION @@
// @@@@@@@@@@@@@@@@@

pub trait TensorProduct<Rhs = Self> {
    type Output;

    fn tensor_product(&self, rhs: impl Into<Rhs>) -> Self::Output;
}

impl<G: QuantumGate> TensorProduct<Block> for G {
    type Output = Block;

    fn tensor_product(&self, rhs: impl Into<Block>) -> Self::Output {
        self.block().tensor_product(rhs)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpannedBlock {
    block: Block,
    span: GateSpan,
}

impl SpannedBlock {
    fn new(block: Block, span: GateSpan) -> Self {
        Self { block, span }
    }

    pub fn merged_span(&self, rhs: &SpannedBlock) -> GateSpan {
        self.span.full_join(&rhs.span)
    }

    pub fn adapt_to_span(mut self, span: GateSpan) -> Self {
        let mut new_block = Block::one();
        for _ in span.start()..self.span.start() {
            new_block = new_block.tensor_product(Identity);
        }
        new_block = new_block.tensor_product(self.block);
        for _ in self.span.end()..span.end() {
            new_block = new_block.tensor_product(Identity);
        }
        self.block = new_block;
        self.span = span;
        self
    }

    pub fn into_block(self) -> Block {
        self.block
    }
}

impl std::fmt::Display for SpannedBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SpannedBlock{}:\n{}", self.span, self.block)
    }
}

impl From<CircuitGate> for SpannedBlock {
    fn from(gate: CircuitGate) -> Self {
        Self::new(gate.kind.block(), gate.span)
    }
}

impl TensorProduct for SpannedBlock {
    type Output = SpannedBlock;

    fn tensor_product(&self, rhs: impl Into<SpannedBlock>) -> Self::Output {
        let rhs = rhs.into();
        SpannedBlock {
            block: &self.block * &rhs.block,
            span: self.span.full_join(&rhs.span),
        }
    }
}

impl Mul<&SpannedBlock> for &SpannedBlock {
    type Output = SpannedBlock;

    fn mul(self, rhs: &SpannedBlock) -> Self::Output {
        assert_eq!(self.span, rhs.span, "Incompatible spans");
        SpannedBlock {
            block: &self.block * &rhs.block,
            span: self.span.full_join(&rhs.span),
        }
    }
}

impl Mul<SpannedBlock> for SpannedBlock {
    type Output = SpannedBlock;

    fn mul(self, rhs: SpannedBlock) -> Self::Output {
        assert_eq!(self.span, rhs.span, "Incompatible spans");
        SpannedBlock {
            block: &self.block * &rhs.block,
            span: self.span.full_join(&rhs.span),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    matrix_repr: DMatrix<Complex<f64>>,
    dim: usize,
}

impl Block {
    pub fn one() -> Self {
        Block {
            matrix_repr: DMatrix::from_row_slice(1, 1, &[1.0]).map(|x| Complex::new(x, 0.0)),
            dim: 1,
        }
    }
}

impl TensorProduct for Block {
    type Output = Block;

    fn tensor_product(&self, rhs: impl Into<Block>) -> Self::Output {
        let b = rhs.into();
        Block {
            matrix_repr: self.as_ref().kronecker(b.as_ref()),
            dim: self.dim * b.dim,
        }
    }
}

impl<G: QuantumGate> From<G> for Block {
    fn from(gate: G) -> Self {
        gate.block()
    }
}

impl AsRef<DMatrix<Complex<f64>>> for Block {
    fn as_ref(&self) -> &DMatrix<Complex<f64>> {
        &self.matrix_repr
    }
}

impl Mul<&Block> for &Block {
    type Output = Block;

    fn mul(self, rhs: &Block) -> Self::Output {
        Block {
            matrix_repr: self.as_ref() * rhs.as_ref(),
            dim: self.dim,
        }
    }
}

impl Mul<Block> for &Block {
    type Output = Block;

    fn mul(self, rhs: Block) -> Self::Output {
        Block {
            matrix_repr: self.as_ref() * rhs.as_ref(),
            dim: self.dim,
        }
    }
}

impl Mul<Block> for Block {
    type Output = Block;

    fn mul(self, rhs: Block) -> Self::Output {
        Block {
            matrix_repr: self.as_ref() * rhs.as_ref(),
            dim: self.dim,
        }
    }
}

impl Mul<&Block> for Block {
    type Output = Block;

    fn mul(self, rhs: &Block) -> Self::Output {
        Block {
            matrix_repr: self.as_ref() * rhs.as_ref(),
            dim: self.dim,
        }
    }
}

impl<Q: Into<QRegister>> Mul<Q> for Block {
    type Output = QRegister;

    fn mul(self, rhs: Q) -> Self::Output {
        QRegister {
            qubits: self.as_ref() * rhs.into().qubits,
        }
    }
}

impl std::ops::Add for Block {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Block {
            matrix_repr: self.as_ref() + rhs.as_ref(),
            dim: self.dim,
        }
    }
}

impl std::ops::Sub for Block {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Block {
            matrix_repr: self.as_ref() - rhs.as_ref(),
            dim: self.dim,
        }
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.matrix_repr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniform_hadamard_5q() {
        let inr = QRegister::from([
            Qubit::zero(),
            Qubit::zero(),
            Qubit::zero(),
            Qubit::zero(),
            Qubit::zero(),
        ]);

        let mut circ = QuantumCircuit::new(5);
        circ.g_h(0);
        circ.g_h(1);
        circ.g_h(2);
        circ.g_h(3);
        circ.g_h(4);

        let t_eval = circ.eval();
        // println!("Circuit eval: {}", t_eval);

        let qstate = t_eval * inr;
        // println!("Qstate: {}", qstate);
        assert_eq!(qstate.qubits.len(), 32);
        for i in 0..32 {
            assert!((qstate.qubits[i] - Complex::new(1.0 / 32.0_f64.sqrt(), 0.0)).norm() < 1e-10);
        }
    }

    #[test]
    fn inverted_cnot() {
        let mut circ = QuantumCircuit::new(2);
        circ.g_h(0);
        circ.g_h(1);
        circ.g_cxu(0..2);
        circ.g_h(0);
        circ.g_h(1);

        let t_eval = circ.eval();
        println!("Circuit eval: {}", t_eval);

        let cnot_inverted = CNOTdown.block();
        // println!("{}", cnot_inverted);

        for i in 0..4 {
            for j in 0..4 {
                assert!(
                    (t_eval.matrix_repr[(i, j)] - cnot_inverted.matrix_repr[(i, j)]).norm() < 1e-10
                );
            }
        }
    }
}
