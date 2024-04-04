use nalgebra::{Complex, DMatrix, DVector, Vector2};

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

    pub fn probabilities(&self) -> Vector2<f64> {
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
            qubits: qubits.into_iter().map(|q| q.amplitudes).enumerate().fold(
                DVector::zeros(N * 2),
                |mut acc, (i, q)| {
                    acc[i * 2] = q[0];
                    acc[i * 2 + 1] = q[1];
                    acc
                },
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum KnownGates {
    #[default]
    Identity,
    PauliX,
    Hadamard,
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

impl From<KnownGates> for Block {
    fn from(gate: KnownGates) -> Self {
        match gate {
            KnownGates::Identity => Block {
                matrix_repr: DMatrix::identity(2, 2),
                dim: 2,
            },
            KnownGates::PauliX => Block {
                matrix_repr: DMatrix::from_row_slice(2, 2, &[0.0, 1.0, 1.0, 0.0])
                    .map(|x| Complex::new(x, 0.0)),
                dim: 2,
            },
            KnownGates::Hadamard => Block {
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
        }
    }
}
