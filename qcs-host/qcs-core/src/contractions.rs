//! This module contains the logic for contracting a tensor network.
//! The tensor network is represented as a directed graph, where the nodes are tensors
//! and the edges are tensor connections. The tensor connections are represented as spans.
//!
//! The contraction is done by contracting the tensors with the smallest rank first.
//! The rank of a contraction is the length of the span that the contraction would cover.
//!
//! Not all arcs can be contracted, only the one that satisfy the following condition:
//! The span of the source tensor and the target tensor is equal to the span of the arc.

use hashbrown::{HashMap, HashSet};
use petgraph::{
    stable_graph::{EdgeIndex, EdgeReference, StableDiGraph},
    visit::{EdgeRef, IntoEdgeReferences},
    Direction,
};

use crate::model::{
    gates::{Gate, QuantumGate},
    span::{Span, SpanRegister},
    QuantumCircuit,
};

/// A tensor network is a directed graph where the nodes are tensors and the
/// edges are tensor connections.
#[derive(Debug, Clone)]
pub struct TensorNetwork {
    graph: StableDiGraph<TensorKind, Span>,
}

impl TensorNetwork {
    /// Contract the tensor network and return the contracted tensors.
    pub fn contract(mut self) -> Vec<TensorKind> {
        println!("{}", self);
        loop {
            let lowest_rank = self
                .contractable()
                .into_iter()
                .map(|e| self.contraction_rank(&e))
                .min();
            let Some(lowest_rank) = lowest_rank else {
                break;
            };

            let mut visited = HashSet::new();
            let to_contract = self
                .contractable()
                .into_iter()
                .filter(|e| self.contraction_rank(e) == lowest_rank)
                .filter(|e| {
                    if !visited.contains(&e.source()) && !visited.contains(&e.target()) {
                        visited.insert(e.source());
                        visited.insert(e.target());
                        true
                    } else {
                        false
                    }
                })
                .map(|e| e.id())
                .collect::<Vec<_>>();

            to_contract.into_iter().for_each(|to_contract| {
                self.contract_edge(to_contract);
            });
            println!("{}", self);
        }

        self.graph.node_weights().cloned().collect()
    }

    /// Find the edges that can be contracted.
    fn contractable(&self) -> Vec<EdgeReference<Span>> {
        self.graph
            .edge_references()
            .filter(|e| {
                let source = self.graph.node_weight(e.source()).unwrap();
                let target = self.graph.node_weight(e.target()).unwrap();
                let max_span = source.span().inner_join(&target.span()).unwrap();
                e.weight() == &max_span
            })
            .collect()
    }

    /// Contract an edge in the tensor network.
    fn contract_edge(&mut self, edge: EdgeIndex) {
        let (source, target) = self.graph.edge_endpoints(edge).unwrap();

        let mut backlinks = HashMap::new();
        self.graph
            .edges_directed(source, Direction::Incoming)
            .chain(self.graph.edges_directed(target, Direction::Incoming))
            .filter(|e| e.source() != target && e.source() != source)
            .map(|e| (e.source(), e.weight().clone()))
            .for_each(|(n, s)| {
                let span = backlinks
                    .remove(&n)
                    .map(|ms: Span| ms.full_join(&s))
                    .unwrap_or_else(|| s.clone());
                backlinks.insert(n, span);
            });
        let mut frontlinks = HashMap::new();
        self.graph
            .edges_directed(target, Direction::Outgoing)
            .chain(self.graph.edges_directed(source, Direction::Outgoing))
            .filter(|e| e.target() != target && e.target() != source)
            .map(|e| (e.target(), e.weight().clone()))
            .for_each(|(n, s)| {
                let span = frontlinks
                    .remove(&n)
                    .map(|ms: Span| ms.full_join(&s))
                    .unwrap_or_else(|| s.clone());
                frontlinks.insert(n, span);
            });

        let source_contr = self.graph.remove_node(source).unwrap();
        let target_contr = self.graph.remove_node(target).unwrap();
        let new_contr = TensorContraction::new(source_contr, target_contr);
        let new_node = self.graph.add_node(new_contr.into());

        for (node, span) in backlinks {
            self.graph.add_edge(node, new_node, span);
        }
        for (node, span) in frontlinks {
            self.graph.add_edge(new_node, node, span);
        }
    }

    /// Calculate the rank of a contraction.
    pub fn contraction_rank(&self, edge: &EdgeReference<Span>) -> u8 {
        let source = self.graph.node_weight(edge.source()).unwrap();
        let target = self.graph.node_weight(edge.target()).unwrap();
        source.span().full_join(&target.span()).span_len() as u8
    }
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
            let linked_spans = span_register.get(&current_span.clone());
            span_register.apply(current_span, new_node);
            for (span, node) in linked_spans {
                graph.add_edge(node, new_node, span);
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

/// The kind of tensor in the tensor network.
/// It can be a contraction or a gate.
#[derive(Debug, Clone)]
pub enum TensorKind {
    /// A tensor contraction.
    Contraction(Box<TensorContraction>),
    /// A single quantum gate.
    Gate(Box<Gate>),
}

impl TensorKind {
    /// Get the span of the tensor.
    pub fn span(&self) -> Span {
        match self {
            Self::Contraction(c) => c.span.clone(),
            Self::Gate(g) => g.span(),
        }
    }
}

impl From<TensorContraction> for TensorKind {
    fn from(value: TensorContraction) -> Self {
        Self::Contraction(Box::new(value))
    }
}

impl From<Gate> for TensorKind {
    fn from(value: Gate) -> Self {
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

/// A tensor contraction is a contraction between two tensors.
/// The rank of the contraction is the length of the span that the contraction would cover.
#[derive(Debug, Clone)]
pub struct TensorContraction {
    pub rank: u8,
    pub span: Span,
    /// The left tensor in the contraction.
    pub lhs: TensorKind,
    /// The right tensor in the contraction.
    pub rhs: TensorKind,
}

impl TensorContraction {
    /// Create a new tensor contraction.
    pub fn new(left: TensorKind, right: TensorKind) -> Self {
        let span = left.span().full_join(&right.span());
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
