use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use nalgebra::{Complex, DMatrix};
use qcs_core::model::gates::{
    Fredkin, Hadamard, Identity, PauliX, PauliY, PauliZ, Phase, QuantumGate, Swap, Toffoli, CRX,
    CRY, CRZ, CU, CX, CY, CZ, RX, RY, RZ, U, U1, U2, U3,
};

trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

impl Serialize for DMatrix<Complex<f64>> {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&(self.len().to_le_bytes()));
        self.iter().for_each(|c| {
            bytes.extend_from_slice(&c.re.to_le_bytes());
            bytes.extend_from_slice(&c.im.to_le_bytes());
        });
        bytes
    }
}

struct Matmul {
    left: DMatrix<Complex<f64>>,
    right: DMatrix<Complex<f64>>,
}

impl Matmul {
    fn new(left: DMatrix<Complex<f64>>, right: DMatrix<Complex<f64>>) -> Self {
        Self { left, right }
    }

    fn compute(&self) -> DMatrix<Complex<f64>> {
        &self.left * &self.right
    }
}

trait MatrixCompatible {
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

struct BinFile {
    file: BufWriter<File>,
}

impl BinFile {
    fn new(path: PathBuf) -> std::io::Result<Self> {
        let file = File::create(path)?;
        Ok(Self {
            file: BufWriter::new(file),
        })
    }

    fn add_matmul(&mut self, op: Matmul) -> std::io::Result<()> {
        // input
        let in_left_bytes = op.left.serialize();
        let in_right_bytes = op.right.transpose().serialize();
        // output
        let out_bytes = op.compute().serialize();

        self.file.write_all(&in_left_bytes)?;
        self.file.write_all(&in_right_bytes)?;
        self.file.write_all(&out_bytes)?;
        Ok(())
    }
}

fn main() {
    let ops: Vec<Matmul> = vec![
        (Hadamard::new(0), Identity::new(0)).into(),
        (PauliX::new(0), PauliY::new(0)).into(),
        (PauliY::new(0), PauliZ::new(0)).into(),
        (Phase::t(0), Phase::s(0)).into(),
        (RX::new(0.0, 0), RY::new(0.0, 0)).into(),
        (RY::new(0.0, 0), RZ::new(0.0, 0)).into(),
        (CX::new(0, 1), CX::new(1, 0)).into(),
        (CY::new(0, 1), CZ::new(0, 1)).into(),
        (CRX::new(0.0, 0, 1), CRY::new(0.0, 0, 1)).into(),
        (CRZ::new(0.0, 0, 1), CRZ::new(0.0, 0, 1)).into(),
        (Toffoli::new((0, 1), 2), (Fredkin::new(0, (1, 2)))).into(),
        (CU::new(1.0, 2.0, 3.0, 4.0, 0, 1), Swap::new(0, 1)).into(),
        (U2::new(1.0, 2.0, 0), U3::new(1.0, 2.0, 3.0, 0)).into(),
        (U::new(1.0, 2.0, 3.0, 0), U1::new(1.0, 0)).into(),
    ];

    let mut bfile = BinFile::new(PathBuf::from("golden-vectors.dat")).unwrap();

    for op in ops {
        bfile.add_matmul(op).unwrap();
    }

    println!("Done!");
}
