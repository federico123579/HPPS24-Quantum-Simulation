use qcs_core::model::{QRegister, QuantumCircuit, Qubit};

#[test]
fn full_adder() {
    let mut circuit = QuantumCircuit::new(4);
    circuit.g_cxx(0, 1, 3);
    circuit.g_cx(0, 1);
    circuit.g_cxx(1, 2, 3);
    circuit.g_cx(1, 2);
    circuit.g_cx(0, 1);

    let t_eval = circuit.eval();
    println!("Circuit eval: {}", t_eval);

    let regs = (0..2).zip(0..2).zip(0..2).map(|((a, b), cin)| (a, b, cin));
    for (a, b, cin) in regs {
        let input = new_reg([a == 1, b == 1, cin == 1, false]);
        let qstate = &t_eval * input;
        let s = a ^ b ^ cin;
        let cout = (a & b) | (b & cin) | (cin & a);
        let state: usize = a * 8 + b * 4 + s * 2 + cout;
        let distr = qstate.distr();
        for i in 0..16 {
            if i == state {
                assert!((distr[i] - 1.0).abs() < 1e-10);
            } else {
                assert!((distr[i] - 0.0).abs() < 1e-10);
            }
        }
    }
}

fn new_reg(bits: [bool; 4]) -> QRegister {
    QRegister::from(
        bits.iter()
            .map(|&b| if b { Qubit::one() } else { Qubit::zero() }),
    )
}
