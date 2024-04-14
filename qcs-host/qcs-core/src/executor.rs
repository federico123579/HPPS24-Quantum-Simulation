use hashbrown::HashMap;

use crate::{
    model::blocks::SpannedBlock,
    scheduler::{Instruction, InstructionOperand},
};

#[derive(Debug, Clone, Default)]
pub struct CpuExecutor {
    memory: HashMap<usize, SpannedBlock>,
}

impl CpuExecutor {
    pub fn new() -> Self {
        Self {
            memory: HashMap::new(),
        }
    }

    #[inline]
    fn load_block(&mut self, instruction: InstructionOperand) -> SpannedBlock {
        match instruction {
            InstructionOperand::Gate(gate) => gate.into(),
            InstructionOperand::Address(address) => self.memory.remove(&address).unwrap(),
        }
    }

    #[inline]
    fn save_block(&mut self, id: usize, block: SpannedBlock) {
        self.memory.insert(id, block);
    }

    fn execute_single(&mut self, instruction: Instruction) {
        let Instruction {
            id, first, second, ..
        } = instruction;

        // dbg!(&id, &first, &second);

        let first_block = self.load_block(first);
        let second_block = self.load_block(second);

        let new_span = first_block.merged_span(&second_block);
        let first_block = first_block.adapt_to_span(new_span.clone());
        let second_block = second_block.adapt_to_span(new_span);

        let result = first_block * second_block;
        self.save_block(id, result);
    }

    pub fn execute(&mut self, instructions: Vec<Instruction>) -> Vec<SpannedBlock> {
        for instruction in instructions {
            // dbg!(&instruction);
            self.execute_single(instruction);
        }
        self.memory.drain().map(|(_, block)| block).collect()
    }
}
