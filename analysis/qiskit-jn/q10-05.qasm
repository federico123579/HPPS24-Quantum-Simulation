OPENQASM 3.0;
include "stdgates.inc";
gate sxdg _gate_q_0 {
  s _gate_q_0;
  h _gate_q_0;
  s _gate_q_0;
}
gate xx_plus_yy_124835207508880(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(0.4860126295685338) _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sx _gate_q_1;
  rz(pi/2) _gate_q_1;
  s _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  ry(-2.2493253973678016) _gate_q_1;
  ry(-2.2493253973678016) _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  sdg _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sxdg _gate_q_1;
  rz(pi/2) _gate_q_1;
  rz(-0.4860126295685338) _gate_q_0;
}
gate rxx_124835207509008(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_0;
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(4.488715489192876) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
  h _gate_q_0;
}
gate rcccx _gate_q_0, _gate_q_1, _gate_q_2, _gate_q_3 {
  u2(0, pi) _gate_q_3;
  u1(pi/4) _gate_q_3;
  cx _gate_q_2, _gate_q_3;
  u1(-pi/4) _gate_q_3;
  u2(0, pi) _gate_q_3;
  cx _gate_q_0, _gate_q_3;
  u1(pi/4) _gate_q_3;
  cx _gate_q_1, _gate_q_3;
  u1(-pi/4) _gate_q_3;
  cx _gate_q_0, _gate_q_3;
  u1(pi/4) _gate_q_3;
  cx _gate_q_1, _gate_q_3;
  u1(-pi/4) _gate_q_3;
  u2(0, pi) _gate_q_3;
  u1(pi/4) _gate_q_3;
  cx _gate_q_2, _gate_q_3;
  u1(-pi/4) _gate_q_3;
  u2(0, pi) _gate_q_3;
}
gate rzz_124835218694480(_gate_p_0) _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  rz(0.009543398380472872) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
}
gate rzx_124835207510992(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(0.4909483772592945) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate rzz_124835207511696(_gate_p_0) _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  rz(5.676935503257807) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
}
gate xx_minus_yy_124835207528848(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(-4.368550998558885) _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sx _gate_q_0;
  rz(pi/2) _gate_q_0;
  s _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  ry(0.12686754622561747) _gate_q_0;
  ry(-0.12686754622561747) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  sdg _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sxdg _gate_q_0;
  rz(pi/2) _gate_q_0;
  rz(4.368550998558885) _gate_q_1;
}
bit[10] c;
qubit[10] q;
xx_plus_yy_124835207508880(4.498650794735603, 0.4860126295685338) q[2], q[3];
swap q[9], q[5];
y q[4];
rxx_124835207509008(4.488715489192876) q[6], q[0];
cswap q[1], q[8], q[7];
rcccx q[0], q[7], q[5], q[3];
sdg q[2];
U(5.087073338952834, 6.078627148687264, 0.4468520636471762) q[6];
crx(5.267938438700591) q[8], q[1];
cry(4.99995602962482) q[4], q[9];
cu(2.8487488745618177, 3.7704418372770285, 3.9684304734699047, 3.0928616920893397) q[5], q[1];
sx q[4];
h q[3];
y q[0];
rzz_124835218694480(0.009543398380472872) q[8], q[6];
y q[2];
rz(3.723401231901788) q[7];
id q[9];
rzx_124835207510992(0.4909483772592945) q[1], q[8];
cz q[5], q[9];
cswap q[6], q[4], q[0];
crx(3.925653538508188) q[7], q[3];
rx(3.475101330158688) q[2];
rzz_124835207511696(5.676935503257807) q[1], q[6];
p(3.1999516676014657) q[0];
cu(2.8450029992674093, 4.195503175103105, 1.6454996210869146, 5.594319364161862) q[3], q[5];
U(6.116855910317201, 0.7651980463013663, 2.2981143078811503) q[4];
xx_minus_yy_124835207528848(0.25373509245123493, 4.368550998558885) q[2], q[8];
cry(1.473297605145872) q[9], q[7];
c[0] = measure q[0];
c[1] = measure q[1];
c[2] = measure q[2];
c[3] = measure q[3];
c[4] = measure q[4];
c[5] = measure q[5];
c[6] = measure q[6];
c[7] = measure q[7];
c[8] = measure q[8];
c[9] = measure q[9];
