use std::ops::Deref;

use either::Either;
use hashbrown::HashSet;
use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    visit::EdgeRef,
    Directed, Direction, Graph,
};

use crate::model::{GateOnLanes, QuantumCircuit};

#[derive(Debug, Clone)]
pub struct TensorNetwork {
    graph: Graph<ContractionItem, u8, Directed>,
}

impl TensorNetwork {
    #[inline]
    fn edge_w(&self, edge: EdgeIndex) -> u8 {
        *self.graph.edge_weight(edge).unwrap()
    }

    #[inline]
    fn edge_source(&self, edge: EdgeIndex) -> NodeIndex {
        self.graph.edge_endpoints(edge).unwrap().0
    }

    #[inline]
    fn edge_target(&self, edge: EdgeIndex) -> NodeIndex {
        self.graph.edge_endpoints(edge).unwrap().1
    }

    fn find_edges(&self, rank: u8) -> Vec<EdgeIndex> {
        self.graph
            .edge_indices()
            .filter(|e| self.edge_w(*e) == rank)
            .collect::<Vec<_>>()
    }

    fn find_disjoint_edges(&self, rank: u8) -> Vec<EdgeIndex> {
        let mut visited = HashSet::new();
        self.find_edges(rank)
            .into_iter()
            .filter(|e| {
                visited.insert(self.edge_source(*e)) && visited.insert(self.edge_target(*e))
            })
            .collect()
    }

    fn contract_edge(&mut self, edge: EdgeIndex) {
        let ew = self.edge_w(edge);
        let source = self.edge_source(edge);
        let target = self.edge_target(edge);

        // Get the nodes linked to the source and target nodes
        let backlinks = self
            .graph
            .edges_directed(source, Direction::Incoming)
            .chain(
                self.graph
                    .edges_directed(target, Direction::Incoming)
                    .filter(|e| e.source() != source),
            )
            .map(|e| (e.source(), *e.weight()))
            .collect::<Vec<_>>();
        let frontlinks = self
            .graph
            .edges_directed(target, Direction::Outgoing)
            .chain(
                self.graph
                    .edges_directed(source, Direction::Outgoing)
                    .filter(|e| e.target() != target),
            )
            .map(|e| (e.target(), *e.weight()))
            .collect::<Vec<_>>();

        let source_contr = self.graph.node_weight(source).unwrap().clone();
        let target_contr = self.graph.node_weight(target).unwrap().clone();

        // Create a new node with the two removed nodes as children
        let new_contr = Contraction::new(source_contr, target_contr);
        let new_node = self.graph.add_node(new_contr.into());

        // Link the new node to the previously linked nodes
        for (node, w) in backlinks {
            self.graph.add_edge(node, new_node, w.max(ew));
        }
        for (node, w) in frontlinks {
            self.graph.add_edge(new_node, node, w.max(ew));
        }

        // Remove the source and target nodes
        self.graph.remove_edge(edge);
        self.graph.remove_node(source).unwrap();
        self.graph.remove_node(target).unwrap();
    }

    pub fn contract(mut self) -> Vec<ContractionItem> {
        let mut curr_rank = 1;
        while self.graph.edge_count() > 0 {
            let edges = self.find_disjoint_edges(curr_rank);
            // If there are no edges to contract, move to the next rank
            if edges.is_empty() {
                curr_rank += 1;
                continue;
            }
            // println!("{}", self);
            // Contract all the edges of the current rank
            for edge in edges {
                self.contract_edge(edge);
            }
        }

        self.graph
            .node_indices()
            .map(|n| self.graph.remove_node(n).unwrap())
            .collect()
    }
}

impl From<QuantumCircuit> for TensorNetwork {
    fn from(value: QuantumCircuit) -> Self {
        let QuantumCircuit { n_qubits, gates } = value;
        let mut graph = Graph::new();
        let mut nodes = vec![None; n_qubits];

        for gl in gates {
            let lanes = gl.lanes();
            let rank = gl.rank();
            let node = graph.add_node(gl.into());
            for i in lanes {
                let old = nodes[i].replace((node, rank));
                if let Some((old_ix, old_r)) = old {
                    graph.add_edge(old_ix, node, rank.max(old_r));
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
pub struct ContractionItem(pub Either<Box<Contraction>, Box<GateOnLanes>>);

impl ContractionItem {
    fn rank(&self) -> u8 {
        match &self.0 {
            Either::Left(c) => c.rank,
            Either::Right(g) => g.rank(),
        }
    }
}

impl Deref for ContractionItem {
    type Target = Either<Box<Contraction>, Box<GateOnLanes>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Contraction> for ContractionItem {
    fn from(value: Contraction) -> Self {
        Self(Either::Left(Box::new(value)))
    }
}

impl From<GateOnLanes> for ContractionItem {
    fn from(value: GateOnLanes) -> Self {
        Self(Either::Right(Box::new(value)))
    }
}

impl std::fmt::Display for ContractionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Either::Left(c) => write!(f, "{}", c),
            Either::Right(g) => write!(f, "{}", g),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Contraction {
    pub rank: u8,
    pub left: ContractionItem,
    pub right: ContractionItem,
}

impl Contraction {
    fn new(left: ContractionItem, right: ContractionItem) -> Self {
        let rank = left.rank().max(right.rank());
        Self { rank, left, right }
    }
}

impl std::fmt::Display for Contraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} ~ {})", self.left, self.right)
    }
}
