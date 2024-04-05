use petgraph::{Directed, Graph};

use crate::{
    model::{Block, GateKind},
    tree::BTree,
};

struct ContractionGraph {
    graph: Graph<GateKind, u8, Directed>,
}

struct ContractionTree {
    tree: BTree<Block>,
}
