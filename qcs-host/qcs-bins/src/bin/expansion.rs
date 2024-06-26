use std::path::PathBuf;

use qcs_bins::{BinFile, TECompatible};
use qcs_core::model::gates::{
    Fredkin, Gate, Hadamard, Identity, PauliX, PauliY, PauliZ, Phase, Swap, Toffoli, CH, CP, CRX,
    CRY, CRZ, CU, CX, CY, CZ, RX, RY, RZ, SX, U, U1, U2, U3,
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

    let mut bfile = BinFile::new(PathBuf::from("golden-vectors.dat")).unwrap();

    for (gate, i) in gates.into_iter().zip(0..) {
        let te1 = gate.left_te(1);
        println!("{}: {}", i * 4, te1);
        bfile.add_te(te1).unwrap();

        let te2 = gate.left_te(2);
        println!("{}: {}", i * 4 + 1, te2);
        bfile.add_te(te2).unwrap();

        let te3 = gate.right_te(1);
        println!("{}: {}", i * 4 + 2, te3);
        bfile.add_te(te3).unwrap();

        let te4 = gate.right_te(2);
        println!("{}: {}", i * 4 + 3, te4);
        bfile.add_te(te4).unwrap();
    }

    println!("Done!");
}
