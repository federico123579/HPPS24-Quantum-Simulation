OPENQASM 3.0;
include "stdgates.inc";
bit[1] c;
qubit[1] q;
sx q[0];
sx q[0];
x q[0];
c[0] = measure q[0];
