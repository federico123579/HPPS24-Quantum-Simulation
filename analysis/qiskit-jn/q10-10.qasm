OPENQASM 3.0;
include "stdgates.inc";
gate cs _gate_q_0, _gate_q_1 {
  p(pi/4) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  p(-pi/4) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  p(pi/4) _gate_q_1;
}
gate rzx_128760746579664(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(pi/4) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate rzx_128760708948048(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(-pi/4) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate ecr _gate_q_0, _gate_q_1 {
  rzx_128760746579664(pi/4) _gate_q_0, _gate_q_1;
  x _gate_q_0;
  rzx_128760708948048(-pi/4) _gate_q_0, _gate_q_1;
}
gate sxdg _gate_q_0 {
  s _gate_q_0;
  h _gate_q_0;
  s _gate_q_0;
}
gate xx_plus_yy_128760604212112(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(3.9243503506255313) _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sx _gate_q_1;
  rz(pi/2) _gate_q_1;
  s _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  ry(-2.5256462101352373) _gate_q_1;
  ry(-2.5256462101352373) _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  sdg _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sxdg _gate_q_1;
  rz(pi/2) _gate_q_1;
  rz(-3.9243503506255313) _gate_q_0;
}
gate iswap _gate_q_0, _gate_q_1 {
  s _gate_q_0;
  s _gate_q_1;
  h _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  cx _gate_q_1, _gate_q_0;
  h _gate_q_1;
}
gate dcx _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  cx _gate_q_1, _gate_q_0;
}
gate cu3_128760604212624(_gate_p_0, _gate_p_1, _gate_p_2) _gate_q_0, _gate_q_1 {
  u1(2.244175599161016) _gate_q_0;
  u1(-0.945872043472931) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(-0.5933267797725718, 0, -2.244175599161016) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(0.5933267797725718, 3.1900476426339472, 0) _gate_q_1;
}
gate cu1_128760604213136(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(2.547781722869658) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-2.547781722869658) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(2.547781722869658) _gate_q_1;
}
gate rzx_128760604214096(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(0.8710159529309867) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate rzz_128760604214992(_gate_p_0) _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  rz(2.679386206760883) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
}
gate cu3_128760604215120(_gate_p_0, _gate_p_1, _gate_p_2) _gate_q_0, _gate_q_1 {
  u1(4.36960689157562) _gate_q_0;
  u1(-0.04893111086946078) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(-1.110181591635635, 0, -4.36960689157562) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(1.110181591635635, 4.41853800244508, 0) _gate_q_1;
}
gate cu3_128760604215184(_gate_p_0, _gate_p_1, _gate_p_2) _gate_q_0, _gate_q_1 {
  u1(0.9367397782247194) _gate_q_0;
  u1(-0.7256557681389798) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(-1.8601169757073752, 0, -0.9367397782247194) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(1.8601169757073752, 1.6623955463636992, 0) _gate_q_1;
}
gate ryy_128760604215952(_gate_p_0) _gate_q_0, _gate_q_1 {
  rx(pi/2) _gate_q_0;
  rx(pi/2) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(0.39070976298036) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rx(-pi/2) _gate_q_0;
  rx(-pi/2) _gate_q_1;
}
gate cu1_128760720226576(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(pi/4) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-pi/4) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(pi/4) _gate_q_1;
}
gate csx _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cu1_128760720226576(pi/2) _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate r_128760604216016(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(2.1594036729190114, 0.05885704422127702, -0.05885704422127702) _gate_q_0;
}
gate rzz_128760604216400(_gate_p_0) _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  rz(4.104644043164832) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
}
bit[10] c;
qubit[10] q;
sdg q[6];
cy q[8], q[7];
ccx q[2], q[9], q[3];
p(3.5219120692876267) q[5];
u3(4.897392059311318, 4.973940531757332, 3.8361118056221173) q[4];
cs q[1], q[0];
cz q[4], q[3];
sdg q[9];
ch q[1], q[5];
t q[0];
ecr q[6], q[7];
crx(4.109632344973469) q[8], q[2];
xx_plus_yy_128760604212112(5.0512924202704745, 3.9243503506255313) q[0], q[5];
tdg q[2];
swap q[3], q[6];
s q[4];
rz(6.183033881874703) q[9];
iswap q[7], q[8];
sxdg q[1];
cx q[0], q[8];
dcx q[6], q[7];
cx q[3], q[1];
rx(2.39788008690304) q[5];
cu3_128760604212624(1.1866535595451435, 3.1900476426339472, 1.2983035556880853) q[2], q[4];
z q[9];
ecr q[3], q[4];
u2(5.506374688491001, 2.513548082381991) q[5];
ccx q[7], q[9], q[8];
dcx q[1], q[6];
cu1_128760604213136(5.095563445739316) q[2], q[0];
x q[2];
rzx_128760604214096(0.8710159529309867) q[6], q[4];
ch q[8], q[1];
id q[0];
crx(5.497770952393233) q[9], q[3];
sx q[7];
sx q[5];
u1(0.8609107817179044) q[3];
rx(0.42044018579583786) q[8];
rzz_128760604214992(2.679386206760883) q[0], q[2];
cu3_128760604215120(2.22036318327127, 4.41853800244508, 4.320675780706159) q[1], q[7];
x q[9];
cu3_128760604215184(3.7202339514147504, 1.6623955463636992, 0.21108401008573963) q[4], q[5];
tdg q[6];
s q[5];
ryy_128760604215952(0.39070976298036) q[9], q[1];
csx q[2], q[4];
r_128760604216016(2.1594036729190114, 1.6296533710161736) q[6];
cswap q[8], q[7], q[0];
tdg q[3];
cz q[8], q[4];
x q[7];
cry(2.2382253668877072) q[6], q[5];
ccx q[2], q[9], q[0];
rzz_128760604216400(4.104644043164832) q[1], q[3];
sdg q[0];
swap q[1], q[2];
cz q[9], q[4];
cx q[6], q[3];
cswap q[5], q[7], q[8];
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
