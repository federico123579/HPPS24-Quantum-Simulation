use petgraph::{Directed, Graph};

use crate::{
    model::{Block, QuantumGate},
    tree::BTree,
};

struct ContractionGraph {
    graph: Graph<Box<dyn QuantumGate>, u8, Directed>,
}

struct ContractionTree {
    tree: BTree<Block>,
}
