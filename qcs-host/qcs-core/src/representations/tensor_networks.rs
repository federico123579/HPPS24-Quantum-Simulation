use crate::{
    model::{gates::CircuitGate, QuantumCircuit},
    utils::{GateSpan, SpanRegister},
};

use super::contraction_graph::ContractionGraph;

#[derive(Debug, Clone)]
pub struct TensorNetwork {
    nodes: Vec<Option<TensorKind>>,
    pub edges: Vec<TensorEdge>,
    dim: usize,
}

impl TensorNetwork {
    pub fn contractable(&self) -> Vec<&TensorEdge> {
        self.edges
            .iter()
            .filter(|edge| {
                let left = self.nodes[edge.left].as_ref().unwrap();
                let right = self.nodes[edge.right].as_ref().unwrap();
                left.span().inner_join(right.span()).unwrap() == edge.span
            })
            .collect()
    }

    pub fn contract(&mut self, edge: TensorEdge) {
        // update nodes
        let left = self.nodes[edge.left].take().unwrap();
        let right = self.nodes[edge.right].take().unwrap();
        let new_node = TensorKind::from(TensorContraction::new(left, right));
        let new_ix = self.push_node(new_node);

        // Update Edges
        for e in self.edges.iter_mut() {
            if e.right == edge.left || e.right == edge.right {
                e.right = new_ix;
                e.
            } else if e.left == edge.left || e.left == edge.right {
                e.left = new_ix;
            }
        }
    }

    pub fn contraction_rank(&self, edge: &TensorEdge) -> u8 {
        let left = self.nodes[edge.left].as_ref().unwrap();
        let right = self.nodes[edge.right].as_ref().unwrap();
        left.span().full_join(right.span()).span_len() as u8
    }

    // fn edge_endpoints(&self, edge: &TensorEdge) -> (&TensorKind, &TensorKind) {
    //     (
    //         self.nodes[edge.left].as_ref().unwrap(),
    //         self.nodes[edge.right].as_ref().unwrap(),
    //     )
    // }

    fn push_node(&mut self, node: TensorKind) -> usize {
        for (ix, n) in self.nodes.iter_mut().enumerate() {
            if n.is_none() {
                *n = Some(node);
                return ix;
            }
        }
        self.nodes.push(Some(node));
        self.nodes.len() - 1
    }
}

impl From<QuantumCircuit> for TensorNetwork {
    fn from(circuit: QuantumCircuit) -> Self {
        let QuantumCircuit { n_qubits, gates } = circuit;

        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        // This will be used as a vertical slice of the last gate in each qubit lane
        let mut span_register = SpanRegister::new(n_qubits);

        for (ix, gate) in gates.into_iter().enumerate() {
            let tensor = TensorKind::Gate(Box::new(gate));
            edges.extend(
                span_register
                    .get(tensor.span())
                    .into_iter()
                    .map(|(span, i)| TensorEdge::new(i, ix, span)),
            );
            span_register.apply(tensor.span().clone(), ix);
            nodes.push(Some(tensor));
        }

        Self {
            nodes,
            edges,
            dim: n_qubits,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TensorEdge {
    left: usize,
    right: usize,
    span: GateSpan,
}

impl TensorEdge {
    fn new(left: usize, right: usize, span: GateSpan) -> Self {
        Self { left, right, span }
    }

    fn is_linked_to(&self, node: usize) -> bool {
        self.left == node || self.right == node
    }

    fn is_linked_to_any(&self, nodes: &[usize]) -> bool {
        nodes.contains(&self.left) || nodes.contains(&self.right)
    }
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
        let span = left.span().full_join(right.span());
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
