use std::path::PathBuf;

use qcs_bins::{BinFile, TECompatible, TE};
use qcs_core::model::{
    gates::{
        Fredkin, Gate, Hadamard, Identity, PauliX, PauliY, PauliZ, Phase, QuantumGate, Swap,
        Toffoli, CH, CP, CRX, CRY, CRZ, CU, CX, CY, CZ, RX, RY, RZ, SX, U, U1, U2, U3,
    },
    TensorProduct,
};

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

    println!("Golden vectors:");
    let mut bfile = BinFile::new(PathBuf::from("golden-vectors.dat")).unwrap();
    let mut j = 0;

    // test all gates with all possible expansions
    for (gate, _) in gates.into_iter().zip(0..) {
        let te1 = gate.left_te(1);
        println!("{}: {}", j, te1);
        j += 1;
        bfile.add(te1).unwrap();

        let te2 = gate.left_te(2);
        println!("{}: {}", j, te2);
        j += 1;
        bfile.add(te2).unwrap();

        let te3 = gate.right_te(1);
        println!("{}: {}", j, te3);
        j += 1;
        bfile.add(te3).unwrap();

        let te4 = gate.right_te(2);
        println!("{}: {}", j, te4);
        j += 1;
        bfile.add(te4).unwrap();
    }

    // test tensor product with a non-sparse matrix
    let big_te = U::new(1.0, 2.0, 3.0, 0).tensor_product(U::new(1.0, 2.0, 3.0, 0));
    let big_te = TE::new(big_te.clone().into_matrix(), big_te.into_matrix());
    println!("{} - big: {}", j, big_te);
    j += 1;
    bfile.add(big_te).unwrap();

    println!("Sparse stress test:");
    j = 0;
    let mut bfile = BinFile::new(PathBuf::from("sparse_stress.dat")).unwrap();

    // test tensor product with sparse matrices larger and larger
    for i in 1..=13 {
        let block = (1..i).fold(PauliX::new(0).block(), |a, _| {
            a.tensor_product(Identity::new(0))
        });
        let te = TE::new(
            block.clone().into_matrix(),
            Identity::new(0).block().into_matrix(),
        );
        println!("{} - sparse {}: {}", j, i, te);
        j += 1;
        bfile.add(te).unwrap();
        let te = TE::new(
            Identity::new(0).block().into_matrix(),
            block.clone().into_matrix(),
        );
        println!("{} - sparse {}: {}", j, i, te);
        j += 1;

        bfile.add(te).unwrap();
    }

    println!("Dense stress test:");
    j = 0;
    let mut bfile = BinFile::new(PathBuf::from("dense_stress.dat")).unwrap();

    // test tensor product with dense matrices larger and larger
    for i in 1..=6 {
        let block = (1..i).fold(Hadamard::new(0).block(), |a, _| {
            a.tensor_product(Hadamard::new(0))
        });
        let te = TE::new(
            block.clone().into_matrix(),
            Hadamard::new(0).block().into_matrix(),
        );
        println!("{} - dense {}: {}", j, i, te);
        j += 1;
        bfile.add(te).unwrap();
        let te = TE::new(
            Hadamard::new(0).block().into_matrix(),
            block.clone().into_matrix(),
        );
        println!("{} - dense {}: {}", j, i, te);
        j += 1;
        bfile.add(te).unwrap();
    }

    println!("Done!");
}
