OPENQASM 3.0;
include "stdgates.inc";
bit[2] c;
qubit[2] q;
y q[1];
cy q[0], q[1];
c[0] = measure q[0];
c[1] = measure q[1];
