OPENQASM 3.0;
include "stdgates.inc";
gate rzx_124271021644560(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(pi/4) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate rzx_124270918343696(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(-pi/4) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate ecr _gate_q_0, _gate_q_1 {
  rzx_124271021644560(pi/4) _gate_q_0, _gate_q_1;
  x _gate_q_0;
  rzx_124270918343696(-pi/4) _gate_q_0, _gate_q_1;
}
gate cs _gate_q_0, _gate_q_1 {
  p(pi/4) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  p(-pi/4) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  p(pi/4) _gate_q_1;
}
gate sxdg _gate_q_0 {
  s _gate_q_0;
  h _gate_q_0;
  s _gate_q_0;
}
gate rxx_124270813590352(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_0;
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(5.383626559749287) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
  h _gate_q_0;
}
gate r_124270813590800(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(3.598355243892237, 2.7425783933620096, -2.7425783933620096) _gate_q_0;
}
gate xx_minus_yy_124270813591312(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(-5.666507114083973) _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sx _gate_q_0;
  rz(pi/2) _gate_q_0;
  s _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  ry(1.5069678747533946) _gate_q_0;
  ry(-1.5069678747533946) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  sdg _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sxdg _gate_q_0;
  rz(pi/2) _gate_q_0;
  rz(5.666507114083973) _gate_q_1;
}
gate csdg _gate_q_0, _gate_q_1 {
  p(-pi/4) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  p(pi/4) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  p(-pi/4) _gate_q_1;
}
gate rzz_124270813591440(_gate_p_0) _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  rz(3.310098862599033) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
}
gate cu1_124270813591888(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(1.9702252857606735) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-1.9702252857606735) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(1.9702252857606735) _gate_q_1;
}
bit[9] c;
qubit[9] q;
crz(4.221875016126573) q[8], q[5];
t q[4];
cp(5.54657321772129) q[2], q[0];
id q[6];
ecr q[1], q[3];
t q[7];
cs q[0], q[3];
sxdg q[8];
ch q[1], q[6];
ch q[2], q[7];
u2(5.002937306446207, 0.4388506603016791) q[4];
sxdg q[5];
ccx q[2], q[0], q[8];
cp(2.5952081295929132) q[1], q[5];
rxx_124270813590352(5.383626559749287) q[3], q[4];
rx(4.227190281107226) q[7];
r_124270813590800(3.598355243892237, 4.313374720156906) q[6];
z q[3];
ry(3.8898722182212166) q[8];
xx_minus_yy_124270813591312(3.013935749506789, 5.666507114083973) q[0], q[2];
x q[5];
csdg q[6], q[1];
rzz_124270813591440(3.310098862599033) q[7], q[4];
cu1_124270813591888(3.940450571521347) q[2], q[0];
ecr q[8], q[1];
cp(0.1489941979638513) q[7], q[6];
tdg q[4];
tdg q[3];
ry(4.310677471230688) q[5];
c[0] = measure q[0];
c[1] = measure q[1];
c[2] = measure q[2];
c[3] = measure q[3];
c[4] = measure q[4];
c[5] = measure q[5];
c[6] = measure q[6];
c[7] = measure q[7];
c[8] = measure q[8];
