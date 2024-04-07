use std::{ops::Deref, usize};

use either::Either;
use hashbrown::HashMap;

use crate::{contractions::Contraction, model::GateOnLanes};

pub struct ContractionPlan {
    instructions: HashMap<usize, Instruction>,
    waiting_dep: HashMap<usize, Vec<usize>>,
    dependants: HashMap<usize, Vec<usize>>,
}

impl ContractionPlan {
    fn get_ready(&self) -> Vec<usize> {
        self.waiting_dep
            .iter()
            .filter(|(_, deps)| deps.is_empty())
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn set_done(&mut self, ids: impl IntoIterator<Item = usize>) {
        for id in ids {
            assert!(self.waiting_dep.get(&id).unwrap().is_empty());
            self.waiting_dep.remove(&id);
            let deps = self.dependants.remove(&id).unwrap();
            for dep in deps {
                let waiting = self.waiting_dep.get_mut(&dep).unwrap();
                waiting.retain(|&iid| iid != id);
            }
        }
    }

    pub fn fetch_ready(&mut self) -> Vec<Instruction> {
        let ready = self.get_ready();
        ready
            .iter()
            .map(|id| self.instructions.remove(id).unwrap())
            .collect()
    }
}

impl From<Contraction> for ContractionPlan {
    fn from(contraction: Contraction) -> Self {
        let (instruction, collaterals) = Instruction::from_contraction(0, contraction, vec![]);

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

#[derive(Debug, Clone)]
pub struct Instruction {
    pub id: usize,
    dependencies: Vec<usize>,
    pub rank: u8,
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

    fn dependencies(&self) -> &[usize] {
        &self.dependencies
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
