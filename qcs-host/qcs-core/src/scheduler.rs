use std::usize;

use hashbrown::HashMap;

use crate::{
    model::gates::CircuitGate,
    representations::tensor_networks::{TensorContraction, TensorKind},
};

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

impl From<TensorContraction> for ContractionPlan {
    fn from(contraction: TensorContraction) -> Self {
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
    pub dependencies: Vec<usize>,
    pub rank: u8,
    pub first: InstructionOperand,
    pub second: InstructionOperand,
}

impl Instruction {
    /// Create a new instruction with the given id from a contraction, returns also collaterals
    fn from_contraction(
        id: usize,
        contr: TensorContraction,
        collaterals: Vec<Instruction>,
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
                InstructionOperand::from(instr_id)
            }
            TensorKind::Gate(gate) => InstructionOperand::Gate(*gate),
        };

        let second = match right {
            TensorKind::Contraction(contr) => {
                let (instr, col) = Self::from_contraction(available_id, *contr, collaterals);
                collaterals = col;
                let instr_id = instr.id;
                dependencies.push(instr_id);
                collaterals.push(instr);
                InstructionOperand::from(instr_id)
            }
            TensorKind::Gate(gate) => InstructionOperand::Gate(*gate),
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

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Instruction {}

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
pub enum InstructionOperand {
    Gate(CircuitGate),
    Address(usize),
}

impl From<CircuitGate> for InstructionOperand {
    fn from(gate: CircuitGate) -> Self {
        Self::Gate(gate)
    }
}

impl From<usize> for InstructionOperand {
    fn from(id: usize) -> Self {
        Self::Address(id)
    }
}

impl std::fmt::Display for InstructionOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Gate(gate) => write!(f, "g:{}", gate),
            Self::Address(id) => write!(f, "id:{}", id),
        }
    }
}
