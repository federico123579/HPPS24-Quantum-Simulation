use std::f64::consts::FRAC_PI_4;

use nalgebra::{Complex, DMatrix, DVector, Vector2};

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

#[derive(Debug, Clone, Copy, Default)]
pub enum GateKind {
    #[default]
    Identity,
    /// Pauli-X Gate (X)
    PauliX,
    /// Pauli-Y Gate (Y)
    PauliY,
    /// Pauli-Z Gate (Z)
    PauliZ,
    /// Hadamard Gate (H)
    Hadamard,
    /// Phase Gate (S, P)
    Phase,
    /// Pi/8 Gate (T)
    Pi8,
    /// Controlled-NOT Gate (CNOT, CX, CNOT) - controlled is up
    CNOTup,
    /// Controlled-NOT Gate (CNOT, CX, CNOT) - controlled is down
    CNOTdown,
    /// Controlled-Z Gate (CZ)
    ConZ,
    /// Swap Gate (SWAP)
    Swap,
    /// Toffoli Gate (CCNOT, CCX, TOFF)
    Toffoli,
}

impl GateKind {
    pub fn into_block(self) -> Block {
        Block::from(self)
    }
}

// @@@@@@@@@@@@@@@@@
// @@ COMPUTATION @@
// @@@@@@@@@@@@@@@@@

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    matrix_repr: DMatrix<Complex<f64>>,
    dim: usize,
}

impl Block {
    pub fn tensor_product(&self, rhs: &Block) -> Block {
        Block {
            matrix_repr: self.as_ref().kronecker(rhs.as_ref()),
            dim: self.dim * rhs.dim,
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

impl From<GateKind> for Block {
    fn from(gate: GateKind) -> Self {
        match gate {
            GateKind::Identity => Block {
                matrix_repr: DMatrix::identity(2, 2),
                dim: 2,
            },
            GateKind::PauliX => Block {
                matrix_repr: DMatrix::from_row_slice(2, 2, &[0.0, 1.0, 1.0, 0.0])
                    .map(|x| Complex::new(x, 0.0)),
                dim: 2,
            },
            GateKind::PauliY => {
                let pauli_y = DMatrix::from_row_slice(
                    2,
                    2,
                    &[0.0.into(), -Complex::i(), Complex::i(), 0.0.into()],
                );
                Block {
                    matrix_repr: pauli_y,
                    dim: 2,
                }
            }
            GateKind::PauliZ => {
                let pauli_z = DMatrix::from_row_slice(2, 2, &[1.0, 0.0, 0.0, -1.0])
                    .map(|x| Complex::new(x, 0.0));
                Block {
                    matrix_repr: pauli_z,
                    dim: 2,
                }
            }
            GateKind::Hadamard => Block {
                matrix_repr: DMatrix::from_row_slice(
                    2,
                    2,
                    &[
                        1.0 / 2.0_f64.sqrt(),
                        1.0 / 2.0_f64.sqrt(),
                        1.0 / 2.0_f64.sqrt(),
                        -1.0 / 2.0_f64.sqrt(),
                    ],
                )
                .map(|x| Complex::new(x, 0.0)),
                dim: 2,
            },
            GateKind::Phase => {
                let phase = DMatrix::from_row_slice(
                    2,
                    2,
                    &[1.0.into(), 0.0.into(), 0.0.into(), Complex::i()],
                );
                Block {
                    matrix_repr: phase,
                    dim: 2,
                }
            }
            GateKind::Pi8 => {
                let pi8 = DMatrix::from_row_slice(
                    2,
                    2,
                    &[1.0.into(), 0.0.into(), 0.0.into(), Complex::cis(FRAC_PI_4)],
                );
                Block {
                    matrix_repr: pi8,
                    dim: 2,
                }
            }
            GateKind::CNOTup => {
                let cnot = DMatrix::from_row_slice(
                    4,
                    4,
                    &[
                        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
                        0.0,
                    ],
                )
                .map(|x| Complex::new(x, 0.0));
                Block {
                    matrix_repr: cnot,
                    dim: 4,
                }
            }
            GateKind::CNOTdown => {
                let cnot = DMatrix::from_row_slice(
                    4,
                    4,
                    &[
                        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
                        0.0,
                    ],
                )
                .map(|x| Complex::new(x, 0.0));
                Block {
                    matrix_repr: cnot,
                    dim: 4,
                }
            }
            GateKind::ConZ => {
                let cz = DMatrix::from_row_slice(
                    4,
                    4,
                    &[
                        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                        -1.0,
                    ],
                )
                .map(|x| Complex::new(x, 0.0));
                Block {
                    matrix_repr: cz,
                    dim: 4,
                }
            }
            GateKind::Swap => {
                let swap = DMatrix::from_row_slice(
                    4,
                    4,
                    &[
                        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                        1.0,
                    ],
                )
                .map(|x| Complex::new(x, 0.0));
                Block {
                    matrix_repr: swap,
                    dim: 4,
                }
            }
            GateKind::Toffoli => {
                let toffoli = DMatrix::from_row_slice(
                    8,
                    8,
                    &[
                        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                        0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                        0.0, 0.0, 1.0, 0.0,
                    ],
                )
                .map(|x| Complex::new(x, 0.0));
                Block {
                    matrix_repr: toffoli,
                    dim: 8,
                }
            }
        }
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

        let h_block = Block::from(GateKind::Hadamard);

        let t1 = h_block
            .tensor_product(&h_block)
            .tensor_product(&h_block)
            .tensor_product(&h_block)
            .tensor_product(&h_block);

        let qstate = t1 * inr;
        // println!("{}", qstate);

        assert_eq!(qstate.qubits.len(), 32);
        for i in 0..32 {
            assert!((qstate.qubits[i] - Complex::new(1.0 / 32.0_f64.sqrt(), 0.0)).norm() < 1e-10);
        }
    }

    #[test]
    fn inverted_cnot() {
        let h_block = Block::from(GateKind::Hadamard);
        let cnot_block = Block::from(GateKind::CNOTup);

        let t1 = h_block.tensor_product(&h_block);
        let t2 = cnot_block;
        let t3 = h_block.tensor_product(&h_block);

        let t_eval = &t1 * &t2 * &t3;
        let cnot_inverted = GateKind::CNOTdown.into_block();

        // println!("{}", t_eval);
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
