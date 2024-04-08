use hashbrown::HashSet;
use petgraph::{
    graph::EdgeIndex,
    stable_graph::{EdgeReference, StableGraph},
    visit::{EdgeRef, IntoEdgeReferences},
    Directed, Direction,
};

use crate::model::{
    gates::{CircuitGate, GateSpan},
    QuantumCircuit,
};

#[derive(Debug, Clone)]
pub struct TensorNetwork {
    graph: StableGraph<ContractionItem, GateSpan, Directed>,
}

impl TensorNetwork {
    fn find_edges(&self, rank: u8) -> Vec<EdgeReference<GateSpan>> {
        self.graph
            .edge_references()
            .filter(|e| e.weight().span_len() as u8 <= rank)
            .collect::<Vec<_>>()
    }

    fn find_disjoint_edges(&self, rank: u8) -> Vec<EdgeReference<GateSpan>> {
        let mut visited = HashSet::new();
        self.find_edges(rank)
            .into_iter()
            .filter(|e| visited.insert(e.source()) && visited.insert(e.target()))
            .collect()
    }

    fn contract_edge(&mut self, edge: EdgeIndex) {
        let (source, target) = self.graph.edge_endpoints(edge).unwrap();

        let source_contr = self.graph.node_weight(source).unwrap().clone();
        let target_contr = self.graph.node_weight(target).unwrap().clone();

        // Create a new node with the two removed nodes as children
        let new_contr = Contraction::new(source_contr, target_contr);
        let contr_item = ContractionItem::from(new_contr);

        // Get the nodes linked to the source and target nodes
        let backlinks = self
            .graph
            .edges_directed(source, Direction::Incoming)
            .chain(
                self.graph
                    .edges_directed(target, Direction::Incoming)
                    .filter(|e| e.source() != source),
            )
            .map(|e| {
                (
                    e.source(),
                    self.graph.node_weight(e.source()).unwrap().span(),
                )
            })
            .map(|(n, w)| (n, w.merge(contr_item.span())))
            .collect::<Vec<_>>();
        let frontlinks = self
            .graph
            .edges_directed(target, Direction::Outgoing)
            .chain(
                self.graph
                    .edges_directed(source, Direction::Outgoing)
                    .filter(|e| e.target() != target),
            )
            .map(|e| {
                (
                    e.target(),
                    self.graph.node_weight(e.target()).unwrap().span(),
                )
            })
            .map(|(n, w)| (n, w.merge(contr_item.span())))
            .collect::<Vec<_>>();

        // Link the new node to the previously linked nodes
        let new_node = self.graph.add_node(contr_item);
        for (node, w) in backlinks {
            self.graph.add_edge(node, new_node, w);
        }
        for (node, w) in frontlinks {
            self.graph.add_edge(new_node, node, w);
        }

        // Remove the source and target nodes
        self.graph.remove_edge(edge);
        self.graph.remove_node(source).unwrap();
        self.graph.remove_node(target).unwrap();
    }

    pub fn contract(mut self) -> Vec<ContractionItem> {
        let mut curr_rank = 1;
        while self.graph.edge_count() > 0 {
            let edges = self
                .find_disjoint_edges(curr_rank)
                .into_iter()
                .map(|e| e.id())
                .collect::<Vec<_>>();
            // If there are no edges to contract, move to the next rank
            if edges.is_empty() {
                curr_rank += 1;
                continue;
            }
            println!("{}", self);
            // Contract all the edges of the current rank
            for edge in edges {
                self.contract_edge(edge);
            }
        }

        self.graph.node_weights().cloned().collect()
    }
}

impl From<QuantumCircuit> for TensorNetwork {
    fn from(value: QuantumCircuit) -> Self {
        let QuantumCircuit { n_qubits, gates } = value;
        let mut graph = StableGraph::new();
        let mut nodes = vec![None; n_qubits];

        for gl in gates {
            let span = gl.span().to_owned();
            let node = graph.add_node(gl.into());
            for i in span.clone().into_range() {
                let old = nodes[i].replace((node, span.clone()));
                if let Some((old_ix, old_r)) = old {
                    graph.add_edge(old_ix, node, span.merge(&old_r));
                }
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
pub enum ContractionItem {
    Contraction(Box<Contraction>),
    Gate(Box<CircuitGate>),
}

impl ContractionItem {
    fn rank(&self) -> u8 {
        match &self {
            Self::Contraction(c) => c.rank,
            Self::Gate(g) => g.rank(),
        }
    }

    fn span(&self) -> &GateSpan {
        match &self {
            Self::Contraction(c) => &c.span,
            Self::Gate(g) => &g.span,
        }
    }
}

impl From<Contraction> for ContractionItem {
    fn from(value: Contraction) -> Self {
        Self::Contraction(Box::new(value))
    }
}

impl From<CircuitGate> for ContractionItem {
    fn from(value: CircuitGate) -> Self {
        Self::Gate(Box::new(value))
    }
}

impl std::fmt::Display for ContractionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Contraction(c) => write!(f, "{}", c),
            Self::Gate(g) => write!(f, "{}", g),
        }
    }
}

// impl LaneAwareRank for ContractionItem {
//     fn rank(&self) -> u8 {
//         self.rank()
//     }

//     fn joined_rank(&self, other: &Self) -> u8 {
//         match (&self.0, &other.0) {
//             (Either::Left(l), Either::Left(r)) => l.joined_rank(r),
//             (Either::Left(l), Either::Right(r)) => {
//                 (l.span.end.max(r.span().end) - l.span.start.min(r.span().start)) as u8
//             }
//             (Either::Right(l), Either::Left(r)) => {
//                 (l.span().end.max(r.span.end) - l.span().start.min(r.span.start)) as u8
//             }
//             (Either::Right(l), Either::Right(r)) => l.joined_rank(r),
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct Contraction {
    pub rank: u8,
    pub span: GateSpan,
    pub left: ContractionItem,
    pub right: ContractionItem,
}

impl Contraction {
    fn new(left: ContractionItem, right: ContractionItem) -> Self {
        let span = left.span().merge(right.span());
        let rank = span.span_len() as u8;
        Self {
            rank,
            span,
            left,
            right,
        }
    }
}

impl std::fmt::Display for Contraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} ~ {})", self.left, self.right)
    }
}

// impl LaneAwareRank for Contraction {
//     fn rank(&self) -> u8 {
//         self.rank
//     }

//     fn joined_rank(&self, other: &Self) -> u8 {
//         (self.span.end.max(other.span.end) - self.span.start.min(other.span.start)) as u8
//     }
// }
