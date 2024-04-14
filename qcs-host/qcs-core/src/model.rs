pub mod blocks;
pub mod gates;
pub mod span;

use nalgebra::{
    allocator::Allocator, Complex, DMatrix, DVector, DefaultAllocator, Dim, DimMul, DimProd, Dyn,
    Matrix, OMatrix, Storage, VecStorage, Vector2, U1,
};

use crate::model::gates::*;

use self::{
    blocks::Block,
    gates::{Gate, QuantumGate},
};

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

impl AsRef<Vector2<Complex<f64>>> for Qubit {
    fn as_ref(&self) -> &Vector2<Complex<f64>> {
        &self.amplitudes
    }
}

impl From<Qubit> for DVector<Complex<f64>> {
    fn from(value: Qubit) -> Self {
        DVector::from_row_slice(value.amplitudes.data.as_slice())
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

impl AsRef<DVector<Complex<f64>>> for QRegister {
    fn as_ref(&self) -> &DVector<Complex<f64>> {
        &self.qubits
    }
}

impl From<QRegister> for DVector<Complex<f64>> {
    fn from(value: QRegister) -> Self {
        value.qubits
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
    pub gates: Vec<Gate>,
}

impl QuantumCircuit {
    pub fn new(n_qubits: usize) -> Self {
        QuantumCircuit {
            n_qubits,
            gates: Vec::new(),
        }
    }

    pub fn push_gate(&mut self, gate: Gate) {
        self.gates.push(gate);
    }

    pub fn g_id(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(Identity::new(qix).into());
    }

    pub fn g_x(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(PauliX::new(qix).into());
    }

    pub fn g_y(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(PauliY::new(qix).into());
    }

    pub fn g_z(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(PauliZ::new(qix).into());
    }

    pub fn g_h(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(Hadamard::new(qix).into());
    }

    pub fn g_p(&mut self, phase: f64, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(Phase::new(phase, qix).into());
    }

    pub fn g_t(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(Phase::pi8(qix).into());
    }

    pub fn g_cx(&mut self, qix_control: usize, qix_target: usize) {
        assert!(qix_control < self.n_qubits && qix_target < self.n_qubits);
        self.gates.push(CX::new(qix_control, qix_target).into());
    }

    pub fn g_cy(&mut self, qix_control: usize, qix_target: usize) {
        assert!(qix_control < self.n_qubits && qix_target < self.n_qubits);
        self.gates.push(CY::new(qix_control, qix_target).into());
    }

    pub fn g_cz(&mut self, qix_control: usize, qix_target: usize) {
        assert!(qix_control < self.n_qubits && qix_target < self.n_qubits);
        self.gates.push(CZ::new(qix_control, qix_target).into());
    }

    pub fn g_swap(&mut self, qix1: usize, qix2: usize) {
        assert!(qix1 < self.n_qubits && qix2 < self.n_qubits);
        self.gates.push(Swap::new(qix1, qix2).into());
    }

    pub fn g_toff(&mut self, qix_control1: usize, qix_control2: usize, qix_target: usize) {
        assert!(
            qix_control1 < self.n_qubits
                && qix_control2 < self.n_qubits
                && qix_target < self.n_qubits
        );
        self.gates
            .push(Toffoli::new((qix_control1, qix_control2), qix_target).into());
    }

    pub fn eval(self) -> Block {
        let Self { n_qubits, gates } = self;
        let mut circuit =
            (0..n_qubits).fold(Block::one(), |acc, i| acc.tensor_product(Identity::new(i)));
        for gate in gates {
            let span = gate.span();
            let mut gate_block = gate.block();
            let mut new_block = Block::one();
            for i in 0..span.min() {
                new_block = new_block.tensor_product(Identity::new(i));
            }
            new_block = new_block.tensor_product(gate_block);
            for i in (span.max() + 1)..self.n_qubits {
                new_block = new_block.tensor_product(Identity::new(i));
            }
            gate_block = new_block;
            circuit = &circuit * &gate_block;
        }
        circuit
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

type ComplexMatrix<R, C> = Matrix<Complex<f64>, R, C, VecStorage<Complex<f64>, R, C>>;

impl<R1, R2, C1, C2> TensorProduct<ComplexMatrix<R2, C2>> for ComplexMatrix<R1, C1>
where
    R1: Dim + DimMul<R2>,
    R2: Dim,
    C1: Dim + DimMul<C2>,
    C2: Dim,
    VecStorage<Complex<f64>, R1, C1>: Storage<Complex<f64>, R1, C1>,
    VecStorage<Complex<f64>, R2, C2>: Storage<Complex<f64>, R2, C2>,
    DefaultAllocator: Allocator<Complex<f64>, DimProd<R1, R2>, DimProd<C1, C2>>,
{
    type Output = OMatrix<Complex<f64>, DimProd<R1, R2>, DimProd<C1, C2>>;

    fn tensor_product(&self, rhs: impl Into<ComplexMatrix<R2, C2>>) -> Self::Output {
        self.kronecker(&rhs.into())
    }
}

pub trait Braket: Sized {
    fn ket(&self) -> DVector<Complex<f64>>;

    fn bra(&self) -> OMatrix<Complex<f64>, U1, Dyn> {
        self.ket().transpose().conjugate()
    }

    fn braket(&self) -> Complex<f64> {
        *(self.bra() * self.ket()).as_scalar()
    }

    fn ketbra(&self) -> DMatrix<Complex<f64>> {
        self.ket() * self.bra()
    }
}

impl Braket for QRegister {
    fn ket(&self) -> DVector<Complex<f64>> {
        self.qubits.to_owned()
    }
}

impl Braket for Qubit {
    fn ket(&self) -> DVector<Complex<f64>> {
        DVector::from_row_slice(self.amplitudes.data.as_slice())
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
        println!("Circuit eval: {}", t_eval);

        let qstate = t_eval * inr;
        println!("Qstate: {}", qstate);
        assert_eq!(qstate.qubits.len(), 32);
        for i in 0..32 {
            assert!((qstate.qubits[i] - Complex::new(1.0 / 32.0_f64.sqrt(), 0.0)).norm() < 1e-10);
        }
    }

    // #[test]
    // fn inverted_cnot() {
    //     let mut circ = QuantumCircuit::new(2);
    //     circ.g_h(0);
    //     circ.g_h(1);
    //     circ.g_cxu(0..2);
    //     circ.g_h(0);
    //     circ.g_h(1);

    //     let t_eval = circ.eval();
    //     println!("Circuit eval: {}", t_eval);

    //     let cnot_inverted = CNOTdown.block();
    //     // println!("{}", cnot_inverted);

    //     for i in 0..4 {
    //         for j in 0..4 {
    //             assert!(
    //                 (t_eval.matrix_repr[(i, j)] - cnot_inverted.matrix_repr[(i, j)]).norm() < 1e-10
    //             );
    //         }
    //     }
    // }
}
