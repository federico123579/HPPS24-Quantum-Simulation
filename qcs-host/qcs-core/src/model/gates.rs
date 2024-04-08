use std::f64::consts::FRAC_PI_4;

use enum_dispatch::enum_dispatch;
use nalgebra::{Complex, DMatrix};

use super::Block;

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

#[enum_dispatch(QuantumGate)]
#[derive(Debug, Clone, Copy)]
pub enum Gate {
    Identity,
    PauliX,
    PauliY,
    PauliZ,
    Hadamard,
    Phase,
    Pi8,
    CNOTup,
    CNOTdown,
    ConZ,
    Swap,
    Toffoli,
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
pub struct Identity;

impl QuantumGate for Identity {
    fn rank(&self) -> u8 {
        1
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        DMatrix::identity(2, 2)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PauliX;

impl QuantumGate for PauliX {
    fn rank(&self) -> u8 {
        1
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        DMatrix::from_row_slice(2, 2, &[0.0, 1.0, 1.0, 0.0]).map(|x| Complex::new(x, 0.0))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PauliY;

impl QuantumGate for PauliY {
    fn rank(&self) -> u8 {
        1
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        DMatrix::from_row_slice(2, 2, &[0.0, -1.0, 1.0, 0.0]).map(|x| Complex::new(x, 0.0))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PauliZ;

impl QuantumGate for PauliZ {
    fn rank(&self) -> u8 {
        1
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        DMatrix::from_row_slice(2, 2, &[1.0, 0.0, 0.0, -1.0]).map(|x| Complex::new(x, 0.0))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hadamard;

impl QuantumGate for Hadamard {
    fn rank(&self) -> u8 {
        1
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        DMatrix::from_row_slice(2, 2, &[1.0, 1.0, 1.0, -1.0])
            .map(|x| Complex::new(x, 0.0) / 2.0_f64.sqrt())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Phase;

impl QuantumGate for Phase {
    fn rank(&self) -> u8 {
        1
    }

    fn matrix(&self) -> DMatrix<Complex<f64>> {
        DMatrix::from_row_slice(2, 2, &[1.0.into(), 0.0.into(), 0.0.into(), Complex::i()])
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pi8;

impl QuantumGate for Pi8 {
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

#[derive(Debug, Clone, Copy)]
pub struct CNOTup;

impl QuantumGate for CNOTup {
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

#[derive(Debug, Clone, Copy)]
pub struct CNOTdown;

impl QuantumGate for CNOTdown {
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

#[derive(Debug, Clone, Copy)]
pub struct ConZ;

impl QuantumGate for ConZ {
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

#[derive(Debug, Clone, Copy)]
pub struct Swap;

impl QuantumGate for Swap {
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

#[derive(Debug, Clone, Copy)]
pub struct Toffoli;

impl QuantumGate for Toffoli {
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
