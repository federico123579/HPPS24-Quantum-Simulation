OPENQASM 3.0;
include "stdgates.inc";
gate rccx _gate_q_0, _gate_q_1, _gate_q_2 {
  u2(0, pi) _gate_q_2;
  u1(pi/4) _gate_q_2;
  cx _gate_q_1, _gate_q_2;
  u1(-pi/4) _gate_q_2;
  cx _gate_q_0, _gate_q_2;
  u1(pi/4) _gate_q_2;
  cx _gate_q_1, _gate_q_2;
  u1(-pi/4) _gate_q_2;
  u2(0, pi) _gate_q_2;
}
gate rzz_124410474260816(_gate_p_0) _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  rz(4.846679151627187) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
}
gate ccz _gate_q_0, _gate_q_1, _gate_q_2 {
  h _gate_q_2;
  ccx _gate_q_0, _gate_q_1, _gate_q_2;
  h _gate_q_2;
}
gate dcx _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  cx _gate_q_1, _gate_q_0;
}
bit[7] c;
qubit[7] q;
ry(4.052698639045546) q[6];
cswap q[0], q[5], q[1];
id q[3];
tdg q[2];
rz(4.465388638990896) q[4];
rccx q[6], q[3], q[2];
rx(2.997532774173221) q[0];
rccx q[5], q[1], q[4];
rzz_124410474260816(4.846679151627187) q[3], q[6];
cz q[5], q[2];
s q[1];
cz q[4], q[0];
ccz q[1], q[2], q[3];
u2(0.06081335309473021, 4.877627582631564) q[6];
cy q[4], q[0];
tdg q[5];
dcx q[3], q[6];
U(2.216843790579552, 2.546871333964584, 0.005312300617783732) q[0];
rz(3.4105372131001728) q[2];
rccx q[5], q[1], q[4];
c[0] = measure q[0];
c[1] = measure q[1];
c[2] = measure q[2];
c[3] = measure q[3];
c[4] = measure q[4];
c[5] = measure q[5];
c[6] = measure q[6];
