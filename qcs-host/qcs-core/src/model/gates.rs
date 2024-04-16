//! This module contains all the abstractions and structures related to quantum
//! gates.
//!
//! A quantum gate is a unitary operator that acts on a quantum state, that is a
//! matrix with norm 1. The gates are represented as matrices of complex
//! numbers, and are used to perform operations on qubits.

use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

use enum_dispatch::enum_dispatch;
use nalgebra::{Complex, DMatrix};

use super::{blocks::SpannedBlock, span::Span, Block, Braket, Qubit, TensorProduct};

/// An interface for quantum gates. Each gate has a matrix representation, a
/// rank, a span, and a block representation.
///
/// The rank of a gate is the number of qubits the gate acts on. For example, a
/// rank 1 gate is a single qubit gate, a rank 2 gate is a two qubit gate, etc.
///
/// The span of a gate is the set of qubits the gate acts on.
///
/// The block representation of a gate is a decorated matrix representation of
/// the gate with some utility functions.
#[enum_dispatch]
pub trait QuantumGate {
    /// Return the matrix representation of the gate
    fn matrix(&self) -> DMatrix<Complex<f64>>;

    /// Return the rank of the gate.
    /// Rank 1 gates are single qubit gates, rank 2 gates are two qubit gates, etc.
    fn rank(&self) -> u8;

    /// Return the span of the gate
    fn span(&self) -> Span;

    /// Return the equivalent block representation of the gate
    fn block(&self) -> Block {
        self.matrix().into()
    }

    /// Return spanned block representation of the gate
    fn spanned_block(&self) -> SpannedBlock {
        SpannedBlock::new(self.block(), self.span())
    }
}

/// This represents all available quantum gates in the system.
#[enum_dispatch(QuantumGate)]
#[derive(Debug, Clone)]
pub enum Gate {
    /// The identity gate, which does nothing to the qubit.
    Identity,
    /// The Pauli X gate, which flips the qubit.
    PauliX,
    /// The Pauli Y gate, which flips the qubit and adds a phase.
    PauliY,
    /// The Pauli Z gate, which adds a phase to the qubit.
    PauliZ,
    /// The Hadamard gate, which puts the qubit in a superposition.
    Hadamard,
    /// The phase gate, which adds a phase to the qubit.
    Phase,
    /// Squared root of NOT gate
    SX,
    /// Rotates the qubit around the X axis
    RX,
    /// Rotates the qubit around the Y axis
    RY,
    /// Rotates the qubit around the Z axis
    RZ,
    /// The controlled X gate, which flips the target qubit if the control qubit is 1.
    CX,
    /// The controlled Y gate, which flips the target qubit if the control qubit is 1.
    CY,
    /// The controlled Z gate, which adds a phase to the target qubit if the control qubit is 1.
    CZ,
    /// Controlled Phase gate
    CP,
    /// Controlled rotation around the X axis
    CRX,
    /// Controlled rotation around the Y axis
    CRY,
    /// Controlled rotation around the Z axis
    CRZ,
    /// Controlled Hadamard gate
    CH,
    /// The swap gate, which swaps the states of two qubits.
    Swap,
    /// The Toffoli gate, which flips the target qubit if both control qubits are 1.
    Toffoli,
    /// Controlled Swap gate
    Fredkin,
    /// Four parameters controlled-U gate with relative phase gamma
    CU,
    /// IBM experience gate 1
    U1,
    /// IBM experience gate 2
    U2,
    /// IBM experience gate 3
    U3,
    /// Universal gate
    U,
}

impl Gate {
    /// Return true if the gate is a rank one gate.
    pub fn is_rank_one(&self) -> bool {
        self.rank() == 1
    }
}

impl std::fmt::Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gate::Identity(i) => write!(f, "I[{}]", i.lane),
            Gate::PauliX(x) => write!(f, "X[{}]", x.lane),
            Gate::PauliY(y) => write!(f, "Y[{}]", y.lane),
            Gate::PauliZ(z) => write!(f, "Z[{}]", z.lane),
            Gate::Hadamard(h) => write!(f, "H[{}]", h.lane),
            Gate::Phase(p) => write!(f, "P({:.2})[{}]", p.phase, p.lane),
            Gate::CX(cx) => write!(f, "CX[{},{}]", cx.control, cx.target),
            Gate::CY(cy) => write!(f, "CY[{},{}]", cy.control, cy.target),
            Gate::CZ(cz) => write!(f, "CZ[{},{}]", cz.control, cz.target),
            Gate::Swap(s) => write!(f, "SWAP[{}, {}]", s.lanes.0, s.lanes.1),
            Gate::Toffoli(t) => write!(f, "CCX[{},{},{}]", t.control.0, t.control.1, t.target),
            Gate::SX(sx) => write!(f, "SX[{}]", sx.lane),
            Gate::RX(rx) => write!(f, "RX({:.2})[{}]", rx.theta, rx.lane),
            Gate::RY(ry) => write!(f, "RY({:.2})[{}]", ry.theta, ry.lane),
            Gate::RZ(rz) => write!(f, "RZ({:.2})[{}]", rz.theta, rz.lane),
            Gate::CP(cp) => write!(f, "CP({:.2})[{},{}]", cp.phase, cp.control, cp.target),
            Gate::CRX(crx) => write!(f, "CRX({:.2})[{},{}]", crx.theta, crx.control, crx.target),
            Gate::CRY(cry) => write!(f, "CRY({:.2})[{},{}]", cry.theta, cry.control, cry.target),
            Gate::CRZ(crz) => write!(f, "CRZ({:.2})[{},{}]", crz.theta, crz.control, crz.target),
            Gate::CH(ch) => write!(f, "CH[{},{}]", ch.control, ch.target),
            Gate::Fredkin(fk) => {
                write!(f, "CSWAP[{},({},{})]", fk.control, fk.target.0, fk.target.1)
            }
            Gate::CU(cu) => write!(
                f,
                "CU({:.2},{:.2},{:.2},{:.2})[{},{}]",
                cu.theta, cu.phi, cu.lambda, cu.gamma, cu.control, cu.target
            ),
            Gate::U1(u1) => write!(f, "U1({:.2})[{}]", u1.lambda, u1.lane),
            Gate::U2(u2) => write!(f, "U2({:.2},{:.2})[{}]", u2.phi, u2.lambda, u2.lane),
            Gate::U3(u3) => write!(
                f,
                "U3({:.2},{:.2},{:.2})[{}]",
                u3.theta, u3.phi, u3.lambda, u3.lane
            ),
            Gate::U(u) => write!(
                f,
                "U({:.2},{:.2},{:.2})[{}]",
                u.theta, u.phi, u.lambda, u.lane
            ),
        }
    }
}

/// The Identity gate, which does nothing to the qubit.
///
/// It is represented by the matrix:
/// ```text
/// ┌     ┐
/// │ 1 0 │
/// │ 0 1 │
/// └     ┘
/// ```
///
///
/// In a quantum circuit this can be represented by either a single line
/// ```text
/// ───────────
/// ```
/// or by the gate `I`
/// ```text
///    ┌───┐
/// ───┤ I ├───
///    └───┘
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Identity {
    lane: usize,
}

impl Identity {
    /// Create a new Identity gate acting on the given lane.
    pub fn new(lane: usize) -> Self {
        Self { lane }
    }
}

impl QuantumGate for Identity {
    fn rank(&self) -> u8 {
        1
    }

    fn span(&self) -> Span {
        Span::single(self.lane)
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        DMatrix::identity(2, 2)
    }
}

/// The Pauli X gate, which flips the qubit.
///
/// It is represented by the matrix:
/// ```text
/// ┌     ┐
/// │ 0 1 │
/// │ 1 0 │
/// └     ┘
/// ```
///
/// In a quantum circuit this can be represented by the gate `X`
/// ```text
///    ┌───┐
/// ───┤ X ├───
///    └───┘
/// ```
#[derive(Debug, Clone, Copy)]
pub struct PauliX {
    lane: usize,
}

impl PauliX {
    /// Create a new Pauli X gate acting on the given lane.
    pub fn new(lane: usize) -> Self {
        Self { lane }
    }
}

impl QuantumGate for PauliX {
    fn rank(&self) -> u8 {
        1
    }

    fn span(&self) -> Span {
        Span::single(self.lane)
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        pauli_x_matrix()
    }
}

/// The Pauli Y gate, which flips the qubit and adds a phase.
///
/// It is represented by the matrix:
/// ```text
/// ┌      ┐
/// │ 0 -i │
/// │ i  0 │
/// └      ┘
/// ```
///
/// In a quantum circuit this can be represented by the gate `Y`
/// ```text
///    ┌───┐
/// ───┤ Y ├───
///    └───┘
/// ```
#[derive(Debug, Clone, Copy)]
pub struct PauliY {
    lane: usize,
}

impl PauliY {
    /// Create a new Pauli Y gate acting on the given lane.
    pub fn new(lane: usize) -> Self {
        Self { lane }
    }
}

impl QuantumGate for PauliY {
    fn rank(&self) -> u8 {
        1
    }

    fn span(&self) -> Span {
        Span::single(self.lane)
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        pauli_y_matrix()
    }
}

/// The Pauli Z gate, which adds a phase to the qubit.
///
/// It is represented by the matrix:
/// ```text
/// ┌      ┐
/// │ 1  0 │
/// │ 0 -1 │
/// └      ┘
/// ```
///
/// In a quantum circuit this can be represented by the gate `Z`
/// ```text
///    ┌───┐
/// ───┤ Z ├───
///    └───┘
/// ```
#[derive(Debug, Clone, Copy)]
pub struct PauliZ {
    lane: usize,
}

impl PauliZ {
    /// Create a new Pauli Z gate acting on the given lane.
    pub fn new(lane: usize) -> Self {
        Self { lane }
    }
}

impl QuantumGate for PauliZ {
    fn rank(&self) -> u8 {
        1
    }

    fn span(&self) -> Span {
        Span::single(self.lane)
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        pauli_z_matrix()
    }
}

/// The Hadamard gate, which puts the qubit in a superposition.
///
/// It is represented by the matrix:
/// ```text
/// ┌            ┐
/// │ 1/√2  1/√2 │
/// │ 1/√2 -1/√2 │
/// └            ┘
/// ```
///
/// In a quantum circuit this can be represented by the gate `H`
/// ```text
///    ┌───┐
/// ───┤ H ├───
///    └───┘
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Hadamard {
    lane: usize,
}

impl Hadamard {
    pub fn new(lane: usize) -> Self {
        Self { lane }
    }
}

impl QuantumGate for Hadamard {
    fn rank(&self) -> u8 {
        1
    }

    fn span(&self) -> Span {
        Span::single(self.lane)
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        hadamard_matrix()
    }
}

/// The phase gate, which adds a phase to the qubit.
/// This gate can accept any phase value.
///
/// It is represented by the matrix (where φ is the phase):
/// ```text
/// ┌        ┐
/// │ 1   0  │
/// │ 0 e^iφ │
/// └        ┘
/// ```
///
/// In a quantum circuit this can be represented by the gate `P(φ)`
/// ```text
///    ┌──────┐
/// ───┤ P(φ) ├───
///    └──────┘
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Phase {
    pub phase: f64,
    lane: usize,
}

impl Phase {
    pub fn new(phase: f64, lane: usize) -> Self {
        Self { phase, lane }
    }

    pub fn t(lane: usize) -> Self {
        Self::new(FRAC_PI_4, lane)
    }

    pub fn s(lane: usize) -> Self {
        Self::new(FRAC_PI_2, lane)
    }

    pub fn s_inv(lane: usize) -> Self {
        Self::new(-FRAC_PI_2, lane)
    }

    pub fn t_inv(lane: usize) -> Self {
        Self::new(-FRAC_PI_4, lane)
    }
}

impl QuantumGate for Phase {
    fn rank(&self) -> u8 {
        1
    }

    fn span(&self) -> Span {
        Span::single(self.lane)
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        phase_matrix(self.phase)
    }
}

/// The squared root of NOT gate, which is a generalization of the Hadamard gate.
/// It is represented by the matrix:
/// ```text
/// ┌                    ┐
/// │ 0.5+0.5i  0.5-0.5i │
/// │ 0.5-0.5i  0.5+0.5i │
/// └                    ┘
/// ```
///
/// In a quantum circuit this can be represented by the gate `SX`
/// ```text
///    ┌────┐
/// ───┤ SX ├───
///    └────┘
/// ```
#[derive(Debug, Clone, Copy)]
pub struct SX {
    lane: usize,
}

impl SX {
    /// Create a new squared root of NOT gate acting on the given lane.
    pub fn new(lane: usize) -> Self {
        Self { lane }
    }
}

impl QuantumGate for SX {
    fn rank(&self) -> u8 {
        1
    }

    fn span(&self) -> Span {
        Span::single(self.lane)
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        DMatrix::from_row_slice(
            2,
            2,
            &[0.5, -0.5, -0.5, 0.5]
                .iter()
                .map(|x| Complex::new(0.5, *x))
                .collect::<Vec<_>>(),
        )
    }
}

/// Rotates the qubit around the X axis.
/// It is represented by the matrix:
/// ```text
/// ┌                       ┐
/// │  cos(θ/2)  -isin(θ/2) │
/// │ -isin(θ/2)  cos(θ/2)  │
/// └                       ┘
/// ```
///
/// In a quantum circuit this can be represented by the gate `RX`
/// ```text
///    ┌────┐
/// ───┤ RX ├───
///    └────┘
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RX {
    lane: usize,
    theta: f64,
}

impl RX {
    /// Create a new rotation around the X axis gate acting on the given lane.
    pub fn new(theta: f64, lane: usize) -> Self {
        Self { lane, theta }
    }
}

impl QuantumGate for RX {
    fn rank(&self) -> u8 {
        1
    }

    fn span(&self) -> Span {
        Span::single(self.lane)
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        rotated_x_matrix(self.theta)
    }
}

/// Rotates the qubit around the Y axis.
/// It is represented by the matrix:
/// ```text
/// ┌                      ┐
/// │  cos(θ/2)  -sin(θ/2) │
/// │  sin(θ/2)   cos(θ/2) │
/// └                      ┘
/// ```
///
/// In a quantum circuit this can be represented by the gate `RY`
/// ```text
///    ┌────┐
/// ───┤ RY ├────
///    └────┘
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RY {
    lane: usize,
    theta: f64,
}

impl RY {
    /// Create a new rotation around the Y axis gate acting on the given lane.
    pub fn new(theta: f64, lane: usize) -> Self {
        Self { lane, theta }
    }
}

impl QuantumGate for RY {
    fn rank(&self) -> u8 {
        1
    }

    fn span(&self) -> Span {
        Span::single(self.lane)
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        rotated_y_matrix(self.theta)
    }
}

/// Rotates the qubit around the Z axis.
/// It is represented by the matrix:
/// ```text
/// ┌                      ┐
/// │  e^(-iθ/2)     0     │
/// │      0      e^(iθ/2) │
/// └                      ┘
/// ```
///
/// In a quantum circuit this can be represented by the gate `RZ`
/// ```text
///    ┌────┐
/// ───┤ RZ ├───
///    └────┘
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RZ {
    lane: usize,
    theta: f64,
}

impl RZ {
    /// Create a new rotation around the Z axis gate acting on the given lane.
    pub fn new(theta: f64, lane: usize) -> Self {
        Self { lane, theta }
    }
}

impl QuantumGate for RZ {
    fn rank(&self) -> u8 {
        1
    }

    fn span(&self) -> Span {
        Span::single(self.lane)
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        rotated_z_matrix(self.theta)
    }
}

/// The controlled X gate, which flips the target qubit if the control qubit is 1.
/// This gate acts on two qubits, the control and the target.
///
/// It is represented by the matrix:
/// ```text
/// ┌         ┐
/// │ 1 0 0 0 │
/// │ 0 1 0 0 │
/// │ 0 0 0 1 │
/// │ 0 0 1 0 │
/// └         ┘
/// ```
///
/// In a quantum circuit this can be represented by the gate `CX`
/// ```ascii
/// ─────@─────  (control)
///    ┌─┴─┐
/// ───┤ X ├─── (target)
///    └───┘
/// ```
#[derive(Debug, Clone, Copy)]
pub struct CX {
    control: usize,
    target: usize,
}

impl CX {
    /// Create a new controlled X gate with the given control and target qubits.
    pub fn new(control: usize, target: usize) -> Self {
        assert_ne!(control, target, "Control and target must be different");
        Self { control, target }
    }
}

impl QuantumGate for CX {
    fn rank(&self) -> u8 {
        2
    }

    fn span(&self) -> Span {
        Span::new([self.control, self.target])
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        controlled_gate_block(self.control, self.target, pauli_x_matrix())
    }
}

/// The controlled Y gate, which flips the target qubit if the control qubit is 1.
/// This gate acts on two qubits, the control and the target.
///
/// It is represented by the matrix:
/// ```text
/// ┌            ┐
/// │ 1  0  0  0 │
/// │ 0  1  0  0 │
/// │ 0  0  0 -i │
/// │ 0  0  i  0 │
/// └            ┘
/// ```
///
/// In a quantum circuit this can be represented by the gate `CY`
/// ```ascii
/// ─────@─────  (control)
///    ┌─┴─┐
/// ───┤ Y ├───  (target)
///    └───┘
/// ```
#[derive(Debug, Clone, Copy)]
pub struct CY {
    control: usize,
    target: usize,
}

impl CY {
    /// Create a new controlled Y gate with the given control and target qubits.
    pub fn new(control: usize, target: usize) -> Self {
        Self { control, target }
    }
}

impl QuantumGate for CY {
    fn rank(&self) -> u8 {
        2
    }

    fn span(&self) -> Span {
        Span::new([self.control, self.target])
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        controlled_gate_block(self.control, self.target, pauli_y_matrix())
    }
}

/// The controlled Z gate, which adds a phase to the target qubit if the control qubit is 1.
/// This gate acts on two qubits, the control and the target.
///
/// It is represented by the matrix:
/// ```text
/// ┌             ┐
/// │ 1  0  0  0  │
/// │ 0  1  0  0  │
/// │ 0  0  1  0  │
/// │ 0  0  0 -1  │
/// └             ┘
/// ```
///
/// In a quantum circuit this can be represented by the gate `CZ`
/// ```ascii
/// ─────@─────  (control)
///    ┌─┴─┐
/// ───┤ Z ├───  (target)
///    └───┘
/// ```
#[derive(Debug, Clone, Copy)]
pub struct CZ {
    control: usize,
    target: usize,
}

impl CZ {
    /// Create a new controlled Z gate with the given control and target qubits.
    pub fn new(control: usize, target: usize) -> Self {
        Self { control, target }
    }
}

impl QuantumGate for CZ {
    fn rank(&self) -> u8 {
        2
    }

    fn span(&self) -> Span {
        Span::new([self.control, self.target])
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        controlled_gate_block(self.control, self.target, pauli_z_matrix())
    }
}

/// The controlled phase gate, which adds a phase to the target qubit if the control qubit is 1.
/// This gate acts on two qubits, the control and the target.
///
/// It is represented by the matrix:
/// ```text
/// ┌                     ┐
/// │ 1    0    0    0    │
/// │ 0    1    0    0    │
/// │ 0    0    1    0    │
/// │ 0    0    0    e^iφ │
/// └                     ┘
/// ```
///
/// In a quantum circuit this can be represented by the gate `CP`
/// ```ascii
/// ───────@───────  (control)
///    ┌───┴───┐
/// ───┤ P(φ)  ├───  (target)
///    └───────┘
/// ```
/// where φ is the phase.
#[derive(Debug, Clone, Copy)]
pub struct CP {
    control: usize,
    target: usize,
    phase: f64,
}

impl CP {
    /// Create a new controlled phase gate with the given control and target qubits.
    pub fn new(phase: f64, control: usize, target: usize) -> Self {
        Self {
            control,
            target,
            phase,
        }
    }
}

impl QuantumGate for CP {
    fn rank(&self) -> u8 {
        2
    }

    fn span(&self) -> Span {
        Span::new([self.control, self.target])
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        controlled_gate_block(self.control, self.target, phase_matrix(self.phase))
    }
}

/// Controlled rotation around the X axis.
/// It is represented by the matrix:
/// ```text
/// ┌                                       ┐
/// │ 1         0          0          0     │
/// │ 0         1          0          0     │
/// │ 0         0      cos(θ/2)  -isin(θ/2) │
/// │ 0         0     -isin(θ/2)  cos(θ/2)  │
/// └                                       ┘
/// ```
///
/// In a quantum circuit this can be represented by the gate `CRX`
/// ```text
/// ───────@───────  (control)
///    ┌───┴───┐
/// ───┤ RX(θ) ├───  (target)
///    └───────┘
/// ```
/// where θ is the angle of rotation.
#[derive(Debug, Clone, Copy)]
pub struct CRX {
    control: usize,
    target: usize,
    theta: f64,
}

impl CRX {
    /// Create a new controlled rotation around the X axis gate acting on the given lane.
    pub fn new(theta: f64, control: usize, target: usize) -> Self {
        Self {
            control,
            target,
            theta,
        }
    }
}

impl QuantumGate for CRX {
    fn rank(&self) -> u8 {
        2
    }

    fn span(&self) -> Span {
        Span::new([self.control, self.target])
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        controlled_gate_block(self.control, self.target, rotated_x_matrix(self.theta))
    }
}

/// Controlled rotation around the Y axis.
/// It is represented by the matrix:
/// ```text
/// ┌                                       ┐
/// │ 1         0          0          0     │
/// │ 0         1          0          0     │
/// │ 0         0      cos(θ/2)  -sin(θ/2)  │
/// │ 0         0      sin(θ/2)   cos(θ/2)  │
/// └                                       ┘
/// ```
///
/// In a quantum circuit this can be represented by the gate `CRY`
/// ```text
/// ───────@───────  (control)
///    ┌───┴───┐
/// ───┤ RY(θ) ├───  (target)
///    └───────┘
/// ```
/// where θ is the angle of rotation.
#[derive(Debug, Clone, Copy)]
pub struct CRY {
    control: usize,
    target: usize,
    theta: f64,
}

impl CRY {
    /// Create a new controlled rotation around the Y axis gate acting on the given lane.
    pub fn new(theta: f64, control: usize, target: usize) -> Self {
        Self {
            control,
            target,
            theta,
        }
    }
}

impl QuantumGate for CRY {
    fn rank(&self) -> u8 {
        2
    }

    fn span(&self) -> Span {
        Span::new([self.control, self.target])
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        controlled_gate_block(self.control, self.target, rotated_y_matrix(self.theta))
    }
}

/// Controlled rotation around the Z axis.
/// It is represented by the matrix:
/// ```text
/// ┌                                  ┐
/// │ 1       0         0        0     │
/// │ 0       1         0        0     │
/// │ 0       0    e^(-iθ/2)     0     │
/// │ 0       0         0     e^(iθ/2) │
/// └                                  ┘
/// ```
///
/// In a quantum circuit this can be represented by the gate `CRZ`
/// ```text
/// ───────@───────  (control)
///    ┌───┴───┐
/// ───┤ RZ(θ) ├───  (target)
///    └───────┘
/// ```
/// where θ is the angle of rotation.
#[derive(Debug, Clone, Copy)]
pub struct CRZ {
    control: usize,
    target: usize,
    theta: f64,
}

impl CRZ {
    /// Create a new controlled rotation around the Z axis gate acting on the given lane.
    pub fn new(theta: f64, control: usize, target: usize) -> Self {
        Self {
            control,
            target,
            theta,
        }
    }
}

impl QuantumGate for CRZ {
    fn rank(&self) -> u8 {
        2
    }

    fn span(&self) -> Span {
        Span::new([self.control, self.target])
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        controlled_gate_block(self.control, self.target, rotated_z_matrix(self.theta))
    }
}

/// Controlled Hadamard gate.
///
/// It is represented by the matrix:
/// ```text
/// ┌                    ┐
/// │ 1    0    0    0   │
/// │ 0    1    0    0   │
/// │ 0    0  1/√2  1/√2 │
/// │ 0    0  1/√2 -1/√2 │
/// └                    ┘
/// ```
///
/// In a quantum circuit this can be represented by the gate `CH`
/// ```text
/// ─────@─────  (control)
///    ┌─┴─┐
/// ───┤ H ├───  (target)
///    └───┘
/// ```
#[derive(Debug, Clone, Copy)]
pub struct CH {
    control: usize,
    target: usize,
}

impl CH {
    /// Create a new controlled Hadamard gate acting on the given control and target qubits.
    pub fn new(control: usize, target: usize) -> Self {
        Self { control, target }
    }
}

impl QuantumGate for CH {
    fn rank(&self) -> u8 {
        2
    }

    fn span(&self) -> Span {
        Span::new([self.control, self.target])
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        controlled_gate_block(self.control, self.target, hadamard_matrix())
    }
}

/// The swap gate, which swaps the states of two qubits.
/// This gate acts on two qubits.
///
/// It is represented by the matrix:
/// ```text
/// ┌         ┐
/// │ 1 0 0 0 │
/// │ 0 0 1 0 │
/// │ 0 1 0 0 │
/// │ 0 0 0 1 │
/// └         ┘
/// ```
///
/// In a quantum circuit this can be represented by the gate `SWAP`
/// ```ascii
/// ─────X─────
///      │
/// ─────X─────
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Swap {
    lanes: (usize, usize),
}

impl Swap {
    /// Create a new swap gate with the given control and target qubits.
    pub fn new(lane1: usize, lane2: usize) -> Self {
        Self {
            lanes: (lane1, lane2),
        }
    }
}

impl QuantumGate for Swap {
    fn rank(&self) -> u8 {
        2
    }

    fn span(&self) -> Span {
        Span::new([self.lanes.0, self.lanes.1])
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        let uninvolved = usize::abs_diff(self.lanes.0, self.lanes.1) - 1;
        let mut a00 = Qubit::zero().ketbra();
        a00 = tensor_expand(a00, uninvolved);
        a00 = a00.tensor_product(Qubit::zero().ketbra());

        let mut a01 = Qubit::zero().ket() * Qubit::one().bra();
        a01 = tensor_expand(a01, uninvolved);
        a01 = a01.tensor_product(Qubit::one().ket() * Qubit::zero().bra());

        let mut a10 = Qubit::one().ket() * Qubit::zero().bra();
        a10 = tensor_expand(a10, uninvolved);
        a10 = a10.tensor_product(Qubit::zero().ket() * Qubit::one().bra());

        let mut a11 = Qubit::one().ketbra();
        a11 = tensor_expand(a11, uninvolved);
        a11 = a11.tensor_product(Qubit::one().ketbra());

        a00 + a01 + a10 + a11
    }
}

/// The Toffoli gate, which flips the target qubit if both control qubits are 1.
/// This gate acts on three qubits, two control qubits and one target qubit.
/// It is also known as the CCX gate.
///
/// It is represented by the matrix:
/// ```text
/// ┌                 ┐
/// │ 1 0 0 0 0 0 0 0 │
/// │ 0 1 0 0 0 0 0 0 │
/// │ 0 0 1 0 0 0 0 0 │
/// │ 0 0 0 1 0 0 0 0 │
/// │ 0 0 0 0 1 0 0 0 │
/// │ 0 0 0 0 0 1 0 0 │
/// │ 0 0 0 0 0 0 0 1 │
/// │ 0 0 0 0 0 0 1 0 │
/// └                 ┘
/// ```
///
/// In a quantum circuit this can be represented by the gate `CCX`
/// ```ascii
/// ─────@───── (control 1)
///      │
/// ─────@───── (control 2)
///    ┌─┴─┐
/// ───┤ X ├─── (target)
///    └───┘
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Toffoli {
    control: (usize, usize),
    target: usize,
}

impl Toffoli {
    /// Create a new Toffoli gate with the given control and target qubits.
    pub fn new(control: (usize, usize), target: usize) -> Self {
        Self { control, target }
    }
}

impl QuantumGate for Toffoli {
    fn rank(&self) -> u8 {
        3
    }

    fn span(&self) -> Span {
        Span::new([self.control.0, self.control.1, self.target])
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        let lanes = [self.control.0, self.control.1, self.target];
        let start = lanes.iter().min().unwrap();
        let end = lanes.iter().max().unwrap();

        let mut res = Block::empty(2_usize.pow((end - start + 1) as u32)).into_matrix();

        // case control <- 0* || *0
        let cases = [
            (Qubit::zero(), Qubit::zero()),
            (Qubit::zero(), Qubit::one()),
            (Qubit::one(), Qubit::zero()),
        ];
        for (c0, c1) in cases.into_iter() {
            let mut a = Block::one().into_matrix();
            for i in *start..=*end {
                a = match i {
                    _ if i == self.control.0 => a.tensor_product(c0.ketbra()),
                    _ if i == self.control.1 => a.tensor_product(c1.ketbra()),
                    _ if i == self.target => a.tensor_product(Block::identity(2)),
                    _ => a.tensor_product(Block::identity(2)),
                };
            }
            res += a;
        }

        // case control <- 11
        let mut a = Block::one().into_matrix();
        for i in *start..=*end {
            a = match i {
                _ if i == self.control.0 => a.tensor_product(Qubit::one().ketbra()),
                _ if i == self.control.1 => a.tensor_product(Qubit::one().ketbra()),
                _ if i == self.target => a.tensor_product(pauli_x_matrix()),
                _ => a.tensor_product(Block::identity(2)),
            };
        }

        res + a
    }
}

/// The Fredkin gate, which swaps the target and control qubits if the control qubit is 1.
/// This gate acts on three qubits, one control qubit and two target qubits.
/// It is also known as the CSWAP gate.
///
/// It is represented by the matrix:
/// ```text
/// ┌                 ┐
/// │ 1 0 0 0 0 0 0 0 │
/// │ 0 1 0 0 0 0 0 0 │
/// │ 0 0 1 0 0 0 0 0 │
/// │ 0 0 0 1 0 0 0 0 │
/// │ 0 0 0 0 1 0 0 0 │
/// │ 0 0 0 0 0 0 1 0 │
/// │ 0 0 0 0 0 1 0 0 │
/// │ 0 0 0 0 0 0 0 1 │
/// └                 ┘
/// ```
///
/// In a quantum circuit this can be represented by the gate `CSWAP`
/// ```ascii
/// ─────@───── (control)
///      │
/// ─────X───── (target 1)
///      │
/// ─────X───── (target 2)
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Fredkin {
    control: usize,
    target: (usize, usize),
}

impl Fredkin {
    /// Create a new Fredkin gate with the given control and target qubits.
    pub fn new(control: usize, target: (usize, usize)) -> Self {
        Self { control, target }
    }
}

impl QuantumGate for Fredkin {
    fn rank(&self) -> u8 {
        3
    }

    fn span(&self) -> Span {
        Span::new([self.control, self.target.0, self.target.1])
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        let lanes = [self.control, self.target.0, self.target.1];
        let start = lanes.iter().min().unwrap();
        let end = lanes.iter().max().unwrap();

        let mut res = Block::empty(2_usize.pow((end - start + 1) as u32)).into_matrix();

        // case control <- 0
        let mut c = Block::one().into_matrix();
        for i in *start..=*end {
            c = match i {
                _ if i == self.control => c.tensor_product(Qubit::zero().ketbra()),
                _ if i == self.target.0 => c.tensor_product(Block::identity(2)),
                _ if i == self.target.1 => c.tensor_product(Block::identity(2)),
                _ => c.tensor_product(Block::identity(2)),
            };
        }
        res += c;

        // case control <- 1
        let cases = [
            (Qubit::zero(), Qubit::zero()),
            (Qubit::zero(), Qubit::one()),
            (Qubit::one(), Qubit::zero()),
            (Qubit::one(), Qubit::one()),
        ];
        for (a, b) in cases.into_iter() {
            let mut c = Block::one().into_matrix();
            for i in *start..=*end {
                c = match i {
                    _ if i == self.control => c.tensor_product(Qubit::one().ketbra()),
                    _ if i == self.target.0 => c.tensor_product(a.ket() * b.bra()),
                    _ if i == self.target.1 => c.tensor_product(b.ket() * a.bra()),
                    _ => c.tensor_product(Block::identity(2)),
                };
            }
            res += c;
        }

        res
    }
}

/// The Universal gate, which is described by a matrix and three parameters (θ, ϕ, λ).
///
/// It is represented by the matrix:
/// ```text
/// ┌                                               ┐
/// │    0.5(1+e^(iθ))     -0.5ie^(iλ)(1-e^(iθ))    │
/// │ 0.5ie^(iϕ)(1-e^(iθ))  0.5e^(i(ϕ+λ))(1+e^(iθ)) │
/// └                                               ┘
/// ```
///
/// In a quantum circuit this can be represented by the gate `U`
/// ```text
///    ┌────────────┐
/// ───┤ U(θ, ϕ, λ) ├───
///    └────────────┘
/// ```
/// where θ, ϕ, and λ are the parameters.
#[derive(Debug, Clone, Copy)]
pub struct U {
    lane: usize,
    theta: f64,
    phi: f64,
    lambda: f64,
}

impl U {
    /// Create a new universal gate acting on the given lane with the given parameters.
    pub fn new(theta: f64, phi: f64, lambda: f64, lane: usize) -> Self {
        Self {
            lane,
            theta,
            phi,
            lambda,
        }
    }
}

impl QuantumGate for U {
    fn rank(&self) -> u8 {
        1
    }

    fn span(&self) -> Span {
        Span::single(self.lane)
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        universal_matrix(self.theta, self.phi, self.lambda)
    }
}

/// IBM Quantum Experience gate 1
///
/// It is represented by following combination of gates:
/// ```ascii
///    ┌────────────┐
/// ───┤ U(0, 0, λ) ├───
///    └────────────┘
/// ```
/// where λ is the parameter.
#[derive(Debug, Clone, Copy)]
pub struct U1 {
    lane: usize,
    lambda: f64,
}

impl U1 {
    /// Create a new U1 gate acting on the given lane with the given parameter.
    pub fn new(lambda: f64, lane: usize) -> Self {
        Self { lane, lambda }
    }
}

impl QuantumGate for U1 {
    fn rank(&self) -> u8 {
        1
    }

    fn span(&self) -> Span {
        Span::single(self.lane)
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        universal_matrix(0.0, 0.0, self.lambda)
    }
}

/// IBM Quantum Experience gate 2
/// It is represented by following combination of gates:
/// ```ascii
///    ┌──────────────────┐   ┌──────────────┐
/// ───┤ Ph(-(φ+λ+π/2)/2) ├───┤ U(π/2, ϕ, λ) ├───
///    └──────────────────┘   └──────────────┘
/// ```
/// where φ, ϕ, and λ are the parameters.
#[derive(Debug, Clone, Copy)]
pub struct U2 {
    lane: usize,
    phi: f64,
    lambda: f64,
}

impl U2 {
    /// Create a new U2 gate acting on the given lane with the given parameters.
    pub fn new(phi: f64, lambda: f64, lane: usize) -> Self {
        Self { lane, phi, lambda }
    }
}

impl QuantumGate for U2 {
    fn rank(&self) -> u8 {
        1
    }

    fn span(&self) -> Span {
        Span::single(self.lane)
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        let phase = -(self.phi + self.lambda + FRAC_PI_2) / 2.0;
        gphase_matrix(phase, 2) * universal_matrix(FRAC_PI_2, self.phi, self.lambda)
    }
}

/// IBM Quantum Experience gate 3
/// It is represented by following combination of gates:
/// ```ascii
///    ┌────────────────┐   ┌────────────┐
/// ───┤ Ph(-(φ+λ+θ)/2) ├───┤ U(θ, ϕ, λ) ├───
///    └────────────────┘   └────────────┘
/// ```
/// where θ, φ, ϕ, and λ are the parameters.
#[derive(Debug, Clone, Copy)]
pub struct U3 {
    lane: usize,
    theta: f64,
    phi: f64,
    lambda: f64,
}

impl U3 {
    /// Create a new U3 gate acting on the given lane with the given parameters.
    pub fn new(theta: f64, phi: f64, lambda: f64, lane: usize) -> Self {
        Self {
            lane,
            theta,
            phi,
            lambda,
        }
    }
}

impl QuantumGate for U3 {
    fn rank(&self) -> u8 {
        1
    }

    fn span(&self) -> Span {
        Span::single(self.lane)
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        let phase = -(self.phi + self.lambda + self.theta) / 2.0;
        gphase_matrix(phase, 2) * universal_matrix(self.theta, self.phi, self.lambda)
    }
}

/// The four parameter controlled-U gate with relative phase γ
///
/// It is represented by the combination of gates:
/// ```ascii
///    ┌──────────┐
/// ───┤ P(γ-θ/2) ├──────────@─────────
///    └──────────┘          │
///                   ┌──────┴─────┐
/// ──────────────────┤ U(θ, ϕ, λ) ├───
///                   └────────────┘
/// ```
/// where θ, φ, ϕ, λ, and γ are the parameters.
#[derive(Debug, Clone, Copy)]
pub struct CU {
    control: usize,
    target: usize,
    theta: f64,
    phi: f64,
    lambda: f64,
    gamma: f64,
}

impl CU {
    /// Create a new controlled-U gate acting on the given control and target qubits with the given parameters.
    pub fn new(
        theta: f64,
        phi: f64,
        lambda: f64,
        gamma: f64,
        control: usize,
        target: usize,
    ) -> Self {
        Self {
            control,
            target,
            theta,
            phi,
            lambda,
            gamma,
        }
    }
}

impl QuantumGate for CU {
    fn rank(&self) -> u8 {
        2
    }

    fn span(&self) -> Span {
        Span::new([self.control, self.target])
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        let mut a = Block::one().into_matrix();
        let start = usize::min(self.control, self.target);
        let end = usize::max(self.control, self.target);

        for i in start..=end {
            a = match i {
                _ if i == self.control => {
                    a.tensor_product(phase_matrix(self.gamma - self.theta / 2.0))
                }
                _ if i == self.target => a.tensor_product(Block::identity(2)),
                _ => a.tensor_product(Block::identity(2)),
            };
        }

        let b = controlled_gate_block(
            self.control,
            self.target,
            universal_matrix(self.theta, self.phi, self.lambda),
        );

        a * b
    }
}

/// See [this link](https://quantumcomputing.stackexchange.com/questions/4252/how-to-derive-the-cnot-matrix-for-a-3-qubit-system-where-the-control-target-qu)
/// for more information on how to derive this matrix.
fn controlled_gate_block(
    control: usize,
    target: usize,
    gate: DMatrix<Complex<f64>>,
) -> DMatrix<Complex<f64>> {
    assert_ne!(control, target, "Control and target must be different");
    let start = usize::min(control, target);
    let end = usize::max(control, target);

    // case control <- 0
    let mut a0 = Block::one().into_matrix();
    for i in start..=end {
        a0 = match i {
            _ if i == control => a0.tensor_product(Qubit::zero().ketbra()),
            _ if i == target => a0.tensor_product(Block::identity(2)),
            _ => a0.tensor_product(Block::identity(2)),
        };
    }

    // case control <- 1
    let mut a1 = Block::one().into_matrix();
    for i in start..=end {
        a1 = match i {
            _ if i == control => a1.tensor_product(Qubit::one().ketbra()),
            _ if i == target => a1.tensor_product(gate.clone()),
            _ => a1.tensor_product(Block::identity(2)),
        };
    }

    a0 + a1
}

fn pauli_x_matrix() -> DMatrix<Complex<f64>> {
    DMatrix::from_row_slice(2, 2, &[0.0, 1.0, 1.0, 0.0]).map(|x| Complex::new(x, 0.0))
}

fn pauli_y_matrix() -> DMatrix<Complex<f64>> {
    DMatrix::from_row_slice(2, 2, &[0.0, -1.0, 1.0, 0.0]).map(|x| Complex::new(0.0, x))
}

fn pauli_z_matrix() -> DMatrix<Complex<f64>> {
    DMatrix::from_row_slice(2, 2, &[1.0, 0.0, 0.0, -1.0]).map(|x| Complex::new(x, 0.0))
}

fn phase_matrix(phase: f64) -> DMatrix<Complex<f64>> {
    DMatrix::from_row_slice(
        2,
        2,
        &[1.0.into(), 0.0.into(), 0.0.into(), Complex::cis(phase)],
    )
}

fn hadamard_matrix() -> DMatrix<Complex<f64>> {
    DMatrix::from_row_slice(2, 2, &[1.0, 1.0, 1.0, -1.0])
        .map(|x| Complex::new(x, 0.0) / 2.0_f64.sqrt())
}

fn rotated_x_matrix(theta: f64) -> DMatrix<Complex<f64>> {
    let cos = Complex::new((theta / 2.0).cos(), 0.0);
    let sin = Complex::new(0.0, -(theta / 2.0).sin());
    DMatrix::from_row_slice(2, 2, &[cos, sin, sin, cos])
}

fn rotated_y_matrix(theta: f64) -> DMatrix<Complex<f64>> {
    let cos = Complex::new((theta / 2.0).cos(), 0.0);
    let sin = Complex::new((theta / 2.0).sin(), 0.0);
    DMatrix::from_row_slice(2, 2, &[cos, -sin, sin, cos])
}

fn rotated_z_matrix(theta: f64) -> DMatrix<Complex<f64>> {
    let e1 = Complex::cis(-theta / 2.0);
    let e2 = Complex::cis(theta / 2.0);
    DMatrix::from_row_slice(2, 2, &[e1, 0.0.into(), 0.0.into(), e2])
}

fn universal_matrix(theta: f64, phi: f64, lambda: f64) -> DMatrix<Complex<f64>> {
    let a = 0.5 * (1.0 + Complex::cis(theta));
    let b = -0.5 * Complex::i() * Complex::cis(lambda) * (1.0 - Complex::cis(theta));
    let c = 0.5 * Complex::i() * Complex::cis(phi) * (1.0 - Complex::cis(theta));
    let d = 0.5 * Complex::cis(phi + lambda) * (1.0 + Complex::cis(theta));

    DMatrix::from_row_slice(2, 2, &[a, b, c, d])
}

fn gphase_matrix(phase: f64, dim: usize) -> DMatrix<Complex<f64>> {
    let phase = Complex::cis(phase);
    let id = Block::identity(dim).into_matrix();
    id * phase
}

fn tensor_expand(mut val: DMatrix<Complex<f64>>, uninvolved: usize) -> DMatrix<Complex<f64>> {
    for _ in 0..uninvolved {
        val = val.tensor_product(Block::identity(2));
    }
    val
}
