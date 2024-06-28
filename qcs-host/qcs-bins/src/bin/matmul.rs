use std::path::PathBuf;

use qcs_bins::{BinFile, Matmul};
use qcs_core::model::{
    gates::{
        Fredkin, Hadamard, Identity, PauliY, PauliZ, Phase, Swap, Toffoli, CRX, CRY, CRZ, CU, CX,
        CY, CZ, RX, RY, RZ, U, U1, U2, U3,
    },
    TensorProduct,
};

fn main() {
    let u = U::new(1.0, 2.0, 3.0, 0);
    let massive_u = u
        .tensor_product(Identity::new(0))
        .tensor_product(Identity::new(0))
        .tensor_product(Identity::new(0))
        .tensor_product(Identity::new(0));
    let ops: Vec<Matmul> = vec![
        (Hadamard::new(0), Identity::new(0)).into(),
        (PauliY::new(0), PauliY::new(0)).into(),
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
        Matmul::new(massive_u.clone().into_matrix(), massive_u.into_matrix()),
    ];

    let mut bfile = BinFile::new(PathBuf::from("golden-vectors.dat")).unwrap();

    for (op, i) in ops.into_iter().zip(0..) {
        println!("{}: {}", i, op);
        bfile.add(op).unwrap();
    }

    println!("Done!");
}
