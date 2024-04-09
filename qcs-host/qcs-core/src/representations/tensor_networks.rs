use petgraph::{
    stable_graph::{EdgeIndex, EdgeReference, StableDiGraph},
    visit::{EdgeRef, IntoEdgeReferences},
    Direction,
};

use crate::{
    model::{gates::CircuitGate, QuantumCircuit},
    utils::{GateSpan, MultipleSpan, SpanRegister},
};

// use super::contraction_graph::ContractionGraph;

#[derive(Debug, Clone)]
pub struct TensorNetwork {
    graph: StableDiGraph<TensorKind, MultipleSpan>,
}

impl TensorNetwork {
    pub fn contractable(&self) -> Vec<EdgeReference<MultipleSpan>> {
        self.graph
            .edge_references()
            .filter(|e| {
                let source = self.graph.node_weight(e.source()).unwrap();
                let target = self.graph.node_weight(e.target()).unwrap();
                let max_span = source.span().inner_join(target.span()).unwrap();
                e.weight() == &max_span.into()
            })
            .collect()
    }

    pub fn contract(&mut self, edge: EdgeIndex) {
        let (source, target) = self.graph.edge_endpoints(edge).unwrap();

        let backlinks = self
            .graph
            .edges_directed(source, Direction::Incoming)
            .chain(self.graph.edges_directed(target, Direction::Incoming))
            .filter(|e| e.source() != target && e.source() != source)
            .map(|e| (e.source(), e.weight().clone()))
            .collect::<Vec<_>>();
        let frontlinks = self
            .graph
            .edges_directed(target, Direction::Outgoing)
            .chain(self.graph.edges_directed(source, Direction::Outgoing))
            .filter(|e| e.target() != target && e.target() != source)
            .map(|e| (e.target(), e.weight().clone()))
            .collect::<Vec<_>>();

        let source_contr = self.graph.remove_node(source).unwrap();
        let target_contr = self.graph.remove_node(target).unwrap();
        todo!()
    }

    // pub fn contraction_rank(&self, edge: &TensorEdge) -> u8 {
    //     let left = self.nodes[edge.left].as_ref().unwrap();
    //     let right = self.nodes[edge.right].as_ref().unwrap();
    //     left.span().full_join(right.span()).span_len() as u8
    // }

    // // fn edge_endpoints(&self, edge: &TensorEdge) -> (&TensorKind, &TensorKind) {
    // //     (
    // //         self.nodes[edge.left].as_ref().unwrap(),
    // //         self.nodes[edge.right].as_ref().unwrap(),
    // //     )
    // // }

    // fn push_node(&mut self, node: TensorKind) -> usize {
    //     for (ix, n) in self.nodes.iter_mut().enumerate() {
    //         if n.is_none() {
    //             *n = Some(node);
    //             return ix;
    //         }
    //     }
    //     self.nodes.push(Some(node));
    //     self.nodes.len() - 1
    // }
}

impl From<QuantumCircuit> for TensorNetwork {
    fn from(circuit: QuantumCircuit) -> Self {
        let QuantumCircuit { gates, .. } = circuit;
        let mut graph = StableDiGraph::new();
        // This will be used as a vertical slice of the last gate in each qubit lane
        let mut span_register = SpanRegister::new();
        for gate in gates.into_iter() {
            let tensor = TensorKind::Gate(Box::new(gate));
            let current_span = tensor.span().clone();
            let new_node = graph.add_node(tensor);
            let linked_spans = span_register.get(&current_span.clone().into());
            span_register.apply(current_span.into(), new_node);
            for (span, node) in linked_spans {
                graph.add_edge(new_node, node, span);
            }
        }
        Self { graph }
    }
}

impl std::fmt::Display for TensorNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", petgraph::dot::Dot::new(&self.graph))
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
