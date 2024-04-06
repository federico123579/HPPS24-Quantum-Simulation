use either::Either;
use hashbrown::HashSet;
use petgraph::{graph::EdgeReference, visit::EdgeRef, Directed, Direction, Graph};

use crate::model::{GateOnLanes, QuantumCircuit};

#[derive(Debug, Clone)]
pub struct TensorNetwork {
    graph: Graph<ContractionItem, u8, Directed>,
}

impl TensorNetwork {
    fn find_edges(&self, rank: u8) -> Vec<EdgeReference<u8>> {
        self.graph
            .edge_references()
            .filter(|e| *e.weight() == rank)
            .collect::<Vec<_>>()
    }

    fn find_disjoint_edges(&self, rank: u8) -> Vec<EdgeReference<u8>> {
        let mut visited = HashSet::new();
        self.find_edges(rank)
            .into_iter()
            .filter(|e| visited.insert(e.source()) && visited.insert(e.target()))
            .collect()
    }

    fn contract_edge(&mut self, edge: &EdgeReference<u8>) {
        let ew = *edge.weight();
        let source = edge.source();
        let target = edge.target();

        // Get the nodes linked to the source and target nodes
        let source_linked_nodes = self
            .graph
            .edges_directed(source, Direction::Incoming)
            .map(|e| (e.source(), *e.weight()))
            .collect::<Vec<_>>();
        let target_linked_nodes = self
            .graph
            .edges_directed(target, Direction::Outgoing)
            .map(|e| (e.target(), *e.weight()))
            .collect::<Vec<_>>();

        // Remove the source and target nodes
        let source_contr = self.graph.remove_node(source).unwrap();
        let target_contr = self.graph.remove_node(target).unwrap();

        // Create a new node with the two removed nodes as children
        let new_contr = Contraction::new(source_contr, target_contr);
        let new_node = self.graph.add_node(new_contr.into());

        // Link the new node to the previously linked nodes
        for (node, w) in source_linked_nodes {
            self.graph.add_edge(new_node, node, w.max(ew));
        }
        for (node, w) in target_linked_nodes {
            self.graph.add_edge(node, new_node, w.max(ew));
        }
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
struct ContractionItem(Either<Box<Contraction>, Box<GateOnLanes>>);

impl ContractionItem {
    fn rank(&self) -> u8 {
        match &self.0 {
            Either::Left(c) => c.rank,
            Either::Right(g) => g.rank(),
        }
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
struct Contraction {
    rank: u8,
    left: ContractionItem,
    right: ContractionItem,
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
