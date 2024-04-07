use std::{any::Any, borrow::BorrowMut, collections::VecDeque, ops::Deref};

use either::Either;

use crate::{
    contractions::Contraction,
    model::{Block, GateOnLanes},
};

pub struct ContractionPlan {
    instructions: Vec<Instruction>,
}

impl From<Contraction> for ContractionPlan {
    fn from(contraction: Contraction) -> Self {
        let (instruction, collaterals) = Instruction::from_contraction(0, contraction, vec![]);
        let mut instructions = collaterals;
        instructions.push(instruction);
        Self { instructions }
    }
}

impl std::fmt::Display for ContractionPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for instr in &self.instructions {
            writeln!(f, "{}", instr)?;
        }
        Ok(())
    }
}

struct Instruction {
    id: usize,
    dependencies: Vec<usize>,
    rank: u8,
    first: IntructionOperand,
    second: IntructionOperand,
}

impl Instruction {
    /// Create a new instruction with the given id from a contraction, returns also collaterals
    fn from_contraction(
        id: usize,
        contr: Contraction,
        collaterals: Vec<Instruction>,
    ) -> (Self, Vec<Self>) {
        let mut collaterals = collaterals;
        let mut available_id = id + 1;
        let mut dependencies = Vec::new();

        let Contraction { left, right, rank } = contr;

        let first = if left.is_left() {
            let (left_instr, col) =
                Self::from_contraction(available_id, *left.0.unwrap_left(), collaterals);
            collaterals = col;
            let instr_id = left_instr.id;
            dependencies.push(instr_id);
            collaterals.push(left_instr);
            available_id += collaterals.len() + 1;
            IntructionOperand::from(instr_id)
        } else {
            IntructionOperand::from(*left.0.unwrap_right())
        };

        let second = if right.is_left() {
            let (right_instr, col) =
                Self::from_contraction(available_id, *right.0.unwrap_left(), collaterals);
            collaterals = col;
            let instr_id = right_instr.id;
            dependencies.push(instr_id);
            collaterals.push(right_instr);
            IntructionOperand::from(instr_id)
        } else {
            IntructionOperand::from(*right.0.unwrap_right())
        };

        let instruction = Self {
            id,
            dependencies,
            rank,
            first,
            second,
        };

        (instruction, collaterals)
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: R{} - {} x {}",
            self.id, self.rank, self.first, self.second
        )
    }
}

#[derive(Debug, Clone)]
struct IntructionOperand(Either<GateOnLanes, usize>);

impl From<GateOnLanes> for IntructionOperand {
    fn from(gate: GateOnLanes) -> Self {
        Self(Either::Left(gate))
    }
}

impl From<usize> for IntructionOperand {
    fn from(id: usize) -> Self {
        Self(Either::Right(id))
    }
}

impl Deref for IntructionOperand {
    type Target = Either<GateOnLanes, usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for IntructionOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Either::Left(gate) => write!(f, "g:{}", gate),
            Either::Right(id) => write!(f, "id:{}", id),
        }
    }
}
