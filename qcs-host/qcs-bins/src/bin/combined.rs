use std::path::PathBuf;

use qcs_bins::{BinFile, Matmul, TECompatible};
use qcs_core::model::gates::U;

fn main() {
    let mut bfile = BinFile::new(PathBuf::from("golden-vectors.dat")).unwrap();

    let left = U::new(1.0, 2.0, 3.0, 0).right_te(1);
    println!("InL {}", left);
    let right = U::new(1.0, 2.0, 3.0, 0).left_te(1);
    println!("InR {}", right);
    let out = Matmul::new(left.compute(), right.compute());
    println!("Out {}", out);

    bfile.add(left).unwrap();
    bfile.add(right.column_major()).unwrap();
    bfile.add(out).unwrap();

    println!("Done!");
}
