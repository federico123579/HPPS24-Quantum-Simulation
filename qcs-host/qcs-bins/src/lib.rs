use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use nalgebra::{Complex, DMatrix};
use qcs_core::model::gates::QuantumGate;

fn count_non_zero(matrix: &DMatrix<Complex<f64>>) -> usize {
    matrix
        .iter()
        .fold(0, |a, b| if b.norm() > 1e-6 { a + 1 } else { a })
}

pub trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

impl Serialize for DMatrix<Complex<f64>> {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&(self.len().to_le_bytes()));
        self.transpose().iter().for_each(|c| {
            bytes.extend_from_slice(&c.re.to_le_bytes());
            bytes.extend_from_slice(&c.im.to_le_bytes());
        });
        bytes
    }
}

pub struct LeftTP {
    pub left: DMatrix<Complex<f64>>,
    pub column_major: bool,
}

impl LeftTP {
    pub fn new(left: DMatrix<Complex<f64>>) -> Self {
        Self {
            left,
            column_major: false,
        }
    }

    pub fn column_major(mut self) -> Self {
        self.column_major = true;
        self
    }

    pub fn compute(&self) -> DMatrix<Complex<f64>> {
        self.left.kronecker(&DMatrix::identity(2, 2))
    }
}

impl Serialize for LeftTP {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(0x00); // magic number for TE
        if !self.column_major {
            bytes.push(0x00); // magic number for row major
            bytes.extend_from_slice(&self.left.serialize());
            bytes.extend_from_slice(&self.compute().serialize());
            bytes
        } else {
            bytes.push(0xff); // magic number for column major
            bytes.extend_from_slice(&self.left.transpose().serialize());
            bytes.extend_from_slice(&self.compute().transpose().serialize());
            bytes
        }
    }
}

impl std::fmt::Display for LeftTP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LTP: {} x I(2x2) = {}",
            count_non_zero(&self.left),
            count_non_zero(&self.compute())
        )
    }
}

pub struct RightTP {
    pub right: DMatrix<Complex<f64>>,
    pub column_major: bool,
}

impl RightTP {
    pub fn new(right: DMatrix<Complex<f64>>) -> Self {
        Self {
            right,
            column_major: false,
        }
    }

    pub fn column_major(mut self) -> Self {
        self.column_major = true;
        self
    }

    pub fn compute(&self) -> DMatrix<Complex<f64>> {
        DMatrix::identity(2, 2).kronecker(&self.right)
    }
}

impl Serialize for RightTP {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(0x01); // magic number for TE
        if !self.column_major {
            bytes.push(0x00); // magic number for row major
            bytes.extend_from_slice(&self.right.serialize());
            bytes.extend_from_slice(&self.compute().serialize());
            bytes
        } else {
            bytes.push(0xff); // magic number for column major
            bytes.extend_from_slice(&self.right.transpose().serialize());
            bytes.extend_from_slice(&self.compute().transpose().serialize());
            bytes
        }
    }
}

impl std::fmt::Display for RightTP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTP: I(2x2) x {} = {}",
            count_non_zero(&self.right),
            count_non_zero(&self.compute())
        )
    }
}

pub trait TECompatible: MatrixCompatible {
    fn left_te(&self) -> LeftTP {
        LeftTP::new(self.as_matrix())
    }
    fn right_te(&self) -> RightTP {
        RightTP::new(self.as_matrix())
    }
}

impl<G: QuantumGate> TECompatible for G {}

pub struct Matmul {
    pub left: DMatrix<Complex<f64>>,
    pub right: DMatrix<Complex<f64>>,
    pub column_major: bool,
}

impl Matmul {
    pub fn new(left: DMatrix<Complex<f64>>, right: DMatrix<Complex<f64>>) -> Self {
        Self {
            left,
            right,
            column_major: false,
        }
    }

    pub fn column_major(mut self) -> Self {
        self.column_major = true;
        self
    }

    pub fn compute(&self) -> DMatrix<Complex<f64>> {
        &self.left * &self.right
    }
}

impl Serialize for Matmul {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(0x02); // magic number for matmul
        if !self.column_major {
            bytes.push(0x00); // magic number for row major
            bytes.extend_from_slice(&self.left.serialize());
            bytes.extend_from_slice(&self.right.transpose().serialize());
            bytes.extend_from_slice(&self.compute().serialize());
            bytes
        } else {
            bytes.push(0xff); // magic number for column major
            bytes.extend_from_slice(&self.left.transpose().serialize());
            bytes.extend_from_slice(&self.right.serialize());
            bytes.extend_from_slice(&self.compute().transpose().serialize());
            bytes
        }
    }
}

impl std::fmt::Display for Matmul {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MM: {} x {} = {}",
            count_non_zero(&self.left),
            count_non_zero(&self.right),
            count_non_zero(&self.compute())
        )
    }
}

pub trait MatrixCompatible {
    fn as_matrix(&self) -> DMatrix<Complex<f64>>;
}

impl<G: QuantumGate> MatrixCompatible for G {
    fn as_matrix(&self) -> DMatrix<Complex<f64>> {
        self.matrix()
    }
}

impl<T: MatrixCompatible, G: MatrixCompatible> From<(T, G)> for Matmul {
    fn from(value: (T, G)) -> Self {
        Self::new(value.0.as_matrix(), value.1.as_matrix())
    }
}

pub struct BinFile {
    file: BufWriter<File>,
}

impl BinFile {
    pub fn new(path: PathBuf) -> std::io::Result<Self> {
        let file = File::create(path)?;
        Ok(Self {
            file: BufWriter::new(file),
        })
    }

    pub fn add(&mut self, ser: impl Serialize) -> std::io::Result<()> {
        self.file.write_all(&ser.serialize())
    }
}

// impl Serialize for Instruction {
//     fn serialize(&self) -> Vec<u8> {
//         let mut bytes = vec![];
//         bytes.extend_from_slice(&self.id.to_le_bytes());

//         bytes.extend_from_slice(&self.first.serialize());
//         bytes.extend_from_slice(&self.second.serialize());
//         bytes
//     }
// }
