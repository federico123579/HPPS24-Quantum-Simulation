//! Module for scheduling the contractions to be executed in the quantum computer
//! simulator.
//!
//! The scheduler is responsible for taking a tensor contraction and creating a
//! plan of instructions to be executed in the simulator. The scheduler will
//! create a plan of instructions that can be executed in parallel, and will
//! return the instructions in the order they can be executed.

use hashbrown::HashMap;

use crate::{
    contractions::{TensorContraction, TensorKind},
    executor::{BlockStore, Computation, ExecutorPlan, InstructionLike},
    model::blocks::SpannedBlock,
};

use super::ExecutionOperand;

/// A plan of instructions to be executed in the simulator
/// The plan is a list of instructions that can be executed in parallel
/// and the dependencies between them.
#[derive(Debug, Clone)]
pub struct ContractionPlan {
    /// The instructions to be executed.
    instructions: HashMap<usize, ContractionInstruction>,
    /// The dependencies of each instruction.
    waiting_dep: HashMap<usize, Vec<usize>>,
    /// The dependants of each instruction.
    dependants: HashMap<usize, Vec<usize>>,
}

impl ContractionPlan {
    /// Extract the instructions that are ready to be executed, these are the
    /// instructions that have no dependencies.
    fn get_ready(&self) -> Vec<usize> {
        self.waiting_dep
            .iter()
            .filter(|(_, deps)| deps.is_empty())
            .map(|(id, _)| *id)
            .collect()
    }

    /// Mark the instructions as done, removing them from the plan and updating
    /// the dependencies of the remaining instructions.
    pub fn set_done(&mut self, ids: impl IntoIterator<Item = usize>) {
        for id in ids {
            assert!(self.waiting_dep.get(&id).unwrap().is_empty());
            self.instructions.remove(&id);
            self.waiting_dep.remove(&id);
            let deps = self.dependants.remove(&id).unwrap();
            for dep in deps {
                let waiting = self.waiting_dep.get_mut(&dep).unwrap();
                waiting.retain(|&iid| iid != id);
            }
        }
    }

    /// Fetch the instructions that are ready to be executed.
    /// The instructions are removed from the plan.
    pub fn fetch_ready(&mut self) -> Vec<ContractionInstruction> {
        let ready = self.get_ready();
        ready
            .iter()
            .map(|id| self.instructions.get(id).unwrap().clone())
            .collect()
    }

    /// Check if the plan is empty.
    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }
}

impl ExecutorPlan for ContractionPlan {
    type Instruction = ContractionInstruction;

    fn get_ready(&self) -> Vec<usize> {
        self.get_ready()
    }

    fn set_done(&mut self, ids: impl IntoIterator<Item = usize>) {
        self.set_done(ids)
    }

    fn fetch_ready(&mut self) -> Vec<Self::Instruction> {
        self.fetch_ready()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl From<TensorContraction> for ContractionPlan {
    fn from(contraction: TensorContraction) -> Self {
        let (instruction, collaterals) =
            ContractionInstruction::from_contraction(0, contraction, vec![]);

        let instructions: HashMap<_, _> = collaterals
            .into_iter()
            .chain(std::iter::once(instruction))
            .map(|instr| (instr.id, instr))
            .collect();
        let waiting_dep: HashMap<usize, _> = instructions
            .iter()
            .map(|(id, instr)| (*id, instr.dependencies().to_vec()))
            .collect();
        let dependants = {
            let mut dependants: HashMap<_, _> =
                instructions.iter().map(|(id, _)| (*id, vec![])).collect();
            for (id, deps) in &waiting_dep {
                for dep in deps {
                    dependants.get_mut(dep).unwrap().push(*id);
                }
            }
            dependants
        };

        Self {
            instructions,
            waiting_dep,
            dependants,
        }
    }
}

impl std::fmt::Display for ContractionPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Instructions:")?;
        for (_, instr) in &self.instructions {
            writeln!(f, "{}", instr)?;
        }
        writeln!(f, "Waiting dependencies:")?;
        for (id, deps) in &self.waiting_dep {
            writeln!(f, "{}: {:?}", id, deps)?;
        }
        writeln!(f, "Dependants:")?;
        for (id, deps) in &self.dependants {
            writeln!(f, "{}: {:?}", id, deps)?;
        }
        Ok(())
    }
}

/// An instruction to be executed in the simulator
/// The instruction is a tensor contraction to be executed in the simulator
/// and the dependencies of the instruction.
#[derive(Debug, Clone)]
pub struct ContractionInstruction {
    /// The id of the instruction
    pub id: usize,
    /// The dependencies on which this instruction depends
    pub dependencies: Vec<usize>,
    /// The rank of the resulting tensor
    pub rank: u8,
    /// The left operand of the instruction
    pub first: ExecutionOperand<SpannedBlock>,
    /// The right operand of the instruction
    pub second: ExecutionOperand<SpannedBlock>,
}

impl ContractionInstruction {
    /// Create an instruction from a tensor contraction
    /// The instruction will be created recursively from the tensor contraction
    /// and the collaterals will be returned as well.
    ///
    /// The id is the id of the instruction to be created, the contr is the
    /// tensor contraction to be executed, and the collaterals are the
    /// instructions that have been created so far.
    fn from_contraction(
        id: usize,
        contr: TensorContraction,
        collaterals: Vec<ContractionInstruction>,
    ) -> (Self, Vec<Self>) {
        let mut collaterals = collaterals;
        let mut available_id = id + 1;
        let mut dependencies = Vec::new();

        let TensorContraction {
            lhs: left,
            rhs: right,
            rank,
            ..
        } = contr;

        let first = match left {
            TensorKind::Contraction(contr) => {
                let (instr, col) = Self::from_contraction(available_id, *contr, collaterals);
                collaterals = col;
                let instr_id = instr.id;
                dependencies.push(instr_id);
                collaterals.push(instr);
                available_id = collaterals.iter().map(|i| i.id).max().unwrap() + 1;
                ExecutionOperand::from(instr_id)
            }
            TensorKind::Gate(gate) => ExecutionOperand::Block(SpannedBlock::from(*gate)),
        };

        let second = match right {
            TensorKind::Contraction(contr) => {
                let (instr, col) = Self::from_contraction(available_id, *contr, collaterals);
                collaterals = col;
                let instr_id = instr.id;
                dependencies.push(instr_id);
                collaterals.push(instr);
                ExecutionOperand::from(instr_id)
            }
            TensorKind::Gate(gate) => ExecutionOperand::Block(SpannedBlock::from(*gate)),
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

    /// Get the dependencies of the instruction
    fn dependencies(&self) -> &[usize] {
        &self.dependencies
    }
}

impl Computation for ContractionInstruction {
    type BlockKind = SpannedBlock;

    fn compute(self, block_map: &impl BlockStore<Self::BlockKind>) -> usize {
        let ContractionInstruction {
            id, first, second, ..
        } = self;

        let first_block = block_map.load_block(first);
        let second_block = block_map.load_block(second);

        let new_span = first_block.merged_span(&second_block);
        let first_block = first_block.adapt_to_span(new_span.clone());
        let second_block = second_block.adapt_to_span(new_span);

        let out = first_block * second_block;
        block_map.save_block(id, out);
        id
    }
}

impl InstructionLike for ContractionInstruction {
    fn id(&self) -> usize {
        self.id
    }
}

impl PartialEq for ContractionInstruction {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ContractionInstruction {}

impl std::fmt::Display for ContractionInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: R{} - {} x {}",
            self.id, self.rank, self.first, self.second
        )
    }
}
