use crate::{
    contractions::{TensorContraction, TensorKind},
    model::{
        gates::{Gate, QuantumGate},
        span::Span,
    },
};

#[derive(Debug, Clone)]
pub struct Operation {
    pub inner: OperationKind,
    /// flag indicating if it's the right operand of a matrix multiplication,
    /// in that case the entire operation is transposed
    pub transposed_op: bool,
}

#[derive(Debug, Clone)]
pub enum OperationKind {
    TensorExpansion { target_span: Span, operand: Operand },
    MatrixMultiplication { left: Operand, right: Operand },
}

#[derive(Debug, Clone)]
pub enum Operand {
    Operation(Box<Operation>),
    Gate(Box<Gate>),
}

impl Operand {
    pub fn span(&self) -> Span {
        match self {
            Self::Operation(op) => op.span(),
            Self::Gate(gate) => gate.span(),
        }
    }

    pub fn from_tensor_kind(kind: TensorKind, transposed_op: bool) -> Self {
        match kind {
            TensorKind::Contraction(contr) => {
                Operand::Operation(Box::new(Operation::from_contraction(*contr, transposed_op)))
            }
            TensorKind::Gate(gate) => Operand::Gate(gate),
        }
    }
}

impl OperationKind {
    pub fn span(&self) -> Span {
        match self {
            Self::TensorExpansion { target_span, .. } => target_span.clone(),
            Self::MatrixMultiplication { left, right } => left.span().union(&right.span()),
        }
    }
}

impl Operation {
    pub fn span(&self) -> Span {
        self.inner.span()
    }

    pub fn from_contraction(contr: TensorContraction, transposed_op: bool) -> Self {
        let TensorContraction {
            lhs: left,
            rhs: right,
            span,
            ..
        } = contr;

        let left = if span.filled() != left.span().filled() {
            Operand::Operation(Box::new(Operation {
                inner: OperationKind::TensorExpansion {
                    target_span: span.clone(),
                    operand: Operand::from_tensor_kind(left, transposed_op),
                },
                transposed_op,
            }))
        } else {
            Operand::from_tensor_kind(left, transposed_op)
        };

        let right = if span.filled() != right.span().filled() {
            Operand::Operation(Box::new(Operation {
                inner: OperationKind::TensorExpansion {
                    target_span: span,
                    operand: Operand::from_tensor_kind(right, !transposed_op),
                },
                transposed_op: !transposed_op,
            }))
        } else {
            Operand::from_tensor_kind(right, !transposed_op)
        };

        Self {
            inner: OperationKind::MatrixMultiplication { left, right },
            transposed_op,
        }
    }
}
