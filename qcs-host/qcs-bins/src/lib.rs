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

trait Serialize {
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

pub struct TE {
    pub left: DMatrix<Complex<f64>>,
    pub right: DMatrix<Complex<f64>>,
    pub column_major: bool,
}

impl TE {
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

    pub fn id(size: usize) -> DMatrix<Complex<f64>> {
        let size = size * 2;
        DMatrix::from_iterator(
            size,
            size,
            (0..size).flat_map(|i| {
                (0..size).map(move |j| {
                    if i == j {
                        Complex::new(1.0, 0.0)
                    } else {
                        Complex::new(0.0, 0.0)
                    }
                })
            }),
        )
    }

    pub fn with_left_id(id_size: usize, right: DMatrix<Complex<f64>>) -> Self {
        Self::new(Self::id(id_size), right)
    }

    pub fn with_right_id(left: DMatrix<Complex<f64>>, id_size: usize) -> Self {
        Self::new(left, Self::id(id_size))
    }

    pub fn compute(&self) -> DMatrix<Complex<f64>> {
        self.left.kronecker(&self.right)
    }
}

impl Serialize for TE {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(0x00); // magic number for TE
        if !self.column_major {
            bytes.push(0x00); // magic number for row major
            bytes.extend_from_slice(&self.left.serialize());
            bytes.extend_from_slice(&self.right.serialize());
            bytes.extend_from_slice(&self.compute().serialize());
            bytes
        } else {
            bytes.push(0xff); // magic number for column major
            bytes.extend_from_slice(&self.left.transpose().serialize());
            bytes.extend_from_slice(&self.right.transpose().serialize());
            bytes.extend_from_slice(&self.compute().transpose().serialize());
            bytes
        }
    }
}

impl std::fmt::Display for TE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TE: {} x {} = {}",
            count_non_zero(&self.left),
            count_non_zero(&self.right),
            count_non_zero(&self.compute())
        )
    }
}

pub trait TECompatible {
    fn as_matrix(&self) -> DMatrix<Complex<f64>>;
    fn left_te(&self, id_size: usize) -> TE {
        TE::with_left_id(id_size, self.as_matrix())
    }
    fn right_te(&self, id_size: usize) -> TE {
        TE::with_right_id(self.as_matrix(), id_size)
    }
}

impl<G: QuantumGate> TECompatible for G {
    fn as_matrix(&self) -> DMatrix<Complex<f64>> {
        self.matrix()
    }
}

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
        bytes.push(0xff); // magic number for matmul
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

    pub fn add_te(&mut self, te: TE) -> std::io::Result<()> {
        self.file.write_all(&te.serialize())
    }

    pub fn add_matmul(&mut self, op: Matmul) -> std::io::Result<()> {
        self.file.write_all(&op.serialize())
    }
}
