use std::f64::consts::FRAC_PI_4;

use enum_dispatch::enum_dispatch;
use nalgebra::{Complex, DMatrix};

use super::Block;

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
