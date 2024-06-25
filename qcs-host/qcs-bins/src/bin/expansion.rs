use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use nalgebra::{Complex, DMatrix};
use qcs_core::model::gates::{
    Fredkin, Gate, Hadamard, Identity, PauliX, PauliY, PauliZ, Phase, QuantumGate, Swap, Toffoli,
    CH, CP, CRX, CRY, CRZ, CU, CX, CY, CZ, RX, RY, RZ, SX, U, U1, U2, U3,
};

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

struct TE {
    left: DMatrix<Complex<f64>>,
    right: DMatrix<Complex<f64>>,
}

impl TE {
    fn new(left: DMatrix<Complex<f64>>, right: DMatrix<Complex<f64>>) -> Self {
        Self { left, right }
    }

    fn id(size: usize) -> DMatrix<Complex<f64>> {
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

    fn with_left_id(id_size: usize, right: DMatrix<Complex<f64>>) -> Self {
        Self::new(Self::id(id_size), right)
    }

    fn with_right_id(left: DMatrix<Complex<f64>>, id_size: usize) -> Self {
        Self::new(left, Self::id(id_size))
    }

    fn compute(&self) -> DMatrix<Complex<f64>> {
        self.left.kronecker(&self.right)
    }
}

trait TECompatible {
    fn as_matrix(&self) -> DMatrix<Complex<f64>>;
    fn left_te(&self, id_size: usize) -> TE {
        TE::with_left_id(id_size, self.as_matrix())
    }
    fn right_te(&self, id_size: usize) -> TE {
        TE::with_right_id(self.as_matrix(), id_size)
    }
}

impl TECompatible for Gate {
    fn as_matrix(&self) -> DMatrix<Complex<f64>> {
        match self {
            Gate::Hadamard(h) => h.matrix(),
            Gate::Identity(i) => i.matrix(),
            Gate::PauliX(x) => x.matrix(),
            Gate::PauliY(y) => y.matrix(),
            Gate::PauliZ(z) => z.matrix(),
            Gate::Phase(p) => p.matrix(),
            Gate::SX(sx) => sx.matrix(),
            Gate::RX(rx) => rx.matrix(),
            Gate::RY(ry) => ry.matrix(),
            Gate::RZ(rz) => rz.matrix(),
            Gate::CX(cx) => cx.matrix(),
            Gate::CY(cy) => cy.matrix(),
            Gate::CZ(cz) => cz.matrix(),
            Gate::CP(cp) => cp.matrix(),
            Gate::CRX(crx) => crx.matrix(),
            Gate::CRY(cry) => cry.matrix(),
            Gate::CRZ(crz) => crz.matrix(),
            Gate::CH(ch) => ch.matrix(),
            Gate::Swap(swap) => swap.matrix(),
            Gate::Toffoli(toffoli) => toffoli.matrix(),
            Gate::Fredkin(fredkin) => fredkin.matrix(),
            Gate::CU(cu) => cu.matrix(),
            Gate::U1(u1) => u1.matrix(),
            Gate::U2(u2) => u2.matrix(),
            Gate::U3(u3) => u3.matrix(),
            Gate::U(u) => u.matrix(),
        }
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

    fn add_te(&mut self, te: TE) -> std::io::Result<()> {
        // input
        let in_left_bytes = te.left.serialize();
        let in_right_bytes = te.right.serialize();
        // output
        let out_bytes = te.compute().serialize();

        self.file.write_all(&in_left_bytes)?;
        self.file.write_all(&in_right_bytes)?;
        self.file.write_all(&out_bytes)?;
        Ok(())
    }
}

fn count_non_zero(matrix: &DMatrix<Complex<f64>>) -> usize {
    matrix
        .iter()
        .fold(0, |a, b| if b.norm() > 1e-6 { a + 1 } else { a })
}

macro_rules! print_non_zero {
    ($i:ident, $te:ident) => {
        println!(
            "{}: {} x {} = {}",
            $i,
            count_non_zero(&$te.left),
            count_non_zero(&$te.right),
            count_non_zero(&$te.compute())
        );
        $i += 1;
    };
}

fn main() {
    let gates = vec![
        Gate::from(Hadamard::new(0)),
        Gate::from(Identity::new(0)),
        Gate::from(PauliX::new(0)),
        Gate::from(PauliY::new(0)),
        Gate::from(PauliZ::new(0)),
        Gate::from(Phase::new(0.0, 0)),
        Gate::from(Phase::t(0)),
        Gate::from(Phase::s(0)),
        Gate::from(SX::new(0)),
        Gate::from(RX::new(0.0, 0)),
        Gate::from(RY::new(0.0, 0)),
        Gate::from(RZ::new(0.0, 0)),
        Gate::from(CX::new(0, 1)),
        Gate::from(CX::new(1, 0)),
        Gate::from(CY::new(0, 1)),
        Gate::from(CZ::new(0, 1)),
        Gate::from(CP::new(0.0, 0, 1)),
        Gate::from(CRX::new(0.0, 0, 1)),
        Gate::from(CRY::new(0.0, 0, 1)),
        Gate::from(CRZ::new(0.0, 0, 1)),
        Gate::from(CH::new(0, 1)),
        Gate::from(Swap::new(0, 1)),
        Gate::from(Toffoli::new((0, 1), 2)),
        Gate::from(Fredkin::new(0, (1, 2))),
        Gate::from(CU::new(1.0, 2.0, 3.0, 4.0, 0, 1)),
        Gate::from(U1::new(1.0, 0)),
        Gate::from(U2::new(1.0, 2.0, 0)),
        Gate::from(U3::new(1.0, 2.0, 3.0, 0)),
        Gate::from(U::new(1.0, 2.0, 3.0, 0)),
    ];

    let mut bfile = BinFile::new(PathBuf::from("golden-vectors.dat")).unwrap();

    let mut i = 0;
    for gate in gates {
        let te1 = gate.left_te(1);
        print_non_zero!(i, te1);
        let te2 = gate.left_te(2);
        print_non_zero!(i, te2);
        let te3 = gate.right_te(1);
        print_non_zero!(i, te3);
        let te4 = gate.right_te(2);
        print_non_zero!(i, te4);

        bfile.add_te(te1).unwrap();
        bfile.add_te(te2).unwrap();
        bfile.add_te(te3).unwrap();
        bfile.add_te(te4).unwrap();
    }

    println!("Done!");
}
