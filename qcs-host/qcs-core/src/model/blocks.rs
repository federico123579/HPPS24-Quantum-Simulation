use std::ops::Mul;

use nalgebra::{Complex, DMatrix};

use crate::model::{gates::*, span::Span};

use super::{QRegister, TensorProduct};

#[derive(Debug, Clone, PartialEq)]
pub struct SpannedBlock {
    block: Block,
    span: Span,
}

impl SpannedBlock {
    pub fn new(block: Block, span: Span) -> Self {
        Self { block, span }
    }

    pub fn merged_span(&self, rhs: &SpannedBlock) -> Span {
        self.span.full_join(&rhs.span)
    }

    pub fn adapt_to_span(mut self, span: Span) -> Self {
        let mut new_block = Block::one();
        for i in span.min()..self.span.min() {
            new_block = new_block.tensor_product(Identity::new(i));
        }
        new_block = new_block.tensor_product(self.block);
        for i in (self.span.max() + 1)..(span.max() + 1) {
            new_block = new_block.tensor_product(Identity::new(i));
        }
        self.block = new_block;
        self.span = span;
        self
    }

    pub fn into_block(self) -> Block {
        self.block
    }
}

impl std::fmt::Display for SpannedBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SpannedBlock{}:\n{}", self.span, self.block)
    }
}

impl From<Gate> for SpannedBlock {
    fn from(gate: Gate) -> Self {
        gate.spanned_block()
    }
}

impl TensorProduct for SpannedBlock {
    type Output = SpannedBlock;

    fn tensor_product(&self, rhs: impl Into<SpannedBlock>) -> Self::Output {
        let rhs = rhs.into();
        SpannedBlock {
            block: self.block.tensor_product(rhs.block),
            span: self.span.full_join(&rhs.span),
        }
    }
}

impl Mul<&SpannedBlock> for &SpannedBlock {
    type Output = SpannedBlock;

    fn mul(self, rhs: &SpannedBlock) -> Self::Output {
        assert_eq!(self.span, rhs.span, "Incompatible spans");
        SpannedBlock {
            block: &self.block * &rhs.block,
            span: self.span.full_join(&rhs.span),
        }
    }
}

impl Mul<SpannedBlock> for SpannedBlock {
    type Output = SpannedBlock;

    fn mul(self, rhs: SpannedBlock) -> Self::Output {
        assert_eq!(self.span, rhs.span, "Incompatible spans");
        SpannedBlock {
            block: &self.block * &rhs.block,
            span: self.span.full_join(&rhs.span),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    matrix_repr: DMatrix<Complex<f64>>,
    dim: usize,
}

impl Block {
    pub fn empty(dim: usize) -> Self {
        DMatrix::from_element(dim, dim, Complex::new(0.0, 0.0)).into()
    }

    pub fn one() -> Self {
        DMatrix::from_row_slice(1, 1, &[1.0])
            .map(|x| Complex::new(x, 0.0))
            .into()
    }

    pub fn identity(dim: usize) -> Self {
        DMatrix::identity(dim, dim).into()
    }

    pub fn into_matrix(self) -> DMatrix<Complex<f64>> {
        self.matrix_repr
    }
}

impl TensorProduct for Block {
    type Output = Block;

    fn tensor_product(&self, rhs: impl Into<Block>) -> Self::Output {
        let b = rhs.into();
        Block {
            matrix_repr: self.as_ref().kronecker(b.as_ref()),
            dim: self.dim * b.dim,
        }
    }
}

impl From<DMatrix<Complex<f64>>> for Block {
    fn from(matrix_repr: DMatrix<Complex<f64>>) -> Self {
        let dim = matrix_repr.nrows();
        Self { matrix_repr, dim }
    }
}

impl<G: QuantumGate> From<G> for Block {
    fn from(gate: G) -> Self {
        gate.block()
    }
}

impl From<Block> for DMatrix<Complex<f64>> {
    fn from(block: Block) -> Self {
        block.matrix_repr
    }
}

impl AsRef<DMatrix<Complex<f64>>> for Block {
    fn as_ref(&self) -> &DMatrix<Complex<f64>> {
        &self.matrix_repr
    }
}

impl Mul<&Block> for &Block {
    type Output = Block;

    fn mul(self, rhs: &Block) -> Self::Output {
        Block {
            matrix_repr: self.as_ref() * rhs.as_ref(),
            dim: self.dim,
        }
    }
}

impl Mul<Block> for &Block {
    type Output = Block;

    fn mul(self, rhs: Block) -> Self::Output {
        Block {
            matrix_repr: self.as_ref() * rhs.as_ref(),
            dim: self.dim,
        }
    }
}

impl Mul<Block> for Block {
    type Output = Block;

    fn mul(self, rhs: Block) -> Self::Output {
        Block {
            matrix_repr: self.as_ref() * rhs.as_ref(),
            dim: self.dim,
        }
    }
}

impl Mul<&Block> for Block {
    type Output = Block;

    fn mul(self, rhs: &Block) -> Self::Output {
        Block {
            matrix_repr: self.as_ref() * rhs.as_ref(),
            dim: self.dim,
        }
    }
}

impl<Q: Into<QRegister>> Mul<Q> for &Block {
    type Output = QRegister;

    fn mul(self, rhs: Q) -> Self::Output {
        QRegister {
            qubits: self.as_ref() * rhs.into().qubits,
        }
    }
}

impl<Q: Into<QRegister>> Mul<Q> for Block {
    type Output = QRegister;

    fn mul(self, rhs: Q) -> Self::Output {
        QRegister {
            qubits: self.as_ref() * rhs.into().qubits,
        }
    }
}

impl std::ops::Add for Block {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Block {
            matrix_repr: self.as_ref() + rhs.as_ref(),
            dim: self.dim,
        }
    }
}

impl std::ops::Sub for Block {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Block {
            matrix_repr: self.as_ref() - rhs.as_ref(),
            dim: self.dim,
        }
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.matrix_repr)
    }
}
