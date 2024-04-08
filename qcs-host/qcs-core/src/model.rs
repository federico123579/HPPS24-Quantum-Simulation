use std::{f64::consts::FRAC_PI_4, ops::Range};

use enum_dispatch::enum_dispatch;
use nalgebra::{Complex, DMatrix, DVector, Vector2};

use crate::contractions::TensorNetwork;

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

impl<const N: usize> From<[Qubit; N]> for QRegister {
    fn from(qubits: [Qubit; N]) -> Self {
        QRegister {
            qubits: qubits
                .into_iter()
                .map(|q| q.amplitudes)
                .fold(DVector::identity(1), |acc, q| acc.kronecker(&q)),
        }
    }
}

// @@@@@@@@@@@
// @@ Gates @@
// @@@@@@@@@@@

#[derive(Debug, Clone, Copy)]
#[enum_dispatch(QuantumGate)]
pub enum Gate {
    Identity(IdentityGate),
    PauliX(PauliXGate),
    PauliY(PauliYGate),
    PauliZ(PauliZGate),
    Hadamard(HadamardGate),
    Phase(PhaseGate),
    Pi8(Pi8Gate),
    CNOTup(CNOTupGate),
    CNOTdown(CNOTdownGate),
    ConZ(ConZGate),
    Swap(SwapGate),
    Toffoli(ToffoliGate),
}

impl std::fmt::Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gate::Identity(_) => write!(f, "I"),
            Gate::PauliX(_) => write!(f, "X"),
            Gate::PauliY(_) => write!(f, "Y"),
            Gate::PauliZ(_) => write!(f, "Z"),
            Gate::Hadamard(_) => write!(f, "H"),
            Gate::Phase(_) => write!(f, "S"),
            Gate::Pi8(_) => write!(f, "T"),
            Gate::CNOTup(_) => write!(f, "CNOT"),
            Gate::CNOTdown(_) => write!(f, "CNOTinv"),
            Gate::ConZ(_) => write!(f, "CZED"),
            Gate::Swap(_) => write!(f, "SWAP"),
            Gate::Toffoli(_) => write!(f, "TOFF"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IdentityGate;
#[derive(Debug, Clone, Copy)]
pub struct PauliXGate;
#[derive(Debug, Clone, Copy)]
pub struct PauliYGate;
#[derive(Debug, Clone, Copy)]
pub struct PauliZGate;
#[derive(Debug, Clone, Copy)]
pub struct HadamardGate;
#[derive(Debug, Clone, Copy)]
pub struct PhaseGate;
#[derive(Debug, Clone, Copy)]
pub struct Pi8Gate;
#[derive(Debug, Clone, Copy)]
pub struct CNOTupGate;
#[derive(Debug, Clone, Copy)]
pub struct CNOTdownGate;
#[derive(Debug, Clone, Copy)]
pub struct ConZGate;
#[derive(Debug, Clone, Copy)]
pub struct SwapGate;
#[derive(Debug, Clone, Copy)]
pub struct ToffoliGate;

#[enum_dispatch]
pub trait QuantumGate {
    fn matrix(&self) -> DMatrix<Complex<f64>>;
    fn rank(&self) -> u8;
    fn block(&self) -> Block {
        Block {
            matrix_repr: self.matrix(),
            dim: self.rank() as usize,
        }
    }
}

impl<G: QuantumGate> From<G> for Block {
    fn from(gate: G) -> Self {
        gate.block()
    }
}

impl QuantumGate for IdentityGate {
    fn rank(&self) -> u8 {
        1
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        DMatrix::identity(2, 2)
    }
}

impl QuantumGate for PauliXGate {
    fn rank(&self) -> u8 {
        1
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        DMatrix::from_row_slice(2, 2, &[0.0, 1.0, 1.0, 0.0]).map(|x| Complex::new(x, 0.0))
    }
}

impl QuantumGate for PauliYGate {
    fn rank(&self) -> u8 {
        1
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        DMatrix::from_row_slice(2, 2, &[0.0, -1.0, 1.0, 0.0]).map(|x| Complex::new(x, 0.0))
    }
}

impl QuantumGate for PauliZGate {
    fn rank(&self) -> u8 {
        1
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        DMatrix::from_row_slice(2, 2, &[1.0, 0.0, 0.0, -1.0]).map(|x| Complex::new(x, 0.0))
    }
}

impl QuantumGate for HadamardGate {
    fn rank(&self) -> u8 {
        1
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        DMatrix::from_row_slice(2, 2, &[1.0, 1.0, 1.0, -1.0])
            .map(|x| Complex::new(x, 0.0) / 2.0_f64.sqrt())
    }
}

impl QuantumGate for PhaseGate {
    fn rank(&self) -> u8 {
        1
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        DMatrix::from_row_slice(2, 2, &[1.0.into(), 0.0.into(), 0.0.into(), Complex::i()])
    }
}

impl QuantumGate for Pi8Gate {
    fn rank(&self) -> u8 {
        1
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        DMatrix::from_row_slice(
            2,
            2,
            &[1.0.into(), 0.0.into(), 0.0.into(), Complex::cis(FRAC_PI_4)],
        )
    }
}

impl QuantumGate for CNOTupGate {
    fn rank(&self) -> u8 {
        2
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        DMatrix::from_row_slice(
            4,
            4,
            &[
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
            ],
        )
        .map(|x| Complex::new(x, 0.0))
    }
}

impl QuantumGate for CNOTdownGate {
    fn rank(&self) -> u8 {
        2
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        DMatrix::from_row_slice(
            4,
            4,
            &[
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            ],
        )
        .map(|x| Complex::new(x, 0.0))
    }
}

impl QuantumGate for ConZGate {
    fn rank(&self) -> u8 {
        2
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        DMatrix::from_row_slice(
            4,
            4,
            &[
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, -1.0,
            ],
        )
        .map(|x| Complex::new(x, 0.0))
    }
}

impl QuantumGate for SwapGate {
    fn rank(&self) -> u8 {
        2
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        DMatrix::from_row_slice(
            4,
            4,
            &[
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        )
        .map(|x| Complex::new(x, 0.0))
    }
}

impl QuantumGate for ToffoliGate {
    fn rank(&self) -> u8 {
        3
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        DMatrix::from_row_slice(
            8,
            8,
            &[
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
            ],
        )
        .map(|x| Complex::new(x, 0.0))
    }
}

// @@@@@@@@@@@@@
// @@ Circuit @@
// @@@@@@@@@@@@@

#[derive(Debug, Clone)]
pub struct QuantumCircuit {
    pub n_qubits: usize,
    pub gates: Vec<GateOnLanes>,
}

impl QuantumCircuit {
    pub fn new(n_qubits: usize) -> Self {
        QuantumCircuit {
            n_qubits,
            gates: Vec::new(),
        }
    }

    pub fn push_gate(&mut self, gate: GateOnLanes) {
        assert!(gate.lanes.end <= self.n_qubits);
        self.gates.push(gate);
    }

    pub fn g_id(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(GateOnLanes::at(IdentityGate, qix));
    }

    pub fn g_x(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(GateOnLanes::at(PauliXGate, qix));
    }

    pub fn g_y(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(GateOnLanes::at(PauliYGate, qix));
    }

    pub fn g_z(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(GateOnLanes::at(PauliZGate, qix));
    }

    pub fn g_h(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(GateOnLanes::at(HadamardGate, qix));
    }

    pub fn g_s(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(GateOnLanes::at(PhaseGate, qix));
    }

    pub fn g_t(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(GateOnLanes::at(Pi8Gate, qix));
    }

    pub fn g_cxu(&mut self, qixr: Range<usize>) {
        assert!(qixr.end <= self.n_qubits);
        self.gates.push(GateOnLanes::new(CNOTupGate, qixr));
    }

    pub fn g_cxd(&mut self, qixr: Range<usize>) {
        assert!(qixr.end <= self.n_qubits);
        self.gates.push(GateOnLanes::new(CNOTdownGate, qixr));
    }

    pub fn g_cz(&mut self, qixr: Range<usize>) {
        assert!(qixr.end <= self.n_qubits);
        self.gates.push(GateOnLanes::new(ConZGate, qixr));
    }

    pub fn g_swap(&mut self, qixr: Range<usize>) {
        assert!(qixr.end <= self.n_qubits);
        self.gates.push(GateOnLanes::new(SwapGate, qixr));
    }

    pub fn g_toff(&mut self, qixr: Range<usize>) {
        assert!(qixr.end <= self.n_qubits);
        self.gates.push(GateOnLanes::new(ToffoliGate, qixr));
    }

    pub fn eval(self) -> Block {
        let Self { n_qubits, gates } = self;
        let mut circuit =
            (0..n_qubits).fold(Block::one(), |acc, _| acc.tensor_product(IdentityGate));
        for (gate, qrange) in gates.into_iter().map(|g| g.deconstruct()) {
            let mut gate_block = gate.block();
            // FIXME: this works only for 1-qubit gates
            let mut new_block = Block::one();
            for _ in 0..qrange.start {
                new_block = new_block.tensor_product(IdentityGate);
            }
            new_block = new_block.tensor_product(gate_block);
            for _ in qrange.end..self.n_qubits {
                new_block = new_block.tensor_product(IdentityGate);
            }
            gate_block = new_block;
            circuit = &circuit * &gate_block;
        }
        circuit
    }

    pub fn into_contraction_graph(self) -> TensorNetwork {
        self.into()
    }
}

#[derive(Debug, Clone)]
pub struct GateOnLanes {
    pub gate: Gate,
    pub lanes: Range<usize>,
}

impl GateOnLanes {
    pub fn new(gate: impl Into<Gate>, lanes: Range<usize>) -> Self {
        GateOnLanes {
            gate: gate.into(),
            lanes,
        }
    }

    pub fn at(gate: impl Into<Gate>, lane: usize) -> Self {
        Self::new(gate, lane..lane + 1)
    }

    pub fn deconstruct(self) -> (Gate, Range<usize>) {
        (self.gate, self.lanes)
    }

    pub fn rank(&self) -> u8 {
        self.gate.rank()
    }

    pub fn block(&self) -> Block {
        self.gate.block()
    }

    pub fn lanes(&self) -> Range<usize> {
        self.lanes.clone()
    }
}

impl std::fmt::Display for GateOnLanes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}[{}:{}]",
            self.gate,
            self.lanes.start,
            self.lanes.end - 1
        )
    }
}

// @@@@@@@@@@@@@@@@@
// @@ COMPUTATION @@
// @@@@@@@@@@@@@@@@@

trait TensorProduct {
    fn tensor_product(&self, rhs: impl Into<Block>) -> Block;
}

impl<G: QuantumGate> TensorProduct for G {
    fn tensor_product(&self, rhs: impl Into<Block>) -> Block {
        self.block().tensor_product(rhs)
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
    fn tensor_product(&self, rhs: impl Into<Block>) -> Block {
        let b = rhs.into();
        Block {
            matrix_repr: self.as_ref().kronecker(b.as_ref()),
            dim: self.dim * b.dim,
        }
    }
}

impl AsRef<DMatrix<Complex<f64>>> for Block {
    fn as_ref(&self) -> &DMatrix<Complex<f64>> {
        &self.matrix_repr
    }
}

impl std::ops::Mul<&Block> for &Block {
    type Output = Block;

    fn mul(self, rhs: &Block) -> Self::Output {
        Block {
            matrix_repr: self.as_ref() * rhs.as_ref(),
            dim: self.dim,
        }
    }
}

impl std::ops::Mul<Block> for &Block {
    type Output = Block;

    fn mul(self, rhs: Block) -> Self::Output {
        Block {
            matrix_repr: self.as_ref() * rhs.as_ref(),
            dim: self.dim,
        }
    }
}

impl std::ops::Mul<Block> for Block {
    type Output = Block;

    fn mul(self, rhs: Block) -> Self::Output {
        Block {
            matrix_repr: self.as_ref() * rhs.as_ref(),
            dim: self.dim,
        }
    }
}

impl std::ops::Mul<&Block> for Block {
    type Output = Block;

    fn mul(self, rhs: &Block) -> Self::Output {
        Block {
            matrix_repr: self.as_ref() * rhs.as_ref(),
            dim: self.dim,
        }
    }
}

impl<Q: Into<QRegister>> std::ops::Mul<Q> for Block {
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

        let cnot_inverted = CNOTdownGate.block();
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
