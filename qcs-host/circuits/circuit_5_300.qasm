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
gate dcx _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  cx _gate_q_1, _gate_q_0;
}
gate rzx_127318481506848(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(2.2496982476760072) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate r_127318481507184(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(5.375633947978526, 4.177295316903021, -4.177295316903021) _gate_q_0;
}
gate r_127318481516448(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(2.7944186645604887, 3.755167488276599, -3.755167488276599) _gate_q_0;
}
gate cu1_127318534067536(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(pi/4) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-pi/4) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(pi/4) _gate_q_1;
}
gate csx _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cu1_127318534067536(pi/2) _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate csdg _gate_q_0, _gate_q_1 {
  p(-pi/4) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  p(pi/4) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  p(-pi/4) _gate_q_1;
}
gate cu1_127318487317056(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(1.2650491196870048) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-1.2650491196870048) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(1.2650491196870048) _gate_q_1;
}
gate rzx_127318487316912(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(4.3391512008224495) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate sxdg _gate_q_0 {
  s _gate_q_0;
  h _gate_q_0;
  s _gate_q_0;
}
gate ryy_127318487316624(_gate_p_0) _gate_q_0, _gate_q_1 {
  rx(pi/2) _gate_q_0;
  rx(pi/2) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(5.537368951486815) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rx(-pi/2) _gate_q_0;
  rx(-pi/2) _gate_q_1;
}
gate cu1_127318487316768(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(3.030357426033981) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-3.030357426033981) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(3.030357426033981) _gate_q_1;
}
gate rzz_127318274390736(_gate_p_0) _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  rz(4.80835203475167) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
}
gate r_127318274391600(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(6.185557422911476, 1.2944422253378725, -1.2944422253378725) _gate_q_0;
}
gate xx_minus_yy_127318274390784(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(-3.720681675380936) _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sx _gate_q_0;
  rz(pi/2) _gate_q_0;
  s _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  ry(0.8048697576156436) _gate_q_0;
  ry(-0.8048697576156436) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  sdg _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sxdg _gate_q_0;
  rz(pi/2) _gate_q_0;
  rz(3.720681675380936) _gate_q_1;
}
gate r_127318274391936(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(6.264342157961093, 2.234172748840188, -2.234172748840188) _gate_q_0;
}
gate cs _gate_q_0, _gate_q_1 {
  p(pi/4) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  p(-pi/4) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  p(pi/4) _gate_q_1;
}
gate rzx_127318274391504(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(5.105263126846988) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate rxx_127318274391120(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_0;
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(3.238588135065225) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
  h _gate_q_0;
}
gate rzz_127318274386896(_gate_p_0) _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  rz(6.110490760285972) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
}
gate ryy_127318274390400(_gate_p_0) _gate_q_0, _gate_q_1 {
  rx(pi/2) _gate_q_0;
  rx(pi/2) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(3.3481918802045634) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rx(-pi/2) _gate_q_0;
  rx(-pi/2) _gate_q_1;
}
gate rzx_127318274387664(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(1.7655480440886615) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
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
gate cu1_127318274390064(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(2.9248766375777784) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-2.9248766375777784) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(2.9248766375777784) _gate_q_1;
}
gate rzz_127318274388048(_gate_p_0) _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  rz(2.5389416362715123) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
}
gate xx_plus_yy_127318274389536(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(1.4984935074203312) _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sx _gate_q_1;
  rz(pi/2) _gate_q_1;
  s _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  ry(-1.0504202549714574) _gate_q_1;
  ry(-1.0504202549714574) _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  sdg _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sxdg _gate_q_1;
  rz(pi/2) _gate_q_1;
  rz(-1.4984935074203312) _gate_q_0;
}
gate cu1_127318533926320(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(pi/16) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-pi/16) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(pi/16) _gate_q_1;
}
gate cu1_127318533926560(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(-pi/16) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(pi/16) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(-pi/16) _gate_q_1;
}
gate cu1_127318534067152(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(pi/16) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-pi/16) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(pi/16) _gate_q_1;
}
gate cu1_127318534067680(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(-pi/16) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(pi/16) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(-pi/16) _gate_q_1;
}
gate cu1_127318534067824(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(pi/16) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-pi/16) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(pi/16) _gate_q_1;
}
gate cu1_127318534067920(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(-pi/16) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(pi/16) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(-pi/16) _gate_q_1;
}
gate cu1_127318534068016(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(pi/16) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-pi/16) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(pi/16) _gate_q_1;
}
gate c3sx _gate_q_0, _gate_q_1, _gate_q_2, _gate_q_3 {
  h _gate_q_3;
  cu1_127318533926320(pi/8) _gate_q_0, _gate_q_3;
  h _gate_q_3;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_3;
  cu1_127318533926560(-pi/8) _gate_q_1, _gate_q_3;
  h _gate_q_3;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_3;
  cu1_127318534067152(pi/8) _gate_q_1, _gate_q_3;
  h _gate_q_3;
  cx _gate_q_1, _gate_q_2;
  h _gate_q_3;
  cu1_127318534067680(-pi/8) _gate_q_2, _gate_q_3;
  h _gate_q_3;
  cx _gate_q_0, _gate_q_2;
  h _gate_q_3;
  cu1_127318534067824(pi/8) _gate_q_2, _gate_q_3;
  h _gate_q_3;
  cx _gate_q_1, _gate_q_2;
  h _gate_q_3;
  cu1_127318534067920(-pi/8) _gate_q_2, _gate_q_3;
  h _gate_q_3;
  cx _gate_q_0, _gate_q_2;
  h _gate_q_3;
  cu1_127318534068016(pi/8) _gate_q_2, _gate_q_3;
  h _gate_q_3;
}
gate r_127318274389584(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(4.799944822028326, 3.1198644172106205, -3.1198644172106205) _gate_q_0;
}
gate cu3_127318274390160(_gate_p_0, _gate_p_1, _gate_p_2) _gate_q_0, _gate_q_1 {
  u1(2.800403555896905) _gate_q_0;
  u1(1.9232565661847256) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(-2.2796875416566644, 0, -2.800403555896905) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(2.2796875416566644, 0.8771469897121793, 0) _gate_q_1;
}
gate rzx_127318274389392(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(1.086401886664922) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate cu3_127318274389872(_gate_p_0, _gate_p_1, _gate_p_2) _gate_q_0, _gate_q_1 {
  u1(2.672401028695132) _gate_q_0;
  u1(-1.0511643939591497) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(-0.11532644283562773, 0, -2.672401028695132) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(0.11532644283562773, 3.7235654226542816, 0) _gate_q_1;
}
gate cu3_127318274384112(_gate_p_0, _gate_p_1, _gate_p_2) _gate_q_0, _gate_q_1 {
  u1(3.244247164193524) _gate_q_0;
  u1(1.0119981946537315) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(-1.01498744876671, 0, -3.244247164193524) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(1.01498744876671, 2.2322489695397927, 0) _gate_q_1;
}
gate rxx_127318274387808(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_0;
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(5.172455064375235) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
  h _gate_q_0;
}
gate xx_plus_yy_127318274388096(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(1.7943321069038343) _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sx _gate_q_1;
  rz(pi/2) _gate_q_1;
  s _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  ry(-1.0607347285012518) _gate_q_1;
  ry(-1.0607347285012518) _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  sdg _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sxdg _gate_q_1;
  rz(pi/2) _gate_q_1;
  rz(-1.7943321069038343) _gate_q_0;
}
gate ryy_127318274388432(_gate_p_0) _gate_q_0, _gate_q_1 {
  rx(pi/2) _gate_q_0;
  rx(pi/2) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(4.183670189392245) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rx(-pi/2) _gate_q_0;
  rx(-pi/2) _gate_q_1;
}
gate r_127318274386944(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(1.6125750327933792, -0.9534196196632584, 0.9534196196632584) _gate_q_0;
}
gate xx_plus_yy_127318274384640(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(1.8840477111833833) _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sx _gate_q_1;
  rz(pi/2) _gate_q_1;
  s _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  ry(-2.7066633825086144) _gate_q_1;
  ry(-2.7066633825086144) _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  sdg _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sxdg _gate_q_1;
  rz(pi/2) _gate_q_1;
  rz(-1.8840477111833833) _gate_q_0;
}
gate rxx_127318274388768(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_0;
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(2.8927678311990483) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
  h _gate_q_0;
}
gate xx_plus_yy_127318274387568(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(3.055481972089687) _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sx _gate_q_1;
  rz(pi/2) _gate_q_1;
  s _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  ry(-1.8976702356810746) _gate_q_1;
  ry(-1.8976702356810746) _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  sdg _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sxdg _gate_q_1;
  rz(pi/2) _gate_q_1;
  rz(-3.055481972089687) _gate_q_0;
}
gate iswap _gate_q_0, _gate_q_1 {
  s _gate_q_0;
  s _gate_q_1;
  h _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  cx _gate_q_1, _gate_q_0;
  h _gate_q_1;
}
gate r_127318274385024(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(3.9499501430075603, 4.0221593040797, -4.0221593040797) _gate_q_0;
}
gate cu3_127318274386848(_gate_p_0, _gate_p_1, _gate_p_2) _gate_q_0, _gate_q_1 {
  u1(4.367175932665644) _gate_q_0;
  u1(1.2649705832809415) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(-1.4562201164526272, 0, -4.367175932665644) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(1.4562201164526272, 3.1022053493847026, 0) _gate_q_1;
}
gate rxx_127318274385888(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_0;
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(1.9179420844199868) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
  h _gate_q_0;
}
gate rzx_127318274387328(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(4.563234042599816) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate xx_plus_yy_127318274385552(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(5.821010231021101) _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sx _gate_q_1;
  rz(pi/2) _gate_q_1;
  s _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  ry(-0.5165618901908796) _gate_q_1;
  ry(-0.5165618901908796) _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  sdg _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sxdg _gate_q_1;
  rz(pi/2) _gate_q_1;
  rz(-5.821010231021101) _gate_q_0;
}
gate rzz_127318274387232(_gate_p_0) _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  rz(3.8384193717506254) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
}
gate r_127318274386704(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(1.8039720029528143, -0.7479294470458436, 0.7479294470458436) _gate_q_0;
}
gate r_127318274386368(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(3.8216020782505886, 0.6462611805264373, -0.6462611805264373) _gate_q_0;
}
gate xx_plus_yy_127318274385936(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(1.6724460671552632) _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sx _gate_q_1;
  rz(pi/2) _gate_q_1;
  s _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  ry(-0.15676107694464317) _gate_q_1;
  ry(-0.15676107694464317) _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  sdg _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sxdg _gate_q_1;
  rz(pi/2) _gate_q_1;
  rz(-1.6724460671552632) _gate_q_0;
}
gate rzx_127318274386320(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(1.131751986372912) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate rzx_127318274383680(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(5.495848405459647) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate rzz_127318274384880(_gate_p_0) _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  rz(4.582759565155751) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
}
gate rzz_127318274386176(_gate_p_0) _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  rz(3.8895470149565172) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
}
gate ccz _gate_q_0, _gate_q_1, _gate_q_2 {
  h _gate_q_2;
  ccx _gate_q_0, _gate_q_1, _gate_q_2;
  h _gate_q_2;
}
gate rxx_127318274384352(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_0;
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(4.801432723385241) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
  h _gate_q_0;
}
gate cu3_127318274385456(_gate_p_0, _gate_p_1, _gate_p_2) _gate_q_0, _gate_q_1 {
  u1(2.578867254522765) _gate_q_0;
  u1(2.2031037457230114) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(-0.4832140888906387, 0, -2.578867254522765) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(0.4832140888906387, 0.37576350879975384, 0) _gate_q_1;
}
gate r_127318274384016(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(2.2637529831471865, 2.6896942706375553, -2.6896942706375553) _gate_q_0;
}
gate cu3_127318274384448(_gate_p_0, _gate_p_1, _gate_p_2) _gate_q_0, _gate_q_1 {
  u1(1.8499057396680123) _gate_q_0;
  u1(-0.5675112242506536) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(-2.9178311096267966, 0, -1.8499057396680123) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(2.9178311096267966, 2.417416963918666, 0) _gate_q_1;
}
gate r_127318274384976(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(0.48311409617623485, 0.5056023030847205, -0.5056023030847205) _gate_q_0;
}
gate r_127318274384160(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(1.1107109290679629, -0.769835336966497, 0.769835336966497) _gate_q_0;
}
gate xx_plus_yy_127318274384304(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(5.540619186047484) _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sx _gate_q_1;
  rz(pi/2) _gate_q_1;
  s _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  ry(-0.9803435088368855) _gate_q_1;
  ry(-0.9803435088368855) _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  sdg _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sxdg _gate_q_1;
  rz(pi/2) _gate_q_1;
  rz(-5.540619186047484) _gate_q_0;
}
gate xx_minus_yy_127318274383152(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(-2.88266642781013) _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sx _gate_q_0;
  rz(pi/2) _gate_q_0;
  s _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  ry(1.4443075616989562) _gate_q_0;
  ry(-1.4443075616989562) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  sdg _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sxdg _gate_q_0;
  rz(pi/2) _gate_q_0;
  rz(2.88266642781013) _gate_q_1;
}
gate xx_minus_yy_127318274384688(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(-5.620770080809869) _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sx _gate_q_0;
  rz(pi/2) _gate_q_0;
  s _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  ry(1.1324780958258864) _gate_q_0;
  ry(-1.1324780958258864) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  sdg _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sxdg _gate_q_0;
  rz(pi/2) _gate_q_0;
  rz(5.620770080809869) _gate_q_1;
}
gate xx_minus_yy_127318274383632(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(-1.1602840612568115) _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sx _gate_q_0;
  rz(pi/2) _gate_q_0;
  s _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  ry(1.8868358852393345) _gate_q_0;
  ry(-1.8868358852393345) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  sdg _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sxdg _gate_q_0;
  rz(pi/2) _gate_q_0;
  rz(1.1602840612568115) _gate_q_1;
}
gate rzx_127318274382240(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(5.500951768588432) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate rzx_127318522225504(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(pi/4) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate rzx_127318522225552(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(-pi/4) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate ecr _gate_q_0, _gate_q_1 {
  rzx_127318522225504(pi/4) _gate_q_0, _gate_q_1;
  x _gate_q_0;
  rzx_127318522225552(-pi/4) _gate_q_0, _gate_q_1;
}
gate xx_minus_yy_127318274381904(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(-4.871382112094206) _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sx _gate_q_0;
  rz(pi/2) _gate_q_0;
  s _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  ry(3.0068508229562942) _gate_q_0;
  ry(-3.0068508229562942) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  sdg _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sxdg _gate_q_0;
  rz(pi/2) _gate_q_0;
  rz(4.871382112094206) _gate_q_1;
}
gate cu3_127318274384064(_gate_p_0, _gate_p_1, _gate_p_2) _gate_q_0, _gate_q_1 {
  u1(0.29150185087047487) _gate_q_0;
  u1(0.21268557555248424) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(-0.456603021823918, 0, -0.29150185087047487) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(0.456603021823918, 0.07881627531799065, 0) _gate_q_1;
}
gate r_127318274380992(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(3.5905730401872584, -1.4406382757241418, 1.4406382757241418) _gate_q_0;
}
gate r_127318274380800(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(1.8540982950278015, -0.19099214877274662, 0.19099214877274662) _gate_q_0;
}
gate cu1_127318274380560(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(2.2309407977798434) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-2.2309407977798434) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(2.2309407977798434) _gate_q_1;
}
gate cu1_127318274380656(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(2.9915627123558353) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-2.9915627123558353) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(2.9915627123558353) _gate_q_1;
}
gate rxx_127318274379504(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_0;
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(1.2270639865673247) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
  h _gate_q_0;
}
gate xx_plus_yy_127318274379456(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(5.133189430225143) _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sx _gate_q_1;
  rz(pi/2) _gate_q_1;
  s _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  ry(-0.9236944025972974) _gate_q_1;
  ry(-0.9236944025972974) _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  sdg _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sxdg _gate_q_1;
  rz(pi/2) _gate_q_1;
  rz(-5.133189430225143) _gate_q_0;
}
gate r_127318274380176(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(2.173148337061091, 2.0827659257354836, -2.0827659257354836) _gate_q_0;
}
gate rzx_127318274379168(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(5.1338960000727) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate xx_minus_yy_127318274378640(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(-0.8414588003941097) _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sx _gate_q_0;
  rz(pi/2) _gate_q_0;
  s _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  ry(2.769130850280771) _gate_q_0;
  ry(-2.769130850280771) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  sdg _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sxdg _gate_q_0;
  rz(pi/2) _gate_q_0;
  rz(0.8414588003941097) _gate_q_1;
}
gate cu3_127318274378448(_gate_p_0, _gate_p_1, _gate_p_2) _gate_q_0, _gate_q_1 {
  u1(3.7447093670809215) _gate_q_0;
  u1(-0.6178298367597919) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(-1.9194993031882508, 0, -3.7447093670809215) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(1.9194993031882508, 4.362539203840713, 0) _gate_q_1;
}
gate r_127318274381040(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(3.39764311779516, 0.15723385272588564, -0.15723385272588564) _gate_q_0;
}
gate xx_minus_yy_127318274378736(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(-2.5850027914212976) _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sx _gate_q_0;
  rz(pi/2) _gate_q_0;
  s _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  ry(0.5552925952466372) _gate_q_0;
  ry(-0.5552925952466372) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  sdg _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sxdg _gate_q_0;
  rz(pi/2) _gate_q_0;
  rz(2.5850027914212976) _gate_q_1;
}
gate xx_minus_yy_127318274378256(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(-5.257506547431099) _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sx _gate_q_0;
  rz(pi/2) _gate_q_0;
  s _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  ry(2.195379454274083) _gate_q_0;
  ry(-2.195379454274083) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  sdg _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sxdg _gate_q_0;
  rz(pi/2) _gate_q_0;
  rz(5.257506547431099) _gate_q_1;
}
gate xx_plus_yy_127318274377776(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(4.57615583796259) _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sx _gate_q_1;
  rz(pi/2) _gate_q_1;
  s _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  ry(-0.07821284107577313) _gate_q_1;
  ry(-0.07821284107577313) _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  sdg _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sxdg _gate_q_1;
  rz(pi/2) _gate_q_1;
  rz(-4.57615583796259) _gate_q_0;
}
gate xx_minus_yy_127318274378160(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(-2.4936431800919636) _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sx _gate_q_0;
  rz(pi/2) _gate_q_0;
  s _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  ry(2.6541145679118645) _gate_q_0;
  ry(-2.6541145679118645) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  sdg _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sxdg _gate_q_0;
  rz(pi/2) _gate_q_0;
  rz(2.4936431800919636) _gate_q_1;
}
gate rzx_127318274377152(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(6.214906564987397) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate xx_minus_yy_127318274377296(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(-5.938004191678131) _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sx _gate_q_0;
  rz(pi/2) _gate_q_0;
  s _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  ry(1.670946664254882) _gate_q_0;
  ry(-1.670946664254882) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  sdg _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sxdg _gate_q_0;
  rz(pi/2) _gate_q_0;
  rz(5.938004191678131) _gate_q_1;
}
gate xx_minus_yy_127318274378928(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(-2.682274363951061) _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sx _gate_q_0;
  rz(pi/2) _gate_q_0;
  s _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  ry(1.118595047167573) _gate_q_0;
  ry(-1.118595047167573) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  sdg _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sxdg _gate_q_0;
  rz(pi/2) _gate_q_0;
  rz(2.682274363951061) _gate_q_1;
}
gate cu3_127318274376192(_gate_p_0, _gate_p_1, _gate_p_2) _gate_q_0, _gate_q_1 {
  u1(4.440187840317099) _gate_q_0;
  u1(1.405528777880026) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(-0.038326497871291554, 0, -4.440187840317099) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(0.038326497871291554, 3.0346590624370737, 0) _gate_q_1;
}
gate rzx_127318479226976(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(0.39299873788899803) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate ryy_127318479232256(_gate_p_0) _gate_q_0, _gate_q_1 {
  rx(pi/2) _gate_q_0;
  rx(pi/2) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(5.336003247489959) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rx(-pi/2) _gate_q_0;
  rx(-pi/2) _gate_q_1;
}
gate xx_plus_yy_127318274162224(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(5.113714673259361) _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sx _gate_q_1;
  rz(pi/2) _gate_q_1;
  s _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  ry(-0.6228038517101351) _gate_q_1;
  ry(-0.6228038517101351) _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  sdg _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sxdg _gate_q_1;
  rz(pi/2) _gate_q_1;
  rz(-5.113714673259361) _gate_q_0;
}
gate rzz_127318274161840(_gate_p_0) _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  rz(4.661217765271799) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
}
gate rzz_127318274162320(_gate_p_0) _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  rz(4.247689508841856) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
}
gate cu1_127318274161984(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(0.1942353257581867) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-0.1942353257581867) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(0.1942353257581867) _gate_q_1;
}
gate r_127318274161456(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(2.3132435595477068, 3.223884433533378, -3.223884433533378) _gate_q_0;
}
gate r_127318274161696(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(3.069095071015332, -0.3150141970717164, 0.3150141970717164) _gate_q_0;
}
gate cu3_127318274162080(_gate_p_0, _gate_p_1, _gate_p_2) _gate_q_0, _gate_q_1 {
  u1(2.6198467565113708) _gate_q_0;
  u1(0.2408961961810585) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(-1.5184314725164272, 0, -2.6198467565113708) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(1.5184314725164272, 2.378950560330312, 0) _gate_q_1;
}
gate rzz_127318274160976(_gate_p_0) _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  rz(2.105512280321992) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
}
gate rzx_127318274160688(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(2.9112373205655633) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate ryy_127318274160544(_gate_p_0) _gate_q_0, _gate_q_1 {
  rx(pi/2) _gate_q_0;
  rx(pi/2) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(0.2641393789231487) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rx(-pi/2) _gate_q_0;
  rx(-pi/2) _gate_q_1;
}
gate rzz_127318274160640(_gate_p_0) _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  rz(1.7527847362063975) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
}
gate xx_plus_yy_127318274160112(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(3.944355959674362) _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sx _gate_q_1;
  rz(pi/2) _gate_q_1;
  s _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  ry(-2.831499805626048) _gate_q_1;
  ry(-2.831499805626048) _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  sdg _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sxdg _gate_q_1;
  rz(pi/2) _gate_q_1;
  rz(-3.944355959674362) _gate_q_0;
}
gate r_127318274160448(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(3.6266720062428774, 2.2868464332834795, -2.2868464332834795) _gate_q_0;
}
gate rzx_127318274160352(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(5.220933499026929) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate cu1_127318274159728(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(0.9880428891235254) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-0.9880428891235254) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(0.9880428891235254) _gate_q_1;
}
gate ryy_127318274159200(_gate_p_0) _gate_q_0, _gate_q_1 {
  rx(pi/2) _gate_q_0;
  rx(pi/2) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(2.996753187462379) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rx(-pi/2) _gate_q_0;
  rx(-pi/2) _gate_q_1;
}
gate ryy_127318274159488(_gate_p_0) _gate_q_0, _gate_q_1 {
  rx(pi/2) _gate_q_0;
  rx(pi/2) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(5.053795451430822) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rx(-pi/2) _gate_q_0;
  rx(-pi/2) _gate_q_1;
}
gate ryy_127318274158288(_gate_p_0) _gate_q_0, _gate_q_1 {
  rx(pi/2) _gate_q_0;
  rx(pi/2) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(5.4005683679912355) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rx(-pi/2) _gate_q_0;
  rx(-pi/2) _gate_q_1;
}
gate rxx_127318274158096(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_0;
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(1.8360923824880313) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
  h _gate_q_0;
}
gate cu3_127318274158144(_gate_p_0, _gate_p_1, _gate_p_2) _gate_q_0, _gate_q_1 {
  u1(0.35592687725187344) _gate_q_0;
  u1(-0.1561585395635572) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(-1.6291236309314685, 0, -0.35592687725187344) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(1.6291236309314685, 0.5120854168154306, 0) _gate_q_1;
}
gate rzx_127318274157664(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(4.060578959490812) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate xx_plus_yy_127318274158048(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(1.0529923352495976) _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sx _gate_q_1;
  rz(pi/2) _gate_q_1;
  s _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  ry(-2.814852541239144) _gate_q_1;
  ry(-2.814852541239144) _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  sdg _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sxdg _gate_q_1;
  rz(pi/2) _gate_q_1;
  rz(-1.0529923352495976) _gate_q_0;
}
gate xx_minus_yy_127318274156944(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(-0.7436151678050824) _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sx _gate_q_0;
  rz(pi/2) _gate_q_0;
  s _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  ry(0.20222582175691847) _gate_q_0;
  ry(-0.20222582175691847) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  sdg _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sxdg _gate_q_0;
  rz(pi/2) _gate_q_0;
  rz(0.7436151678050824) _gate_q_1;
}
gate cu1_127318274157040(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(1.1448805285301502) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-1.1448805285301502) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(1.1448805285301502) _gate_q_1;
}
gate xx_minus_yy_127318274156896(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(-1.6744168026124415) _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sx _gate_q_0;
  rz(pi/2) _gate_q_0;
  s _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  ry(2.6576892507948218) _gate_q_0;
  ry(-2.6576892507948218) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  sdg _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sxdg _gate_q_0;
  rz(pi/2) _gate_q_0;
  rz(1.6744168026124415) _gate_q_1;
}
gate ryy_127318274156320(_gate_p_0) _gate_q_0, _gate_q_1 {
  rx(pi/2) _gate_q_0;
  rx(pi/2) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(1.4881562404180475) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rx(-pi/2) _gate_q_0;
  rx(-pi/2) _gate_q_1;
}
gate cu1_127318274157760(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(2.4130121246466234) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-2.4130121246466234) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(2.4130121246466234) _gate_q_1;
}
gate cu1_127318274155984(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(1.5303436762252864) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-1.5303436762252864) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(1.5303436762252864) _gate_q_1;
}
gate r_127318274155120(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(3.751180533539548, 2.026123843202276, -2.026123843202276) _gate_q_0;
}
gate rxx_127318274155072(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_0;
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(3.740892912913651) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
  h _gate_q_0;
}
gate rzz_127318274154784(_gate_p_0) _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  rz(5.812718523511535) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
}
gate rzz_127318274155792(_gate_p_0) _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  rz(4.2105352634956725) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
}
gate r_127318274154496(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(2.1245262873753754, 0.821800914949935, -0.821800914949935) _gate_q_0;
}
gate ryy_127318274154304(_gate_p_0) _gate_q_0, _gate_q_1 {
  rx(pi/2) _gate_q_0;
  rx(pi/2) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(2.345534187261207) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rx(-pi/2) _gate_q_0;
  rx(-pi/2) _gate_q_1;
}
gate rzz_127318274155456(_gate_p_0) _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  rz(1.9360078990517706) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
}
gate ryy_127318274154592(_gate_p_0) _gate_q_0, _gate_q_1 {
  rx(pi/2) _gate_q_0;
  rx(pi/2) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(5.340808672926667) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rx(-pi/2) _gate_q_0;
  rx(-pi/2) _gate_q_1;
}
gate cu1_127318274154640(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(3.1282407355816564) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-3.1282407355816564) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(3.1282407355816564) _gate_q_1;
}
gate r_127318274153680(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(6.236390377102064, 1.1676882841844183, -1.1676882841844183) _gate_q_0;
}
gate rxx_127318274153104(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_0;
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(3.838244140546363) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
  h _gate_q_0;
}
gate rzx_127318274152096(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(3.8600496852716337) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate r_127318274152720(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(5.264692206871664, -0.19409829870104645, 0.19409829870104645) _gate_q_0;
}
gate r_127318274151280(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(5.717995175144732, 1.8626700153615796, -1.8626700153615796) _gate_q_0;
}
gate rzz_127318274152000(_gate_p_0) _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  rz(3.4510829654861084) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
}
gate rzx_127318274150992(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(1.4133610932583593) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate rzx_127318274150560(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(4.398669146672025) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate rzx_127318274150176(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(5.264498832558266) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate xx_minus_yy_127318274150272(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(-0.5582498606148125) _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sx _gate_q_0;
  rz(pi/2) _gate_q_0;
  s _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  ry(1.6596462056040053) _gate_q_0;
  ry(-1.6596462056040053) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  sdg _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sxdg _gate_q_0;
  rz(pi/2) _gate_q_0;
  rz(0.5582498606148125) _gate_q_1;
}
gate r_127318274150464(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(0.17147807117965452, 1.4805282661214032, -1.4805282661214032) _gate_q_0;
}
gate cu1_127318274149600(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(0.6807747032394699) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-0.6807747032394699) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(0.6807747032394699) _gate_q_1;
}
gate cu3_127318274148688(_gate_p_0, _gate_p_1, _gate_p_2) _gate_q_0, _gate_q_1 {
  u1(1.636215475057608) _gate_q_0;
  u1(-1.548140076752063) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(-2.0805481879231635, 0, -1.636215475057608) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(2.0805481879231635, 3.184355551809671, 0) _gate_q_1;
}
gate xx_minus_yy_127318274148784(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(-5.638609489192684) _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sx _gate_q_0;
  rz(pi/2) _gate_q_0;
  s _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  ry(1.2042511670543472) _gate_q_0;
  ry(-1.2042511670543472) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  sdg _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sxdg _gate_q_0;
  rz(pi/2) _gate_q_0;
  rz(5.638609489192684) _gate_q_1;
}
gate r_127318274148352(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(4.849877483455675, 4.228323245937174, -4.228323245937174) _gate_q_0;
}
gate cu1_127318274147728(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(1.7417106804612459) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-1.7417106804612459) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(1.7417106804612459) _gate_q_1;
}
gate xx_plus_yy_127318274147344(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(0.09982243169642702) _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sx _gate_q_1;
  rz(pi/2) _gate_q_1;
  s _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  ry(-1.2504168322704905) _gate_q_1;
  ry(-1.2504168322704905) _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  sdg _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sxdg _gate_q_1;
  rz(pi/2) _gate_q_1;
  rz(-0.09982243169642702) _gate_q_0;
}
gate r_127318274147488(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(3.8541268329485843, -1.4835512710932002, 1.4835512710932002) _gate_q_0;
}
gate xx_minus_yy_127318274146960(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(-2.398467318675467) _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sx _gate_q_0;
  rz(pi/2) _gate_q_0;
  s _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  ry(0.9048703663518596) _gate_q_0;
  ry(-0.9048703663518596) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  sdg _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sxdg _gate_q_0;
  rz(pi/2) _gate_q_0;
  rz(2.398467318675467) _gate_q_1;
}
gate ryy_127318274146624(_gate_p_0) _gate_q_0, _gate_q_1 {
  rx(pi/2) _gate_q_0;
  rx(pi/2) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(5.738348200736418) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rx(-pi/2) _gate_q_0;
  rx(-pi/2) _gate_q_1;
}
gate cu3_127318274146720(_gate_p_0, _gate_p_1, _gate_p_2) _gate_q_0, _gate_q_1 {
  u1(4.1677210710681525) _gate_q_0;
  u1(1.914309859726133) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(-1.4554436458080675, 0, -4.1677210710681525) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(1.4554436458080675, 2.2534112113420197, 0) _gate_q_1;
}
gate r_127318276079104(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(4.836510669712646, 0.37944478839802165, -0.37944478839802165) _gate_q_0;
}
gate rxx_127318276078816(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_0;
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(2.6009230316951486) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
  h _gate_q_0;
}
gate cu1_127318276079056(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(0.34426990145405284) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-0.34426990145405284) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(0.34426990145405284) _gate_q_1;
}
gate r_127318276078384(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(1.7610159056436685, 3.3749944677380324, -3.3749944677380324) _gate_q_0;
}
gate r_127318276078480(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(2.833733802982907, 2.2625513145188014, -2.2625513145188014) _gate_q_0;
}
gate cu1_127318276078960(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(2.094024200655383) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-2.094024200655383) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(2.094024200655383) _gate_q_1;
}
gate r_127318276078096(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(0.23273002774870163, -1.5704050331084516, 1.5704050331084516) _gate_q_0;
}
gate xx_minus_yy_127318276078336(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(-1.9924643443486405) _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sx _gate_q_0;
  rz(pi/2) _gate_q_0;
  s _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  ry(0.5417181919569352) _gate_q_0;
  ry(-0.5417181919569352) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  sdg _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sxdg _gate_q_0;
  rz(pi/2) _gate_q_0;
  rz(1.9924643443486405) _gate_q_1;
}
gate r_127318276077136(_gate_p_0, _gate_p_1) _gate_q_0 {
  u3(5.790540124231192, 0.05733161866398451, -0.05733161866398451) _gate_q_0;
}
gate rzx_127318276077184(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(1.1470856059640049) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate cu1_127318276075984(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(0.23066900984341826) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-0.23066900984341826) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(0.23066900984341826) _gate_q_1;
}
gate cu1_127318276075216(_gate_p_0) _gate_q_0, _gate_q_1 {
  u1(1.740797298772855) _gate_q_0;
  cx _gate_q_0, _gate_q_1;
  u1(-1.740797298772855) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u1(1.740797298772855) _gate_q_1;
}
gate rzz_127318276075120(_gate_p_0) _gate_q_0, _gate_q_1 {
  cx _gate_q_0, _gate_q_1;
  rz(0.026207090733531748) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
}
gate rzx_127318276075168(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(2.6800544594456026) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
}
gate rxx_127318276074976(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_0;
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(0.5612930790855425) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
  h _gate_q_0;
}
gate cu3_127318276073824(_gate_p_0, _gate_p_1, _gate_p_2) _gate_q_0, _gate_q_1 {
  u1(1.4245635115941624) _gate_q_0;
  u1(1.3709576643824517) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(-0.5137753882320591, 0, -1.4245635115941624) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  u3(0.5137753882320591, 0.053605847211710904, 0) _gate_q_1;
}
gate rxx_127318276073728(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_0;
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(5.774306147663343) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
  h _gate_q_0;
}
gate rxx_127318276073488(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_0;
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(3.6891853201550795) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
  h _gate_q_0;
}
gate xx_minus_yy_127318276074544(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(-1.2372537055051864) _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sx _gate_q_0;
  rz(pi/2) _gate_q_0;
  s _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  ry(1.6276496718945388) _gate_q_0;
  ry(-1.6276496718945388) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  sdg _gate_q_1;
  rz(-pi/2) _gate_q_0;
  sxdg _gate_q_0;
  rz(pi/2) _gate_q_0;
  rz(1.2372537055051864) _gate_q_1;
}
gate ryy_127318276073104(_gate_p_0) _gate_q_0, _gate_q_1 {
  rx(pi/2) _gate_q_0;
  rx(pi/2) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(5.841702581833379) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rx(-pi/2) _gate_q_0;
  rx(-pi/2) _gate_q_1;
}
gate rxx_127318276072528(_gate_p_0) _gate_q_0, _gate_q_1 {
  h _gate_q_0;
  h _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  rz(4.151829324518218) _gate_q_1;
  cx _gate_q_0, _gate_q_1;
  h _gate_q_1;
  h _gate_q_0;
}
gate xx_plus_yy_127318276071760(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(4.436246572198519) _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sx _gate_q_1;
  rz(pi/2) _gate_q_1;
  s _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  ry(-2.0873787885309016) _gate_q_1;
  ry(-2.0873787885309016) _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  sdg _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sxdg _gate_q_1;
  rz(pi/2) _gate_q_1;
  rz(-4.436246572198519) _gate_q_0;
}
gate xx_plus_yy_127318276071808(_gate_p_0, _gate_p_1) _gate_q_0, _gate_q_1 {
  rz(2.3362162476259942) _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sx _gate_q_1;
  rz(pi/2) _gate_q_1;
  s _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  ry(-1.8890897485067002) _gate_q_1;
  ry(-1.8890897485067002) _gate_q_0;
  cx _gate_q_1, _gate_q_0;
  sdg _gate_q_0;
  rz(-pi/2) _gate_q_1;
  sxdg _gate_q_1;
  rz(pi/2) _gate_q_1;
  rz(-2.3362162476259942) _gate_q_0;
}
qubit[5] q;
rccx q[3], q[0], q[4];
dcx q[2], q[1];
rzx_127318481506848(2.2496982476760072) q[2], q[1];
u2(0.6598915089460567, 4.734116809229324) q[0];
r_127318481507184(5.375633947978526, 5.748091643697918) q[3];
h q[4];
t q[4];
r_127318481516448(2.7944186645604887, 5.325963815071495) q[3];
h q[0];
csx q[2], q[1];
csdg q[3], q[2];
cu1_127318487317056(2.5300982393740097) q[0], q[1];
t q[4];
rzx_127318487316912(4.3391512008224495) q[4], q[2];
ch q[1], q[0];
sxdg q[3];
s q[1];
ryy_127318487316624(5.537368951486815) q[0], q[4];
cu1_127318487316768(6.060714852067962) q[2], q[3];
rzz_127318274390736(4.80835203475167) q[1], q[2];
r_127318274391600(6.185557422911476, 2.865238552132769) q[0];
rx(0.651497848740195) q[3];
s q[4];
xx_minus_yy_127318274390784(1.6097395152312872, 3.720681675380936) q[2], q[1];
crz(4.777596606450901) q[3], q[4];
rz(2.0883603383405833) q[0];
h q[0];
rx(0.29380148506831183) q[2];
sx q[1];
y q[4];
r_127318274391936(6.264342157961093, 3.8049690756350847) q[3];
cs q[4], q[0];
dcx q[2], q[1];
rx(5.3495566081626835) q[3];
p(1.9772261721344844) q[0];
cx q[3], q[2];
crz(5.203130220465278) q[1], q[4];
rzx_127318274391504(5.105263126846988) q[3], q[4];
cry(4.773346726705486) q[0], q[1];
y q[2];
rxx_127318274391120(3.238588135065225) q[4], q[1];
cry(0.65475624424713) q[2], q[0];
id q[3];
rzz_127318274386896(6.110490760285972) q[4], q[0];
cswap q[2], q[3], q[1];
z q[2];
crx(3.6066607150832315) q[1], q[4];
U(0.3116975612927075, 0.8489494571470392, 1.1148604565590152) q[3];
y q[0];
sdg q[2];
ryy_127318274390400(3.3481918802045634) q[3], q[1];
ry(0.8905370468301278) q[4];
tdg q[0];
csx q[4], q[2];
sdg q[1];
s q[3];
rz(6.116179090383784) q[0];
tdg q[4];
rzx_127318274387664(1.7655480440886615) q[1], q[2];
cry(3.995786968796252) q[0], q[3];
rcccx q[4], q[2], q[3], q[0];
u1(2.2486434640405113) q[1];
crx(1.8220441886182699) q[1], q[2];
cu1_127318274390064(5.849753275155557) q[4], q[0];
t q[3];
sdg q[2];
tdg q[0];
u1(1.2230118541001882) q[4];
sdg q[1];
u3(1.8362696144538044, 4.4660774102435195, 4.885566670562086) q[3];
rzz_127318274388048(2.5389416362715123) q[2], q[0];
U(4.259367446645384, 5.12532776828064, 5.902110334075423) q[1];
xx_plus_yy_127318274389536(2.100840509942915, 1.4984935074203312) q[3], q[4];
cz q[0], q[1];
sxdg q[3];
csx q[4], q[2];
cry(4.261746736954605) q[2], q[0];
crz(4.7404411584767425) q[1], q[3];
sxdg q[4];
c3sx q[0], q[4], q[1], q[3];
u1(5.86230347683199) q[2];
ch q[4], q[0];
cp(5.581272342061126) q[2], q[1];
U(1.781196725225999, 2.7198278707701964, 2.2304026506751087) q[3];
crx(3.5786870810962617) q[4], q[0];
rccx q[1], q[3], q[2];
cy q[2], q[3];
dcx q[4], q[0];
ry(5.876625324647503) q[1];
r_127318274389584(4.799944822028326, 4.690660744005517) q[2];
ry(5.32678672640607) q[3];
cu3_127318274390160(4.559375083313329, 0.8771469897121793, 4.723660122081631) q[1], q[4];
tdg q[0];
rzx_127318274389392(1.086401886664922) q[4], q[0];
U(1.8499491560339998, 3.4918694366214074, 2.8075622752491984) q[3];
cu3_127318274389872(0.23065288567125547, 3.7235654226542816, 1.621236634735982) q[2], q[1];
u1(0.010814948483778205) q[4];
ry(1.9885970517165854) q[0];
cu3_127318274384112(2.02997489753342, 2.2322489695397927, 4.256245358847256) q[2], q[1];
ry(5.296463065745882) q[3];
cx q[2], q[3];
cswap q[0], q[4], q[1];
ch q[0], q[2];
s q[1];
rxx_127318274387808(5.172455064375235) q[4], q[3];
csx q[1], q[0];
rz(1.7170058236239736) q[2];
z q[4];
p(2.389800456808423) q[3];
x q[0];
rz(5.93233281195481) q[2];
xx_plus_yy_127318274388096(2.1214694570025037, 1.7943321069038343) q[4], q[1];
h q[3];
ryy_127318274388432(4.183670189392245) q[0], q[1];
u1(2.548335032190825) q[2];
csdg q[4], q[3];
id q[4];
r_127318274386944(1.6125750327933792, 0.6173767071316382) q[2];
xx_plus_yy_127318274384640(5.413326765017229, 1.8840477111833833) q[0], q[1];
y q[3];
rxx_127318274388768(2.8927678311990483) q[2], q[3];
rz(0.22487699685053533) q[1];
u1(5.938288898222214) q[0];
z q[4];
u2(6.192655145606564, 2.230873088534608) q[0];
u1(4.909452556896646) q[4];
rz(5.833476283928587) q[3];
xx_plus_yy_127318274387568(3.7953404713621492, 3.055481972089687) q[2], q[1];
cs q[3], q[2];
u3(4.450806019425856, 4.076894699592162, 0.061977360828065174) q[1];
iswap q[0], q[4];
sx q[1];
crz(4.678594396219029) q[3], q[4];
cy q[2], q[0];
r_127318274385024(3.9499501430075603, 5.592955630874597) q[2];
rz(5.168589783498636) q[4];
cu(2.612644145389236, 2.0579819434351694, 5.8307355503837055, 4.61969167220445) q[1], q[0];
u3(3.8112812798101885, 5.329057477425111, 4.877186533683435) q[3];
csx q[1], q[2];
x q[4];
dcx q[0], q[3];
h q[3];
x q[4];
cu3_127318274386848(2.9124402329052543, 3.1022053493847026, 5.6321465159465856) q[1], q[0];
p(4.521189414630355) q[2];
sdg q[4];
swap q[2], q[1];
rxx_127318274385888(1.9179420844199868) q[0], q[3];
rz(3.3713031641258113) q[1];
cx q[4], q[2];
csx q[3], q[0];
rzx_127318274387328(4.563234042599816) q[2], q[3];
cry(2.7563538504752394) q[0], q[4];
sx q[1];
dcx q[3], q[0];
xx_plus_yy_127318274385552(1.0331237803817592, 5.821010231021101) q[1], q[2];
s q[4];
rzz_127318274387232(3.8384193717506254) q[4], q[1];
csdg q[2], q[3];
rx(2.3137299044221713) q[0];
cp(1.2479786326003677) q[2], q[4];
sx q[1];
sdg q[3];
p(1.3188218128414826) q[0];
cry(6.096381559036595) q[3], q[2];
swap q[4], q[0];
r_127318274386704(1.8039720029528143, 0.8228668797490529) q[1];
csdg q[3], q[0];
rx(2.9556117280460112) q[1];
t q[2];
u3(0.8421789988094978, 5.301952968284809, 2.907999024550707) q[4];
r_127318274386368(3.8216020782505886, 2.217057507321334) q[2];
xx_plus_yy_127318274385936(0.31352215388928634, 1.6724460671552632) q[1], q[0];
sx q[4];
rz(6.053948933438855) q[3];
cry(0.2599099384346256) q[4], q[1];
sx q[3];
sdg q[0];
s q[2];
s q[4];
rzx_127318274386320(1.131751986372912) q[3], q[2];
csx q[1], q[0];
cx q[1], q[4];
rx(5.096175951446251) q[0];
rzx_127318274383680(5.495848405459647) q[2], q[3];
csdg q[2], q[3];
u3(1.7949178464525193, 3.844501090183846, 3.568449663847496) q[0];
cp(5.320124092423286) q[4], q[1];
swap q[1], q[2];
rzz_127318274384880(4.582759565155751) q[4], q[0];
rx(3.604033334710062) q[3];
csx q[4], q[0];
rzz_127318274386176(3.8895470149565172) q[3], q[2];
u2(3.59338425145813, 0.7629246776874555) q[1];
cry(4.797436803149853) q[3], q[0];
ccz q[1], q[2], q[4];
swap q[0], q[2];
ccx q[3], q[1], q[4];
rxx_127318274384352(4.801432723385241) q[1], q[2];
cu3_127318274385456(0.9664281777812774, 0.37576350879975384, 4.781971000245776) q[4], q[0];
u3(1.378075867628125, 0.011845340219672283, 2.8270152081825883) q[3];
cp(2.6059121993657053) q[1], q[0];
r_127318274384016(2.2637529831471865, 4.260490597432452) q[2];
tdg q[3];
t q[4];
cu3_127318274384448(5.835662219253593, 2.417416963918666, 1.2823945154173588) q[1], q[2];
cswap q[0], q[3], q[4];
dcx q[2], q[3];
u2(2.537816066041769, 0.21832557956425924) q[0];
sx q[4];
r_127318274384976(0.48311409617623485, 2.076398629879617) q[1];
r_127318274384160(1.1107109290679629, 0.8009609898283996) q[3];
csx q[2], q[0];
cs q[4], q[1];
sxdg q[0];
xx_plus_yy_127318274384304(1.960687017673771, 5.540619186047484) q[2], q[4];
cy q[3], q[1];
xx_minus_yy_127318274383152(2.8886151233979125, 2.88266642781013) q[4], q[0];
xx_minus_yy_127318274384688(2.264956191651773, 5.620770080809869) q[1], q[2];
sxdg q[3];
ccz q[3], q[4], q[2];
z q[1];
p(4.732292111704204) q[0];
cx q[2], q[0];
cp(3.5314488098231225) q[4], q[1];
sdg q[3];
c3sx q[2], q[1], q[3], q[4];
u3(0.42760671069919187, 5.131269494429248, 1.630779146525389) q[0];
crx(2.255208190215272) q[3], q[2];
ry(2.2555561350122337) q[0];
ch q[1], q[4];
cry(4.405336286801408) q[4], q[1];
csdg q[3], q[0];
s q[2];
cx q[3], q[4];
y q[2];
cp(0.32684276627755254) q[0], q[1];
ccz q[3], q[0], q[1];
dcx q[2], q[4];
csdg q[1], q[2];
xx_minus_yy_127318274383632(3.773671770478669, 1.1602840612568115) q[4], q[3];
rz(1.3817759978102373) q[0];
cry(0.9013896975115678) q[0], q[2];
ch q[4], q[3];
x q[1];
sdg q[0];
sdg q[2];
sx q[3];
ch q[1], q[4];
u3(4.672251695907885, 5.478283529673472, 0.7305979398180038) q[4];
rcccx q[2], q[3], q[0], q[1];
rx(5.1934119669584895) q[2];
h q[1];
rx(1.800746379133698) q[0];
rzx_127318274382240(5.500951768588432) q[3], q[4];
u2(5.7032501439912515, 3.1839515867342794) q[1];
y q[2];
sx q[4];
rx(2.9268268100307275) q[0];
y q[3];
sdg q[1];
id q[4];
h q[2];
id q[3];
z q[0];
z q[3];
iswap q[2], q[0];
cy q[1], q[4];
y q[4];
ecr q[3], q[2];
iswap q[1], q[0];
cry(4.345372972569691) q[3], q[1];
crx(1.403164851319921) q[0], q[4];
u3(0.39104819617530834, 4.065022316010648, 0.022019692529715133) q[2];
x q[2];
xx_minus_yy_127318274381904(6.0137016459125885, 4.871382112094206) q[1], q[4];
csdg q[0], q[3];
y q[4];
csx q[3], q[0];
cry(0.5281222479153335) q[1], q[2];
rccx q[1], q[3], q[4];
cp(1.7205657508528898) q[2], q[0];
cu3_127318274384064(0.913206043647836, 0.07881627531799065, 0.5041874264229591) q[4], q[0];
dcx q[1], q[2];
sdg q[3];
t q[0];
rx(4.805425251089329) q[1];
r_127318274380992(3.5905730401872584, 0.13015805107075473) q[3];
sx q[4];
p(2.9219547734403752) q[2];
s q[1];
ecr q[4], q[3];
u2(1.2159707863132336, 6.240747933556812) q[0];
r_127318274380800(1.8540982950278015, 1.37980417802215) q[2];
dcx q[1], q[3];
cu1_127318274380560(4.461881595559687) q[4], q[0];
sxdg q[2];
cy q[0], q[3];
crz(0.850622162339262) q[2], q[4];
U(2.9893837703328536, 5.717981523464527, 3.278290561310717) q[1];
cy q[2], q[4];
cswap q[0], q[3], q[1];
t q[3];
iswap q[2], q[1];
u2(0.102856338301133, 5.876017792269575) q[0];
u3(0.5881456451686096, 5.626960105182106, 0.34851225474427194) q[4];
csdg q[2], q[1];
cz q[0], q[3];
s q[4];
cswap q[3], q[2], q[1];
cu1_127318274380656(5.9831254247116705) q[0], q[4];
cs q[2], q[0];
ccz q[1], q[4], q[3];
ecr q[1], q[2];
cx q[4], q[3];
U(5.301995561667131, 4.950552320988975, 4.900752322140707) q[0];
rxx_127318274379504(1.2270639865673247) q[2], q[1];
ccz q[4], q[0], q[3];
cswap q[4], q[0], q[3];
xx_plus_yy_127318274379456(1.8473888051945948, 5.133189430225143) q[1], q[2];
r_127318274380176(2.173148337061091, 3.65356225253038) q[2];
ry(4.288011205521896) q[0];
p(1.641782854951801) q[1];
id q[4];
U(2.8922778683714747, 5.337396471407287, 2.4876825556878845) q[3];
rzx_127318274379168(5.1338960000727) q[2], q[3];
cy q[0], q[4];
U(0.4165822789014336, 1.7667023772054264, 4.067208116343589) q[1];
ccx q[4], q[1], q[2];
sx q[0];
u2(5.296198665517217, 5.567469914879786) q[3];
ch q[0], q[1];
u1(0.7437414111980893) q[4];
y q[2];
ry(5.120621782391008) q[3];
xx_minus_yy_127318274378640(5.538261700561542, 0.8414588003941097) q[3], q[4];
y q[0];
u1(4.652471414443505) q[1];
sxdg q[2];
rccx q[3], q[2], q[0];
cu3_127318274378448(3.8389986063765016, 4.362539203840713, 3.1268795303211294) q[4], q[1];
ccx q[4], q[3], q[2];
cy q[0], q[1];
x q[1];
cry(1.0649963017121897) q[0], q[3];
r_127318274381040(3.39764311779516, 1.7280301795207822) q[4];
u2(1.4512235533318305, 0.2783878054862104) q[2];
u3(3.2350737567646024, 1.4461299440736282, 1.8005528625648444) q[2];
y q[1];
xx_minus_yy_127318274378736(1.1105851904932744, 2.5850027914212976) q[0], q[4];
sdg q[3];
ecr q[0], q[4];
s q[2];
xx_minus_yy_127318274378256(4.390758908548166, 5.257506547431099) q[3], q[1];
u3(1.1263398001116143, 6.216908595989184, 2.152606710938276) q[1];
ry(6.037490713727492) q[4];
ccx q[2], q[3], q[0];
u1(4.139170097518792) q[3];
xx_plus_yy_127318274377776(0.15642568215154626, 4.57615583796259) q[2], q[1];
ecr q[4], q[0];
xx_minus_yy_127318274378160(5.308229135823729, 2.4936431800919636) q[3], q[1];
cu(4.1329951695761, 4.997562739912371, 2.693569790460426, 2.0605256936778242) q[4], q[0];
ry(4.631651507380006) q[2];
tdg q[0];
ccx q[1], q[4], q[3];
sdg q[2];
dcx q[3], q[1];
crx(3.5974143390814612) q[0], q[4];
s q[2];
sxdg q[2];
c3sx q[0], q[4], q[3], q[1];
p(5.6806734457673365) q[4];
csx q[1], q[3];
rzx_127318274377152(6.214906564987397) q[2], q[0];
iswap q[2], q[3];
h q[1];
rz(5.771674974967442) q[0];
ry(2.4119795075547255) q[4];
t q[4];
cx q[1], q[0];
xx_minus_yy_127318274377296(3.341893328509764, 5.938004191678131) q[3], q[2];
sdg q[0];
rcccx q[1], q[2], q[3], q[4];
c3sx q[2], q[1], q[4], q[0];
id q[3];
xx_minus_yy_127318274378928(2.237190094335146, 2.682274363951061) q[2], q[3];
z q[1];
id q[4];
z q[0];
cs q[2], q[1];
u1(3.5689466182139933) q[4];
iswap q[0], q[3];
csx q[1], q[0];
cp(3.941599662755979) q[2], q[4];
p(2.259731659123908) q[3];
t q[4];
u2(4.083217033495944, 4.736890948400934) q[0];
rz(1.9363858377047383) q[3];
u1(1.94145632026428) q[2];
u3(4.310994299174199, 2.041636828262335, 3.972176562843455) q[1];
c3sx q[3], q[0], q[2], q[4];
rz(3.7016661123789567) q[1];
cu3_127318274376192(0.07665299574258311, 3.0346590624370737, 5.845716618197126) q[1], q[4];
csx q[2], q[3];
ry(4.60730033261851) q[0];
U(3.782601776405413, 5.421050205007441, 4.9077446855325615) q[0];
s q[3];
cp(0.24899542141917735) q[2], q[1];
sx q[4];
rzx_127318479226976(0.39299873788899803) q[4], q[2];
ryy_127318479232256(5.336003247489959) q[1], q[0];
ry(2.9587449396407806) q[3];
xx_plus_yy_127318274162224(1.2456077034202702, 5.113714673259361) q[2], q[0];
rzz_127318274161840(4.661217765271799) q[3], q[4];
tdg q[1];
rzz_127318274162320(4.247689508841856) q[3], q[4];
cu1_127318274161984(0.3884706515163734) q[2], q[1];
x q[0];
cry(0.27531096372723324) q[1], q[2];
U(2.792145984245659, 1.8550953277687467, 2.3786643107089533) q[3];
ry(1.207014079252322) q[4];
r_127318274161456(2.3132435595477068, 4.794680760328275) q[0];
ch q[3], q[1];
swap q[4], q[0];
sdg q[2];
p(3.5093455244704272) q[1];
tdg q[2];
p(2.1680415363626233) q[3];
u1(3.5449020441769297) q[4];
y q[0];
ch q[4], q[2];
csx q[1], q[3];
s q[0];
id q[2];
sxdg q[3];
r_127318274161696(3.069095071015332, 1.2557821297231802) q[0];
dcx q[4], q[1];
cu3_127318274162080(3.0368629450328544, 2.378950560330312, 2.860742952692429) q[0], q[2];
ry(2.3987658743341584) q[3];
ecr q[4], q[1];
rzz_127318274160976(2.105512280321992) q[1], q[4];
dcx q[3], q[2];
u2(5.822250413744008, 5.772016858090265) q[0];
ry(2.9382471994544876) q[4];
cs q[0], q[1];
rzx_127318274160688(2.9112373205655633) q[2], q[3];
ryy_127318274160544(0.2641393789231487) q[2], q[1];
rzz_127318274160640(1.7527847362063975) q[3], q[0];
u3(2.57613870688867, 6.218278598763148, 4.060585650117373) q[4];
cswap q[4], q[0], q[2];
x q[1];
rz(4.168729218268444) q[3];
z q[4];
xx_plus_yy_127318274160112(5.662999611252096, 3.944355959674362) q[0], q[2];
sdg q[3];
sxdg q[1];
cy q[2], q[4];
sx q[1];
sdg q[0];
x q[3];
r_127318274160448(3.6266720062428774, 3.857642760078376) q[4];
crz(2.9180810464338274) q[2], q[0];
rzx_127318274160352(5.220933499026929) q[1], q[3];
crx(4.62847881318369) q[1], q[2];
cswap q[3], q[4], q[0];
rx(0.19496887648070896) q[0];
cz q[4], q[3];
crx(2.163840786772696) q[1], q[2];
cu1_127318274159728(1.976085778247051) q[1], q[2];
csdg q[0], q[3];
u1(6.060991989520119) q[4];
ccx q[2], q[3], q[4];
crx(0.12979379627833865) q[1], q[0];
id q[3];
y q[0];
h q[2];
sdg q[4];
x q[1];
rz(6.257122183154674) q[2];
sxdg q[4];
rccx q[3], q[1], q[0];
t q[3];
cz q[1], q[2];
t q[4];
rx(0.2559703797404892) q[0];
u1(5.734033770104945) q[1];
u3(4.294434665956946, 4.996326261644157, 2.3140698889696325) q[2];
id q[0];
cy q[4], q[3];
csx q[2], q[4];
s q[0];
ryy_127318274159200(2.996753187462379) q[3], q[1];
sx q[1];
ryy_127318274159488(5.053795451430822) q[3], q[2];
u2(0.9883836431484001, 5.451690365893319) q[0];
u2(2.0916728098764756, 6.200850300696747) q[4];
h q[4];
y q[2];
h q[3];
h q[1];
ry(1.9760905932114583) q[0];
ryy_127318274158288(5.4005683679912355) q[1], q[3];
cry(5.92857843656909) q[4], q[0];
rx(4.9369287524962) q[2];
rx(0.7197355182485855) q[1];
crx(5.63574947654557) q[0], q[3];
rz(3.8896229498662733) q[2];
x q[4];
t q[0];
cswap q[4], q[2], q[1];
sdg q[3];
swap q[0], q[1];
sxdg q[3];
rx(5.799533464111318) q[2];
x q[4];
h q[2];
ccz q[4], q[1], q[0];
sx q[3];
h q[4];
p(0.7184990541943045) q[0];
tdg q[2];
ecr q[3], q[1];
rxx_127318274158096(1.8360923824880313) q[4], q[2];
cu3_127318274158144(3.258247261862937, 0.5120854168154306, 0.1997683376883162) q[0], q[1];
U(2.9817659317129177, 1.8043048997201778, 3.2684551623042544) q[3];
ry(0.36492688289222663) q[3];
ccx q[2], q[0], q[4];
sx q[1];
rzx_127318274157664(4.060578959490812) q[3], q[2];
ccx q[0], q[4], q[1];
u3(1.1591559948678045, 0.4004548550083631, 2.3374282536284348) q[2];
xx_plus_yy_127318274158048(5.629705082478288, 1.0529923352495976) q[3], q[1];
ry(0.8997404740556826) q[4];
y q[0];
crz(5.201196862206918) q[4], q[2];
cy q[3], q[1];
tdg q[0];
cy q[4], q[3];
cz q[1], q[2];
id q[0];
cz q[1], q[3];
csx q[0], q[4];
y q[2];
cu(0.33868747613514116, 0.4370192534808439, 0.749709995096389, 4.020572495832461) q[2], q[1];
ry(4.263509186409283) q[0];
csdg q[3], q[4];
u2(2.8344617231793983, 1.813379969639575) q[0];
cswap q[4], q[3], q[2];
sxdg q[1];
xx_minus_yy_127318274156944(0.40445164351383694, 0.7436151678050824) q[3], q[4];
cu1_127318274157040(2.2897610570603004) q[2], q[0];
rz(0.07696849339684284) q[1];
xx_minus_yy_127318274156896(5.3153785015896435, 1.6744168026124415) q[4], q[2];
z q[1];
s q[0];
z q[3];
iswap q[4], q[3];
U(5.643715934764551, 5.444363476983645, 5.1723134302152) q[2];
u2(5.856068630688819, 4.967851949178397) q[1];
z q[0];
z q[2];
u1(2.573682963475838) q[4];
z q[3];
x q[0];
z q[1];
z q[1];
cx q[0], q[2];
rx(1.9582274161543132) q[3];
p(0.7221180626611927) q[4];
dcx q[2], q[3];
ry(4.705621923720372) q[0];
u1(4.290058270460444) q[1];
rx(2.237101708118455) q[4];
cs q[2], q[4];
ecr q[1], q[3];
rx(3.444172463982561) q[0];
sdg q[4];
ryy_127318274156320(1.4881562404180475) q[0], q[1];
cz q[2], q[3];
csdg q[0], q[1];
rccx q[3], q[4], q[2];
cswap q[0], q[1], q[2];
U(3.7520891823021563, 5.880949084050483, 4.2887800961521565) q[4];
ry(4.120566302576873) q[3];
csx q[4], q[1];
cu1_127318274157760(4.826024249293247) q[3], q[0];
sxdg q[2];
t q[2];
cu1_127318274155984(3.060687352450573) q[4], q[3];
crz(5.908232822918342) q[0], q[1];
rccx q[3], q[0], q[4];
sxdg q[1];
sxdg q[2];
ry(0.43339971379964176) q[1];
ccx q[3], q[0], q[2];
rx(6.199247159851627) q[4];
r_127318274155120(3.751180533539548, 3.5969201699971727) q[4];
u1(2.5345940972514156) q[3];
ch q[1], q[0];
id q[2];
rxx_127318274155072(3.740892912913651) q[2], q[1];
rzz_127318274154784(5.812718523511535) q[3], q[4];
sx q[0];
rzz_127318274155792(4.2105352634956725) q[0], q[2];
r_127318274154496(2.1245262873753754, 2.3925972417448316) q[1];
ryy_127318274154304(2.345534187261207) q[4], q[3];
t q[2];
cswap q[0], q[3], q[4];
id q[1];
rzz_127318274155456(1.9360078990517706) q[1], q[3];
tdg q[2];
cp(1.8399396237806496) q[4], q[0];
ryy_127318274154592(5.340808672926667) q[3], q[1];
cu1_127318274154640(6.256481471163313) q[4], q[0];
s q[2];
swap q[3], q[2];
ccx q[1], q[4], q[0];
rcccx q[3], q[0], q[1], q[2];
rx(3.981009746143599) q[4];
s q[4];
tdg q[2];
U(3.0852610803341403, 6.112301605401776, 3.2041602945741796) q[0];
crz(0.1764499917717279) q[3], q[1];
cp(5.996545654411489) q[0], q[3];
cry(0.323760455341976) q[1], q[4];
p(0.21782425016070892) q[2];
sxdg q[3];
cs q[2], q[4];
p(6.2751698286263045) q[0];
U(1.7534739400591444, 0.1317110875695796, 1.863128410525404) q[1];
r_127318274153680(6.236390377102064, 2.738484610979315) q[1];
csdg q[2], q[0];
cu(0.1872338667586472, 1.625571559117273, 1.7527221722033823, 0.4171482443137115) q[3], q[4];
cu(3.5762333695494557, 3.5325847252834364, 4.451001665537618, 4.881944995325157) q[3], q[1];
y q[0];
csdg q[4], q[2];
cswap q[2], q[4], q[0];
csx q[1], q[3];
rcccx q[3], q[1], q[0], q[4];
tdg q[2];
crz(3.2637916986932214) q[2], q[3];
rx(5.136045870986404) q[4];
p(1.2379262277313559) q[1];
sdg q[0];
ecr q[0], q[2];
z q[3];
rz(4.9926052318069365) q[1];
z q[4];
u1(1.6943532226484292) q[2];
iswap q[3], q[1];
U(6.182332837805802, 4.599150631911535, 1.2596113033736682) q[0];
sx q[4];
tdg q[4];
dcx q[2], q[3];
s q[0];
x q[1];
sxdg q[3];
y q[2];
rxx_127318274153104(3.838244140546363) q[4], q[0];
rz(2.1552280408174522) q[1];
sdg q[2];
s q[4];
U(2.7535756770510855, 1.8758265023794956, 2.131205337203193) q[1];
u3(5.365504515119414, 4.571311193239568, 1.299979008497789) q[3];
h q[0];
rx(5.218045863839746) q[0];
rzx_127318274152096(3.8600496852716337) q[1], q[4];
id q[3];
r_127318274152720(5.264692206871664, 1.3766980280938501) q[2];
swap q[4], q[0];
rz(3.564140851417618) q[2];
rx(5.189238739240813) q[3];
r_127318274151280(5.717995175144732, 3.433466342156476) q[1];
c3sx q[2], q[4], q[3], q[0];
tdg q[1];
sdg q[3];
crx(0.1402106516635958) q[0], q[4];
sxdg q[2];
u1(1.241965411456453) q[1];
cy q[2], q[4];
rzz_127318274152000(3.4510829654861084) q[1], q[3];
u2(0.48101947311158477, 1.1024241939797437) q[0];
ch q[3], q[1];
z q[4];
rz(5.172058753001693) q[0];
p(0.09465309231501676) q[2];
rccx q[3], q[0], q[2];
rx(1.5541019325009837) q[1];
u1(2.4035189193569293) q[4];
u3(0.896863716225172, 3.2275543075446596, 3.869703068065698) q[1];
cs q[4], q[2];
crx(4.155930735577645) q[0], q[3];
cs q[3], q[2];
rx(2.374927365611466) q[4];
z q[0];
id q[1];
rzx_127318274150992(1.4133610932583593) q[1], q[4];
rzx_127318274150560(4.398669146672025) q[0], q[3];
sxdg q[2];
csdg q[2], q[3];
crz(1.7234068229006572) q[0], q[1];
u2(5.719167856214146, 5.009602772783034) q[4];
rzx_127318274150176(5.264498832558266) q[1], q[4];
cswap q[3], q[0], q[2];
cy q[3], q[2];
xx_minus_yy_127318274150272(3.3192924112080107, 0.5582498606148125) q[0], q[1];
id q[4];
tdg q[2];
ccx q[4], q[1], q[0];
sx q[3];
r_127318274150464(0.17147807117965452, 3.0513245929162998) q[3];
sx q[4];
z q[2];
y q[1];
s q[0];
x q[4];
cy q[3], q[0];
ecr q[2], q[1];
rz(2.327401632868768) q[2];
crx(2.5571608617427124) q[0], q[3];
z q[1];
rz(5.797339864798011) q[4];
crz(5.1568877843674175) q[1], q[0];
sdg q[2];
cry(6.153219411271034) q[3], q[4];
sdg q[2];
U(6.006292605351524, 4.508966471825257, 4.573910027212873) q[4];
tdg q[0];
cu1_127318274149600(1.3615494064789397) q[3], q[1];
cz q[2], q[0];
sxdg q[1];
tdg q[3];
t q[4];
csdg q[1], q[4];
rx(1.494198442006517) q[0];
z q[2];
y q[3];
u2(1.6004845651476718, 1.5471666692868984) q[1];
cz q[0], q[4];
u2(3.7842330653397354, 4.153995462347227) q[3];
s q[2];
swap q[4], q[2];
crz(5.173956476854009) q[3], q[1];
ry(4.989120936930098) q[0];
cu3_127318274148688(4.161096375846327, 3.184355551809671, 0.08807539830554477) q[0], q[2];
cz q[3], q[4];
id q[1];
rz(2.16452640111904) q[1];
u2(2.199940839327788, 3.395911498333865) q[3];
u1(2.6073961796878535) q[4];
u3(0.2885432199098835, 5.135410149370114, 0.17685434068824304) q[2];
ry(3.8868861317734495) q[0];
z q[0];
xx_minus_yy_127318274148784(2.4085023341086944, 5.638609489192684) q[4], q[3];
tdg q[1];
u2(0.05650984400293282, 3.8382232268288075) q[2];
rcccx q[0], q[2], q[4], q[3];
x q[1];
u3(0.6015276644787112, 2.8903486005548333, 0.5325603916251745) q[2];
crz(4.192263837948884) q[1], q[0];
sx q[3];
ry(0.8382014997380134) q[4];
ry(4.174192131045504) q[3];
h q[1];
ch q[4], q[2];
rz(3.856419949904041) q[0];
u1(3.857370220660721) q[4];
csx q[3], q[2];
r_127318274148352(4.849877483455675, 5.799119572732071) q[0];
z q[1];
sdg q[1];
csdg q[4], q[0];
sxdg q[2];
u2(2.1210864705426955, 4.667763844834596) q[3];
ecr q[1], q[0];
cu1_127318274147728(3.4834213609224918) q[3], q[4];
u1(0.5198433590497326) q[2];
xx_plus_yy_127318274147344(2.500833664540981, 0.09982243169642702) q[3], q[0];
U(2.773092255689258, 0.9636715668851039, 2.4123891920608966) q[1];
sx q[2];
t q[4];
ry(5.049982088317567) q[0];
swap q[1], q[4];
r_127318274147488(3.8541268329485843, 0.08724505570169644) q[2];
u1(2.8117075118846926) q[3];
xx_minus_yy_127318274146960(1.8097407327037192, 2.398467318675467) q[2], q[4];
s q[0];
z q[3];
s q[1];
U(0.7045770762470942, 2.5891823372338862, 1.7975857476365955) q[2];
iswap q[4], q[1];
ry(0.5017666837136383) q[0];
x q[3];
ccz q[4], q[0], q[3];
sx q[2];
id q[1];
cswap q[3], q[4], q[1];
u2(3.0484960389961238, 2.6977116717387752) q[2];
u1(0.5647962199701195) q[0];
y q[0];
cs q[2], q[4];
cu(4.85125366535689, 0.9082535278525115, 5.244030496894937, 4.95660211126588) q[1], q[3];
ryy_127318274146624(5.738348200736418) q[1], q[2];
cu3_127318274146720(2.910887291616135, 2.2534112113420197, 6.082030930794286) q[4], q[0];
sxdg q[3];
ry(0.2509851458915264) q[2];
u3(4.926342155573598, 1.717700868239552, 1.1275646331241396) q[0];
ccx q[3], q[1], q[4];
cx q[2], q[1];
u1(2.249139908643209) q[0];
sx q[4];
r_127318276079104(4.836510669712646, 1.9502411151929182) q[3];
rxx_127318276078816(2.6009230316951486) q[1], q[2];
cy q[0], q[3];
u2(5.352120042493151, 4.575450962642021) q[4];
y q[3];
sx q[2];
id q[4];
s q[1];
U(0.7648436781857513, 0.544742977058187, 5.988685830726626) q[0];
cu(0.5566468153298239, 0.6670711295941135, 3.072389706993817, 0.01747343797260968) q[2], q[0];
rccx q[3], q[4], q[1];
u2(5.814813138806912, 3.551604488434476) q[2];
u1(1.1802860218310671) q[1];
ccx q[4], q[3], q[0];
cu1_127318276079056(0.6885398029081057) q[3], q[0];
id q[2];
cz q[4], q[1];
cry(2.3376009821213644) q[0], q[4];
r_127318276078384(1.7610159056436685, 4.945790794532929) q[3];
csdg q[1], q[2];
r_127318276078480(2.833733802982907, 3.833347641313698) q[3];
x q[2];
cy q[4], q[0];
u3(0.5982734935928289, 2.0711962856773733, 1.6163975221320726) q[1];
cp(3.7858767558503668) q[0], q[4];
x q[2];
cu1_127318276078960(4.188048401310766) q[1], q[3];
ccz q[3], q[1], q[2];
z q[0];
sx q[4];
ecr q[4], q[1];
cs q[0], q[2];
tdg q[3];
crz(4.213090251909154) q[2], q[4];
cswap q[3], q[1], q[0];
U(0.2557292411739753, 3.177198165839475, 4.489166145179776) q[1];
r_127318276078096(0.23273002774870163, 0.0003912936864449834) q[0];
cswap q[4], q[3], q[2];
xx_minus_yy_127318276078336(1.0834363839138703, 1.9924643443486405) q[2], q[1];
cz q[3], q[4];
sxdg q[0];
cx q[1], q[4];
ccz q[0], q[3], q[2];
rcccx q[3], q[4], q[1], q[2];
rz(2.3686503197270423) q[0];
ccx q[4], q[0], q[1];
cs q[3], q[2];
rx(4.2878852980520055) q[4];
ccz q[1], q[0], q[3];
r_127318276077136(5.790540124231192, 1.628127945458881) q[2];
rcccx q[0], q[2], q[3], q[4];
y q[1];
y q[3];
rz(4.78551319970034) q[4];
cp(4.6084225174918005) q[1], q[2];
y q[0];
t q[2];
rzx_127318276077184(1.1470856059640049) q[0], q[1];
ch q[4], q[3];
sxdg q[4];
cp(5.307953845041364) q[2], q[0];
id q[3];
ry(1.78185044136665) q[1];
csx q[1], q[0];
iswap q[4], q[2];
sxdg q[3];
x q[1];
sx q[2];
cs q[0], q[4];
s q[3];
dcx q[3], q[1];
crx(2.714649139119403) q[4], q[2];
t q[0];
cry(3.957984880410235) q[3], q[4];
ecr q[2], q[1];
ry(1.384401566595402) q[0];
crz(3.302963784368486) q[3], q[0];
cp(2.7772049842719775) q[1], q[2];
x q[4];
t q[4];
cu1_127318276075984(0.4613380196868365) q[0], q[1];
dcx q[2], q[3];
rccx q[2], q[1], q[0];
x q[3];
u3(3.0708188456240784, 4.453224204332864, 4.554313180128185) q[4];
u2(2.9464456909839147, 5.839673239761143) q[4];
ry(2.2219257881535324) q[2];
cu1_127318276075216(3.48159459754571) q[3], q[0];
sdg q[1];
x q[4];
crz(3.059704746707251) q[1], q[3];
cu(4.019807396006542, 3.7494154189037365, 3.587567640200596, 4.926523239555409) q[2], q[0];
p(5.729206115844678) q[0];
rcccx q[1], q[4], q[3], q[2];
cswap q[3], q[2], q[0];
rzz_127318276075120(0.026207090733531748) q[1], q[4];
cswap q[1], q[4], q[0];
rzx_127318276075168(2.6800544594456026) q[2], q[3];
x q[3];
ry(0.7453231575832022) q[4];
u3(2.3734923560744785, 0.6021490238233492, 3.749580240463885) q[0];
cx q[2], q[1];
y q[3];
swap q[0], q[1];
sx q[2];
x q[4];
csdg q[4], q[0];
u2(3.765193811763537, 6.270305247630946) q[2];
cz q[3], q[1];
rxx_127318276074976(0.5612930790855425) q[3], q[1];
crx(4.847214593127084) q[4], q[0];
rz(0.8136644924488695) q[2];
t q[0];
sdg q[1];
csdg q[2], q[3];
x q[4];
s q[2];
x q[3];
cu3_127318276073824(1.0275507764641183, 0.053605847211710904, 2.795521175976614) q[4], q[0];
h q[1];
cu(1.1383155441612753, 0.21550294919518764, 4.02689443627039, 3.3431159998361912) q[1], q[2];
tdg q[0];
s q[4];
ry(4.1484396335586915) q[3];
rxx_127318276073728(5.774306147663343) q[4], q[1];
t q[3];
rx(0.1485914718426009) q[0];
u1(1.3376253915404468) q[2];
x q[4];
rxx_127318276073488(3.6891853201550795) q[1], q[0];
cz q[2], q[3];
xx_minus_yy_127318276074544(3.2552993437890776, 1.2372537055051864) q[4], q[3];
crx(5.010884436353664) q[1], q[0];
u3(2.3982655815194684, 3.967101805534164, 4.823224171528255) q[2];
cs q[3], q[4];
dcx q[1], q[0];
p(1.8499902680098512) q[2];
y q[4];
cz q[2], q[1];
U(2.111112003988309, 2.379667764205101, 4.417701332325105) q[3];
t q[0];
ry(1.320788729357891) q[1];
ryy_127318276073104(5.841702581833379) q[0], q[2];
rxx_127318276072528(4.151829324518218) q[4], q[3];
ch q[4], q[0];
h q[3];
cs q[2], q[1];
crx(4.812626806668313) q[4], q[2];
crx(5.8877699981316205) q[0], q[3];
rx(2.318062125669271) q[1];
id q[0];
csx q[3], q[4];
ry(0.08715440613057565) q[1];
U(5.412192992962227, 1.2023376980378235, 5.3384530520792515) q[2];
xx_plus_yy_127318276071760(4.174757577061803, 4.436246572198519) q[1], q[0];
s q[3];
cx q[4], q[2];
u2(0.37079887712405396, 2.319212591220979) q[1];
t q[0];
xx_plus_yy_127318276071808(3.7781794970134004, 2.3362162476259942) q[2], q[3];
sdg q[4];
cx q[3], q[2];
u1(4.322359555008806) q[1];
U(5.4754739967577954, 1.5268365097298155, 5.552214682865525) q[0];
h q[4];
