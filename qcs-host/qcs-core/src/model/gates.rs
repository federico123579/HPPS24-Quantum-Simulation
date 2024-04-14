use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

use enum_dispatch::enum_dispatch;
use nalgebra::{Complex, DMatrix};

use super::{span::Span, Block, Braket, Qubit, TensorProduct};

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
}

#[enum_dispatch(QuantumGate)]
#[derive(Debug, Clone, Copy)]
pub enum Gate {
    Identity,
    PauliX,
    PauliY,
    PauliZ,
    Hadamard,
    Phase,
    CX,
    CY,
    CZ,
    Swap,
    Toffoli,
}

impl Gate {
    pub fn is_rank_one(&self) -> bool {
        match self {
            Gate::Identity(_)
            | Gate::PauliX(_)
            | Gate::PauliY(_)
            | Gate::PauliZ(_)
            | Gate::Hadamard(_)
            | Gate::Phase(_) => true,
            Gate::CX(_) | Gate::CY(_) | Gate::CZ(_) | Gate::Swap(_) | Gate::Toffoli(_) => false,
        }
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
            Gate::Phase(p) => write!(f, "P({})[{}]", p.phase, p.lane),
            Gate::CX(cx) => write!(f, "CX[{},{}]", cx.control, cx.target),
            Gate::CY(cy) => write!(f, "CY[{},{}]", cy.control, cy.target),
            Gate::CZ(cz) => write!(f, "CZ[{},{}]", cz.control, cz.target),
            Gate::Swap(s) => write!(f, "SWAP[{}, {}]", s.lanes.0, s.lanes.1),
            Gate::Toffoli(t) => write!(f, "CCX[{},{},{}]", t.control.0, t.control.1, t.target),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Identity {
    lane: usize,
}

impl Identity {
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

#[derive(Debug, Clone, Copy)]
pub struct PauliX {
    lane: usize,
}

impl PauliX {
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
        pauli_x_block()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PauliY {
    lane: usize,
}

impl PauliY {
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
        pauli_y_block()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PauliZ {
    lane: usize,
}

impl PauliZ {
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
        pauli_z_block()
    }
}

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
        DMatrix::from_row_slice(2, 2, &[1.0, 1.0, 1.0, -1.0])
            .map(|x| Complex::new(x, 0.0) / 2.0_f64.sqrt())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Phase {
    pub phase: f64,
    lane: usize,
}

impl Phase {
    pub fn new(phase: f64, lane: usize) -> Self {
        Self { phase, lane }
    }

    pub fn pi8(lane: usize) -> Self {
        Self::new(FRAC_PI_4, lane)
    }

    pub fn s(lane: usize) -> Self {
        Self::new(FRAC_PI_2, lane)
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
        DMatrix::from_row_slice(
            2,
            2,
            &[1.0.into(), 0.0.into(), 0.0.into(), Complex::cis(self.phase)],
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CX {
    control: usize,
    target: usize,
}

impl CX {
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
        controlled_gate_block(self.control, self.target, pauli_x_block())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CY {
    control: usize,
    target: usize,
}

impl CY {
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
        controlled_gate_block(self.control, self.target, pauli_y_block())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CZ {
    control: usize,
    target: usize,
}

impl CZ {
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
        controlled_gate_block(self.control, self.target, pauli_z_block())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Swap {
    lanes: (usize, usize),
}

impl Swap {
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

#[derive(Debug, Clone, Copy)]
pub struct Toffoli {
    control: (usize, usize),
    target: usize,
}

impl Toffoli {
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

        let mut res = Block::empty(8).into_matrix();

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
                _ if i == self.target => a.tensor_product(pauli_x_block()),
                _ => a.tensor_product(Block::identity(2)),
            };
        }

        res + a
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

fn pauli_x_block() -> DMatrix<Complex<f64>> {
    DMatrix::from_row_slice(2, 2, &[0.0, 1.0, 1.0, 0.0]).map(|x| Complex::new(x, 0.0))
}

fn pauli_y_block() -> DMatrix<Complex<f64>> {
    DMatrix::from_row_slice(2, 2, &[0.0, -1.0, 1.0, 0.0]).map(|x| Complex::new(0.0, x))
}

fn pauli_z_block() -> DMatrix<Complex<f64>> {
    DMatrix::from_row_slice(2, 2, &[1.0, 0.0, 0.0, -1.0]).map(|x| Complex::new(x, 0.0))
}

fn tensor_expand(mut val: DMatrix<Complex<f64>>, uninvolved: usize) -> DMatrix<Complex<f64>> {
    for _ in 0..uninvolved {
        val = val.tensor_product(Block::identity(2));
    }
    val
}
