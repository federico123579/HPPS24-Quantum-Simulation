use nalgebra::{Complex, ComplexField, DMatrix, DVector, Matrix2, Vector2};

mod dag;

#[derive(Clone)]
struct Qubit {
    amplitudes: Vector2<Complex<f64>>,
}

impl Qubit {
    fn new(alpha: Complex<f64>, beta: Complex<f64>) -> Self {
        Qubit {
            amplitudes: Vector2::new(alpha, beta),
        }
    }

    fn probabilities(&self) -> Vector2<f64> {
        self.amplitudes.map(|x| x.norm_sqr())
    }
}

impl std::fmt::Display for Qubit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.amplitudes)
    }
}

#[derive(Clone)]
struct QRegister {
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

struct Gate {
    matrix_repr: Matrix2<Complex<f64>>,
}

impl From<KnownGates> for Gate {
    fn from(gate: KnownGates) -> Self {
        match gate {
            KnownGates::Identity => Gate {
                matrix_repr: Matrix2::identity(),
            },
            KnownGates::PauliX => Gate {
                matrix_repr: Matrix2::new(0.0.into(), 1.0.into(), 1.0.into(), 0.0.into()),
            },
            KnownGates::Hadamard => Gate {
                matrix_repr: Matrix2::new(1.0.into(), 1.0.into(), 1.0.into(), (-1.0).into())
                    / Complex::new(2.0, 0.0).sqrt(),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
enum KnownGates {
    #[default]
    Identity,
    PauliX,
    Hadamard,
}

#[derive(Debug, Clone, PartialEq)]
struct Block {
    matrix_repr: DMatrix<Complex<f64>>,
}

impl Block {
    fn tensor_product(&self, rhs: &Block) -> Block {
        Block {
            matrix_repr: self.as_ref().kronecker(rhs.as_ref()),
        }
    }
}

impl AsRef<DMatrix<Complex<f64>>> for Block {
    fn as_ref(&self) -> &DMatrix<Complex<f64>> {
        &self.matrix_repr
    }
}

// impl Deref for Block {
//     type Target = DMatrix<Complex<f64>>;

//     fn deref(&self) -> &Self::Target {
//         &self.matrix_repr
//     }
// }

// impl DerefMut for Block {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.matrix_repr
//     }
// }

impl std::ops::Mul<&Block> for &Block {
    type Output = Block;

    fn mul(self, rhs: &Block) -> Self::Output {
        Block {
            matrix_repr: self.as_ref() * rhs.as_ref(),
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
        }
    }
}

impl std::ops::Sub for Block {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Block {
            matrix_repr: self.as_ref() - rhs.as_ref(),
        }
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.matrix_repr)
    }
}

impl<G: Into<Gate>> From<G> for Block {
    fn from(gate: G) -> Self {
        Block {
            matrix_repr: DMatrix::from_column_slice(2, 2, gate.into().matrix_repr.data.as_slice()),
        }
    }
}

fn main() {
    let a_block = Block::from(KnownGates::PauliX);
    let b_block = Block::from(KnownGates::Identity);
    let c_block = a_block.tensor_product(&b_block);
    println!("Block A: {}", a_block);
    println!("Block B: {}", b_block);
    println!("Block C: {}", c_block);

    let phi = Qubit::new(Complex::i(), 0.0.into());
    let psi = Qubit::new(0.0.into(), (-1.0).into());
    let a_reg = QRegister::from([phi.clone()]);
    let b_reg = QRegister::from([psi.clone()]);
    let c_reg = QRegister::from([phi, psi]);
    println!("Initial A: {}", a_reg);
    println!("Initial B: {}", b_reg);
    println!("Initial C: {}", c_reg);

    let a_reg = a_block * a_reg;
    let b_reg = b_block * b_reg;
    let c_reg = c_block * c_reg;
    println!("A After gate: {}", a_reg);
    println!("B After gate: {}", b_reg);
    println!("C After gate: {}", c_reg);
}
