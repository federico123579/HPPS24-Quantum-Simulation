#!/usr/bin/env python

from fire import Fire
from qiskit.circuit.random import random_circuit
from qiskit.qasm3 import dump


def main(dim, depth, name=None):
    circ = random_circuit(dim, depth, measure=True)
    if name is None:
        name = f"q{dim:02}-{depth:02}"
    with open(f"{name}.qasm", "w") as f:
        dump(circ, f)


if __name__ == "__main__":
    Fire(main)
