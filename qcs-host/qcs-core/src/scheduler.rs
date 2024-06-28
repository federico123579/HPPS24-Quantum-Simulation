pub mod contraction;
pub mod operation;

pub use contraction::ContractionPlan;
pub use operation::OperationPlan;

use crate::model::blocks::BlockLike;

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionOperand<B: BlockLike> {
    Block(B),
    Address(usize),
}

impl<B: BlockLike> From<B> for ExecutionOperand<B> {
    fn from(block: B) -> Self {
        Self::Block(block)
    }
}

impl<B: BlockLike> From<usize> for ExecutionOperand<B> {
    fn from(id: usize) -> Self {
        Self::Address(id)
    }
}

impl<B: BlockLike> std::fmt::Display for ExecutionOperand<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Block(block) => write!(
                f,
                "M({:02}x{:02})",
                block.as_ref().nrows(),
                block.as_ref().ncols()
            ),
            Self::Address(id) => write!(f, "I({:05})", id),
        }
    }
}
