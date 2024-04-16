//! The quantum model module contains the basic structures for quantum computing.
//!
//! The `Qubit` struct represents a single qubit, which is a quantum system with two states.
//! The `QRegister` struct represents a quantum register, which is a collection of qubits.
//! The `QuantumCircuit` struct represents a quantum circuit, which is a sequence of quantum gates.

pub mod blocks;
pub mod gates;
pub mod span;

use nalgebra::{
    allocator::Allocator, Complex, DMatrix, DVector, DefaultAllocator, Dim, DimMul, DimProd, Dyn,
    Matrix, OMatrix, Storage, VecStorage, Vector2,
};

use crate::model::gates::*;

use self::{
    blocks::Block,
    gates::{Gate, QuantumGate},
};

// @@@@@@@@@@@@
// @@ Qubits @@
// @@@@@@@@@@@@

/// A qubit is a quantum system with two states, |0⟩ and |1⟩.
/// It can be represented as a linear combination of these states:
/// |ψ⟩ = α|0⟩ + β|1⟩
/// where α and β are complex numbers.
///
/// The probability of measuring the qubit in state |0⟩ is |α|² and in state |1⟩ is |β|².
/// The qubit is normalized if |α|² + |β|² = 1.
/// The amplitudes α and β are stored in a 2-dimensional vector.
///
/// The qubit can be initialized as |0⟩, |1⟩ or with custom amplitudes.
#[derive(Debug, Clone, PartialEq)]
pub struct Qubit {
    amplitudes: Vector2<Complex<f64>>,
}

impl Qubit {
    /// Creates a new qubit with custom amplitudes.
    pub fn new(alpha: Complex<f64>, beta: Complex<f64>) -> Self {
        Qubit {
            amplitudes: Vector2::new(alpha, beta),
        }
    }

    /// Creates the qubit |0⟩.
    pub fn zero() -> Self {
        Qubit {
            amplitudes: Vector2::new(Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)),
        }
    }

    /// Creates the qubit |1⟩.
    pub fn one() -> Self {
        Qubit {
            amplitudes: Vector2::new(Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)),
        }
    }

    /// Returns the distribution of the qubit.
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

/// A quantum register is a collection of qubits.
#[derive(Debug, Clone, PartialEq)]
pub struct QRegister {
    qubits: DVector<Complex<f64>>,
}

impl QRegister {
    /// Returns the distribution of the register.
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

/// A quantum circuit is a sequence of quantum gates that are applied to a quantum register.
/// The gates are applied in order from left to right.
/// The circuit is initialized with a number of qubits and an empty list of gates.
///
/// The gates can be added to the circuit using the `push_gate` method.
/// The circuit can be evaluated to a block using the `eval` method.
///
/// The circuit can also be built using the methods that correspond to the quantum gates.
#[derive(Debug, Clone)]
pub struct QuantumCircuit {
    pub n_qubits: usize,
    pub gates: Vec<Gate>,
}

impl QuantumCircuit {
    /// Creates a new quantum circuit with a number of qubits.
    pub fn new(n_qubits: usize) -> Self {
        QuantumCircuit {
            n_qubits,
            gates: Vec::new(),
        }
    }

    /// Adds a gate to the circuit.
    pub fn push_gate(&mut self, gate: Gate) {
        self.gates.push(gate);
    }

    /// Adds the Identity gate to the circuit.
    pub fn g_id(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(Identity::new(qix).into());
    }

    /// Adds the Pauli-X gate to the circuit.
    pub fn g_x(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(PauliX::new(qix).into());
    }

    /// Adds the Pauli-Y gate to the circuit.
    pub fn g_y(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(PauliY::new(qix).into());
    }

    /// Adds the Pauli-Z gate to the circuit.
    pub fn g_z(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(PauliZ::new(qix).into());
    }

    /// Adds the Hadamard gate to the circuit.
    pub fn g_h(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(Hadamard::new(qix).into());
    }

    /// Adds the Phase gate to the circuit.
    pub fn g_p(&mut self, phase: f64, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(Phase::new(phase, qix).into());
    }

    /// Adds the S gate to the circuit.
    pub fn g_s(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(Phase::s(qix).into());
    }

    /// Adds the T gate to the circuit.
    pub fn g_t(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(Phase::t(qix).into());
    }

    /// Adds the inverse of S
    pub fn g_s_dg(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(Phase::s_inv(qix).into());
    }

    /// Adds the inverse of T
    pub fn g_t_dg(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(Phase::t_inv(qix).into());
    }

    /// Adds the sqrt(NOT) gate to the circuit.
    pub fn g_sx(&mut self, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(SX::new(qix).into());
    }

    /// Adds the RX gate to the circuit.
    pub fn g_rx(&mut self, angle: f64, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(RX::new(angle, qix).into());
    }

    /// Adds the RY gate to the circuit.
    pub fn g_ry(&mut self, angle: f64, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(RY::new(angle, qix).into());
    }

    /// Adds the RZ gate to the circuit.
    pub fn g_rz(&mut self, angle: f64, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(RZ::new(angle, qix).into());
    }

    /// Adds the CX gate to the circuit.
    pub fn g_cx(&mut self, qix_control: usize, qix_target: usize) {
        assert!(qix_control < self.n_qubits && qix_target < self.n_qubits);
        self.gates.push(CX::new(qix_control, qix_target).into());
    }

    /// Adds the CY gate to the circuit.
    pub fn g_cy(&mut self, qix_control: usize, qix_target: usize) {
        assert!(qix_control < self.n_qubits && qix_target < self.n_qubits);
        self.gates.push(CY::new(qix_control, qix_target).into());
    }

    /// Adds the CZ gate to the circuit.
    pub fn g_cz(&mut self, qix_control: usize, qix_target: usize) {
        assert!(qix_control < self.n_qubits && qix_target < self.n_qubits);
        self.gates.push(CZ::new(qix_control, qix_target).into());
    }

    /// Adds the CP gate to the circuit.
    pub fn g_cp(&mut self, phase: f64, qix_control: usize, qix_target: usize) {
        assert!(qix_control < self.n_qubits && qix_target < self.n_qubits);
        self.gates
            .push(CP::new(phase, qix_control, qix_target).into());
    }

    /// Adds the CRX gate to the circuit.
    pub fn g_crx(&mut self, angle: f64, qix_control: usize, qix_target: usize) {
        assert!(qix_control < self.n_qubits && qix_target < self.n_qubits);
        self.gates
            .push(CRX::new(angle, qix_control, qix_target).into());
    }

    /// Adds the CRY gate to the circuit.
    pub fn g_cry(&mut self, angle: f64, qix_control: usize, qix_target: usize) {
        assert!(qix_control < self.n_qubits && qix_target < self.n_qubits);
        self.gates
            .push(CRY::new(angle, qix_control, qix_target).into());
    }

    /// Adds the CRZ gate to the circuit.
    pub fn g_crz(&mut self, angle: f64, qix_control: usize, qix_target: usize) {
        assert!(qix_control < self.n_qubits && qix_target < self.n_qubits);
        self.gates
            .push(CRZ::new(angle, qix_control, qix_target).into());
    }

    /// Adds the CH gate to the circuit.
    pub fn g_ch(&mut self, qix_control: usize, qix_target: usize) {
        assert!(qix_control < self.n_qubits && qix_target < self.n_qubits);
        self.gates.push(CH::new(qix_control, qix_target).into());
    }

    /// Adds the SWAP gate to the circuit.
    pub fn g_swap(&mut self, qix1: usize, qix2: usize) {
        assert!(qix1 < self.n_qubits && qix2 < self.n_qubits);
        self.gates.push(Swap::new(qix1, qix2).into());
    }

    /// Adds the Toffoli gate to the circuit.
    pub fn g_cxx(&mut self, qix_control1: usize, qix_control2: usize, qix_target: usize) {
        assert!(
            qix_control1 < self.n_qubits
                && qix_control2 < self.n_qubits
                && qix_target < self.n_qubits
        );
        self.gates
            .push(Toffoli::new((qix_control1, qix_control2), qix_target).into());
    }

    /// Adds the Fredkit gate to the circuit.
    pub fn g_cswap(&mut self, qix_control: usize, qix_target1: usize, qix_target2: usize) {
        assert!(
            qix_control < self.n_qubits
                && qix_target1 < self.n_qubits
                && qix_target2 < self.n_qubits
        );
        self.gates
            .push(Fredkin::new(qix_control, (qix_target1, qix_target2)).into());
    }

    /// Adds the CU gate to the circuit.
    pub fn g_cu(
        &mut self,
        theta: f64,
        phi: f64,
        lambda: f64,
        gamma: f64,
        qix_control: usize,
        qix_target: usize,
    ) {
        assert!(qix_control < self.n_qubits && qix_target < self.n_qubits);
        self.gates
            .push(CU::new(theta, phi, lambda, gamma, qix_control, qix_target).into());
    }

    /// Adds the U gate to the circuit.
    pub fn g_u(&mut self, theta: f64, phi: f64, lambda: f64, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(U::new(theta, phi, lambda, qix).into());
    }

    /// Adds the U1 gate to the circuit.
    pub fn g_u1(&mut self, lambda: f64, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(U1::new(lambda, qix).into());
    }

    /// Adds the U2 gate to the circuit.
    pub fn g_u2(&mut self, phi: f64, lambda: f64, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(U2::new(phi, lambda, qix).into());
    }

    /// Adds the U3 gate to the circuit.
    pub fn g_u3(&mut self, theta: f64, phi: f64, lambda: f64, qix: usize) {
        assert!(qix < self.n_qubits);
        self.gates.push(U3::new(theta, phi, lambda, qix).into());
    }

    /// Evaluates the circuit to a single block, equivalent to the matrix
    /// representation of the whole circuit.
    pub fn eval(self) -> Block {
        let Self { n_qubits, gates } = self;
        let mut circuit =
            (0..n_qubits).fold(Block::one(), |acc, i| acc.tensor_product(Identity::new(i)));
        for gate in gates {
            let span = gate.span();
            let mut gate_block = gate.block();
            let mut new_block = Block::one();
            for i in 0..span.start() {
                new_block = new_block.tensor_product(Identity::new(i));
            }
            new_block = new_block.tensor_product(gate_block);
            for i in (span.end() + 1)..self.n_qubits {
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

/// An interface for tensor product operations.
pub trait TensorProduct<Rhs = Self> {
    type Output;

    /// Computes the tensor product of two objects.
    /// An example of tensor product of two matrices A and B:
    /// ```text
    /// A ⊗ B = [a₁₁B a₁₂B ... a₁ₘB]
    ///         [a₂₁B a₂₂B ... a₂ₘB]
    ///         [...  ...  ... ... ]
    ///         [aₙ₁B  aₙ₂B ... aₙₘB]
    /// ```
    /// where A is an n×m matrix and B is an p×q matrix.
    /// The result is an np×mq matrix.
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

/// An interface for braket operations.
///
/// A braket is a pair of a bra and a ket, denoted as ⟨ψ|ϕ⟩.
/// The braket is a complex number that represents the inner product of two quantum states.
/// The braket can be computed from the bra and ket using the formula ⟨ψ|ϕ⟩ = ψ†ϕ.
///
/// The outer product of a ket and a bra is called a ketbra, denoted as |ψ⟩⟨ϕ|.
/// The ketbra is a matrix that represents the outer product of two quantum states.
/// The ketbra can be computed from the bra and ket using the formula |ψ⟩⟨ϕ| = ψϕ†.
///
/// The braket and ketbra are useful for computing the probability of measuring a quantum state.
pub trait Braket: Sized {
    /// Returns the ket of the quantum state.
    /// The ket is a column vector that represents the quantum state.
    fn ket(&self) -> DVector<Complex<f64>>;

    /// Returns the bra of the quantum state.
    /// The bra is a row vector that represents the conjugate transpose of the quantum state.
    fn bra(&self) -> OMatrix<Complex<f64>, nalgebra::U1, Dyn> {
        self.ket().transpose().conjugate()
    }

    /// Returns the braket of the quantum state.
    /// The braket is a complex number that represents the inner product of two quantum states.
    fn braket(&self) -> Complex<f64> {
        *(self.bra() * self.ket()).as_scalar()
    }

    /// Returns the ketbra of the quantum state.
    /// The ketbra is a matrix that represents the outer product of two quantum states.
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

    #[test]
    fn inverted_cnot() {
        let mut circ = QuantumCircuit::new(2);
        circ.g_h(0);
        circ.g_h(1);
        circ.g_cx(0, 1);
        circ.g_h(0);
        circ.g_h(1);

        let t_eval = circ.eval().into_matrix();
        let cx_inverted = CX::new(1, 0).block().into_matrix();

        assert!((t_eval - cx_inverted).norm() < 1e-10);
    }

    #[test]
    fn not_ajacent_cnot() {
        let mut circ = QuantumCircuit::new(3);
        circ.g_h(0);
        circ.g_h(2);
        circ.g_cx(0, 2);
        circ.g_h(0);
        circ.g_h(2);

        let t_eval = circ.eval().into_matrix();
        let cx_inverted = CX::new(2, 0).block().into_matrix();

        assert!((t_eval - cx_inverted).norm() < 1e-10);
    }

    #[test]
    fn swap() {
        let mut circ = QuantumCircuit::new(2);
        circ.g_swap(0, 1);
        circ.g_swap(1, 0);
        circ.g_swap(0, 1);
        circ.g_swap(0, 1);

        let t_eval = circ.eval().into_matrix();
        let id = Block::identity(4).into_matrix();

        assert!((t_eval - id).norm() < 1e-10);
    }
}
