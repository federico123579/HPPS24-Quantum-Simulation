use petgraph::{Directed, Graph};

use crate::{
    model::{Block, GateOnLanes, QuantumCircuit},
    tree::BTree,
};

#[derive(Debug, Clone)]
pub struct ContractionGraph {
    pub graph: Graph<GateOnLanes, u8, Directed>,
}

impl From<QuantumCircuit> for ContractionGraph {
    fn from(value: QuantumCircuit) -> Self {
        let QuantumCircuit { n_qubits, gates } = value;
        let mut graph = Graph::new();
        let mut nodes = vec![None; n_qubits];

        for gl in gates {
            let lanes = gl.lanes();
            let rank = gl.rank();
            let node = graph.add_node(gl);
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

impl std::fmt::Display for ContractionGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", petgraph::dot::Dot::new(&self.graph))
    }
}

struct ContractionTree {
    tree: BTree<Block>,
}
