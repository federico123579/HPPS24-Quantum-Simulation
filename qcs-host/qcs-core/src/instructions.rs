use crate::model::gates::Gate;

pub enum Instruction {
    /// A tensor contraction between two tensors
    Contraction(TensorContraction),
    /// A tensor product between two tensors
    Expansion(TensorExpansion),
}

/// An instruction that represents a tensor contraction between two tensors.
/// The tensor contraction is a matrix multiplication between the two blocks.
pub struct TensorContraction {
    /// The rank of the contraction, that must be the same as the rank of the operands.
    pub rank: u8,
    /// The left tensor in the contraction.
    pub left: Operand,
    /// The right tensor in the contraction.
    pub right: Operand,
}

/// An instruction that represents a tensor product between two tensors.
/// The tensor product is the kronecker product between the two blocks.
pub struct TensorExpansion {
    /// The rank of the expansion, which is the rank of the resulting tensor.
    pub rank: u8,
    /// The tensor up in the expansion.
    pub up: Operand,
    /// The tensor down in the expansion.
    pub down: Operand,
}

pub enum Operand {
    /// The result of another instruction
    Instruction(Box<Instruction>),
    /// A quantum gate
    Gate(Box<Gate>),
}
