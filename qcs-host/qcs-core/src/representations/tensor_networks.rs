use crate::model::{
    gates::{CircuitGate, GateSpan},
    QuantumCircuit,
};

use super::contraction_graph::ContractionGraph;

#[derive(Debug, Clone)]
pub struct TensorNetwork {
    nodes: Vec<TensorKind>,
    edges: Vec<TensorEdges>,
}

impl TensorNetwork {
    pub fn contractable(&self) -> Vec<&TensorKind> {
        // self.nodes
        //     .iter()
        //     .enumerate()
        //     .filter(|(ix, _)| self.edges[*ix][e
        todo!()
    }
}

impl From<QuantumCircuit> for TensorNetwork {
    fn from(circuit: QuantumCircuit) -> Self {
        let QuantumCircuit { n_qubits, gates } = circuit;

        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        // This will be used as a vertical slice of the last gate in each qubit lane
        let mut last_in_line = vec![None; n_qubits];

        for (ix, gate) in gates.into_iter().enumerate() {
            let tensor = TensorKind::Gate(Box::new(gate));
            tensor
                .span()
                .clone()
                .into_range()
                .for_each(|i| last_in_line[i] = Some(ix));
            nodes.push(tensor);
            let tensor_edges = last_in_line
                .iter()
                .map(|gate_ix| match gate_ix {
                    Some(i) => TensorEdge::internal(*i),
                    None => TensorEdge::external(),
                })
                .collect();
            edges.push(TensorEdges::new(tensor_edges));
        }

        Self { nodes, edges }
    }
}

#[derive(Debug, Clone)]
struct TensorEdges {
    left: Vec<TensorEdge>,
}

impl TensorEdges {
    fn new(edges: Vec<TensorEdge>) -> Self {
        Self { left: edges }
    }
}

#[derive(Debug, Clone)]
struct TensorEdge {
    node_index: Option<usize>,
    kind: EdgeKind,
}

impl TensorEdge {
    fn external() -> Self {
        Self {
            node_index: None,
            kind: EdgeKind::External,
        }
    }

    fn internal(node_index: usize) -> Self {
        Self {
            node_index: Some(node_index),
            kind: EdgeKind::Internal,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum EdgeKind {
    External,
    Internal,
}

#[derive(Debug, Clone)]
pub enum TensorKind {
    Contraction(Box<TensorContraction>),
    Gate(Box<CircuitGate>),
}

impl TensorKind {
    fn rank(&self) -> u8 {
        match &self {
            Self::Contraction(c) => c.rank,
            Self::Gate(g) => g.rank(),
        }
    }

    pub fn span(&self) -> &GateSpan {
        match &self {
            Self::Contraction(c) => &c.span,
            Self::Gate(g) => &g.span,
        }
    }
}

impl From<TensorContraction> for TensorKind {
    fn from(value: TensorContraction) -> Self {
        Self::Contraction(Box::new(value))
    }
}

impl From<CircuitGate> for TensorKind {
    fn from(value: CircuitGate) -> Self {
        Self::Gate(Box::new(value))
    }
}

impl std::fmt::Display for TensorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Contraction(c) => write!(f, "{}", c),
            Self::Gate(g) => write!(f, "{}", g),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TensorContraction {
    pub rank: u8,
    pub span: GateSpan,
    pub lhs: TensorKind,
    pub rhs: TensorKind,
}

impl TensorContraction {
    pub fn new(left: TensorKind, right: TensorKind) -> Self {
        let span = left.span().merge(right.span());
        let rank = span.span_len() as u8;
        Self {
            rank,
            span,
            lhs: left,
            rhs: right,
        }
    }
}

impl std::fmt::Display for TensorContraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} ~ {})", self.lhs, self.rhs)
    }
}
