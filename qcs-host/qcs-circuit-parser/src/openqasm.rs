use std::{cell::RefCell, f64::consts::PI, path::Path};

use hashbrown::HashMap;
use oq3_semantics::{
    asg::{
        DeclareQuantum, Expr, GateCall, GateDeclaration, GateOperand, IndexOperator, Literal, Stmt,
        TExpr,
    },
    symbols::{SymbolId, SymbolTable, SymbolType},
    syntax_to_semantics::parse_source_file,
    types::{ArrayDims, Type},
};
use qcs_core::model::QuantumCircuit;

use crate::error::OwnedParserError;

pub fn parse_qasm_program(filename: impl AsRef<Path>) -> Result<QuantumCircuit, OwnedParserError> {
    let source_file = parse_source_file(filename.as_ref(), None::<&[&Path]>);
    let symbols = ExpandedSymbolTable::new(source_file.symbol_table());
    // source_file.program().print_asg_debug_pretty();
    // dbg!(symbols);
    let mut builder = CircuitBuilder::new();

    for stm in source_file.program().stmts() {
        match stm {
            // Define new gates as vector of ordered gates
            Stmt::GateDeclaration(d) => {
                let symbols = &symbols;
                let gate_name = symbols.get_symbol_name(d.name().as_ref().unwrap());
                let gate_params_names = get_def_params(d, symbols);
                let gate_qubits_names = get_def_qubits(d, symbols);

                let fun = move |circ: &CircuitBuilder, params: &[f64], lanes: &[usize]| {
                    for stm in d.block().statements() {
                        if let Stmt::GateCall(g) = stm {
                            let gate_name = symbols.get_symbol_name(g.name().as_ref().unwrap());
                            let params = get_call_params_expr(g)
                                .iter()
                                .map(|e| match e.expression() {
                                    Expr::Literal(Literal::Float(f)) => f.value().parse().unwrap(),
                                    Expr::Identifier(id) => {
                                        if id.name() == "pi" {
                                            PI
                                        } else {
                                            // dbg!(id, &gate_params_names);
                                            let ix = gate_params_names
                                                .iter()
                                                .position(|e| *e == id.name())
                                                .unwrap();
                                            params[ix]
                                        }
                                    }
                                    _ => 0.0,
                                })
                                .collect::<Vec<_>>();
                            let lanes = g
                                .qubits()
                                .iter()
                                .filter_map(|e| {
                                    let Expr::GateOperand(GateOperand::Identifier(id)) =
                                        e.expression()
                                    else {
                                        return None;
                                    };
                                    let ix = gate_qubits_names
                                        .iter()
                                        .position(|e| *e == id.name())
                                        .unwrap();
                                    Some(lanes[ix])
                                })
                                .collect::<Vec<_>>();
                            circ.add_gate(gate_name, &params, &lanes);
                        }
                    }
                };

                builder.add_gate_definition(gate_name.to_owned(), Box::new(fun));
            }
            // Find the number of qubits in the program
            Stmt::DeclareQuantum(q) => {
                builder.set_n_qubits(symbols.get_n_qubits(q));
            }
            // from now on we consider q as the quantum register of dimension n_qubits
            Stmt::GateCall(g) => {
                let gate_name = symbols.get_symbol_name(g.name().as_ref().unwrap());
                let params = get_call_params(g);
                let lanes = get_call_lanes(g);
                builder.add_gate(gate_name, &params, &lanes);
            }
            _ => (),
        }
    }

    Ok(builder.finish())
}

fn get_def_params<'a, 'b: 'a>(
    d: &'a GateDeclaration,
    symbols: &'b ExpandedSymbolTable,
) -> Vec<&'a str> {
    d.params()
        .unwrap_or_default()
        .iter()
        .map(|p| symbols.get_symbol_name(p.as_ref().unwrap()))
        .collect::<Vec<_>>()
}

fn get_def_qubits<'a, 'b: 'a>(
    d: &'a GateDeclaration,
    symbols: &'b ExpandedSymbolTable,
) -> Vec<&'a str> {
    d.qubits()
        .iter()
        .map(|q| symbols.get_symbol_name(q.as_ref().unwrap()))
        .collect::<Vec<_>>()
}

fn get_call_params_expr(g: &GateCall) -> Vec<&TExpr> {
    g.params().unwrap_or_default().iter().collect::<Vec<_>>()
}

fn get_call_params(g: &GateCall) -> Vec<f64> {
    g.params()
        .unwrap_or_default()
        .iter()
        .map(|p| {
            if let Expr::Literal(Literal::Float(f)) = p.expression() {
                f.value().parse().unwrap()
            } else {
                0.0
            }
        })
        .collect::<Vec<_>>()
}

fn get_call_lanes(g: &GateCall) -> Vec<usize> {
    g.qubits()
        .iter()
        .filter_map(|e| {
            let Expr::GateOperand(GateOperand::IndexedIdentifier(id)) = e.expression() else {
                return None;
            };
            let IndexOperator::ExpressionList(el) = id.indexes().first()? else {
                return None;
            };
            let Expr::Literal(Literal::Int(i)) = el.expressions[0].expression() else {
                return None;
            };
            Some(*i.value() as usize)
        })
        .collect::<Vec<_>>()
}

struct ExpandedSymbolTable<'a> {
    symbols: &'a SymbolTable,
}

impl<'a> ExpandedSymbolTable<'a> {
    fn new(symbols: &'a SymbolTable) -> Self {
        Self { symbols }
    }

    // fn get_symbol(&self, id: &SymbolId) -> &Symbol {
    //     &self.symbols[id]
    // }

    fn get_symbol_name(&self, id: &SymbolId) -> &str {
        self.symbols[id].name()
    }

    // fn get_symbol_type(&self, id: &SymbolId) -> &Type {
    //     self.symbols[id].symbol_type()
    // }

    fn get_n_qubits(&self, stm: &DeclareQuantum) -> usize {
        let sym = stm.name().as_ref().unwrap();
        if let Type::QubitArray(ArrayDims::D1(d)) = self.symbols[sym].symbol_type() {
            *d
        } else {
            0
        }
    }
}

type GateCombinationFn<'a> = Box<dyn Fn(&CircuitBuilder, &[f64], &[usize]) + 'a>;

struct CircuitBuilder<'a> {
    n_qubits: Option<usize>,
    circuit: Option<RefCell<QuantumCircuit>>,
    gates_definitions: HashMap<String, GateCombinationFn<'a>>,
}

impl<'a> CircuitBuilder<'a> {
    fn new() -> Self {
        Self {
            n_qubits: None,
            circuit: None,
            gates_definitions: HashMap::new(),
        }
    }

    fn set_n_qubits(&mut self, n_qubits: usize) {
        self.n_qubits = Some(n_qubits);
        self.circuit = Some(RefCell::new(QuantumCircuit::new(n_qubits)));
    }

    fn add_gate(&self, gate_name: &str, params: &[f64], lanes: &[usize]) {
        macro_rules! circ {
            () => {
                self.circuit.as_ref().unwrap().borrow_mut()
            };
        }

        match gate_name {
            "id" => circ!().g_id(lanes[0]),
            "U" => circ!().g_u(params[0], params[1], params[2], lanes[0]),
            "p" | "phase" => circ!().g_p(params[0], lanes[0]),
            "x" => circ!().g_x(lanes[0]),
            "y" => circ!().g_y(lanes[0]),
            "z" => circ!().g_z(lanes[0]),
            "h" => circ!().g_h(lanes[0]),
            "s" => circ!().g_s(lanes[0]),
            "sdg" => circ!().g_s_dg(lanes[0]),
            "t" => circ!().g_t(lanes[0]),
            "tdg" => circ!().g_t_dg(lanes[0]),
            "sx" => circ!().g_sx(lanes[0]),
            "rx" => circ!().g_rx(params[0], lanes[0]),
            "ry" => circ!().g_ry(params[0], lanes[0]),
            "rz" => circ!().g_rz(params[0], lanes[0]),
            "CX" | "cx" => circ!().g_cx(lanes[0], lanes[1]),
            "cy" => circ!().g_cy(lanes[0], lanes[1]),
            "cz" => circ!().g_cz(lanes[0], lanes[1]),
            "cp" | "cphase" => circ!().g_cp(params[0], lanes[0], lanes[1]),
            "crx" => circ!().g_crx(params[0], lanes[0], lanes[1]),
            "cry" => circ!().g_cry(params[0], lanes[0], lanes[1]),
            "crz" => circ!().g_crz(params[0], lanes[0], lanes[1]),
            "ch" => circ!().g_ch(lanes[0], lanes[1]),
            "swap" => circ!().g_swap(lanes[0], lanes[1]),
            "ccx" => circ!().g_cxx(lanes[0], lanes[1], lanes[2]),
            "cswap" => circ!().g_cswap(lanes[0], lanes[1], lanes[2]),
            "cu" => circ!().g_cu(
                params[0], params[1], params[2], params[3], lanes[0], lanes[1],
            ),
            "u1" => circ!().g_u1(params[0], lanes[0]),
            "u2" => circ!().g_u2(params[0], params[1], lanes[0]),
            "u3" => circ!().g_u3(params[0], params[1], params[2], lanes[0]),
            s if self.gates_definitions.contains_key(s) => {
                let fun = self.gates_definitions.get(s).unwrap();
                fun(self, params, lanes);
            }
            _ => (),
        }
    }

    fn add_gate_definition<'b>(&'b mut self, gate_name: String, fun: GateCombinationFn<'a>)
    where
        'a: 'b,
    {
        self.gates_definitions.insert(gate_name, fun);
    }

    fn finish(self) -> QuantumCircuit {
        self.circuit.unwrap().into_inner()
    }
}
