//! Module for scheduling the contractions to be executed in the quantum computer
//! simulator.
//!
//! The scheduler is responsible for taking a tensor contraction and creating a
//! plan of instructions to be executed in the simulator. The scheduler will
//! create a plan of instructions that can be executed in parallel, and will
//! return the instructions in the order they can be executed.

use hashbrown::HashMap;
use nalgebra::{Complex, DMatrix};

use crate::{model::gates::QuantumGate, op_tree};

/// A plan of instructions to be executed in the simulator
/// The plan is a list of instructions that can be executed in parallel
/// and the dependencies between them.
#[derive(Debug, Clone)]
pub struct OperationPlan {
    /// The instructions to be executed.
    instructions: HashMap<usize, OperationInstruction>,
    /// The dependencies of each instruction.
    waiting_dep: HashMap<usize, Vec<usize>>,
    /// The dependants of each instruction.
    dependants: HashMap<usize, Vec<usize>>,
}

impl OperationPlan {
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
    pub fn fetch_ready(&mut self) -> Vec<OperationInstruction> {
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

impl From<op_tree::Operation> for OperationPlan {
    fn from(value: op_tree::Operation) -> Self {
        let mut builder = OperationPlanBuilder::default();
        builder.populate(value);
        builder.build()
    }
}

impl std::fmt::Display for OperationPlan {
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

#[derive(Default)]
struct OperationPlanBuilder {
    next_available_id: usize,
    instructions: Vec<OperationInstruction>,
}

impl OperationPlanBuilder {
    fn new_id(&mut self) -> usize {
        let id = self.next_available_id;
        self.next_available_id += 1;
        id
    }

    fn populate(&mut self, operation: op_tree::Operation) -> usize {
        let op_tree::Operation {
            inner,
            transposed_op,
        } = operation;
        let address = match inner {
            op_tree::OperationKind::TensorExpansion {
                target_span,
                operand,
            } => {
                let mut dependencies = Vec::new();
                let op_span = operand.span();
                let op = match operand {
                    op_tree::Operand::Operation(op) => {
                        let dep = self.populate(*op);
                        dependencies.push(dep);
                        KernelOperand::Address(dep)
                    }
                    op_tree::Operand::Gate(gate) => KernelOperand::from(gate.matrix()),
                };

                let id_dim = op_span.start() - target_span.start();
                let id_matrix = DMatrix::identity(id_dim, id_dim);
                let left = KernelOperand::from(id_matrix);
                let id = self.new_id();
                let first_te = OperationInstruction {
                    id,
                    dependencies,
                    kernel: Kernel::TE { left, right: op },
                    left_format: if transposed_op {
                        SerializeFormat::ColumnMajor
                    } else {
                        SerializeFormat::RowMajor
                    },
                };
                self.instructions.push(first_te);

                let left = KernelOperand::Address(id);
                let id_dim = target_span.end() - op_span.end();
                let id_matrix = DMatrix::identity(id_dim, id_dim);
                let right = KernelOperand::from(id_matrix);
                let id = self.new_id();
                let second_te = OperationInstruction {
                    id,
                    dependencies: vec![id],
                    kernel: Kernel::TE { left, right },
                    left_format: SerializeFormat::ColumnMajor,
                };
                self.instructions.push(second_te);
                id
            }
            op_tree::OperationKind::MatrixMultiplication { left, right } => {
                let mut dependencies = Vec::new();
                let left = match left {
                    op_tree::Operand::Operation(op) => {
                        let dep = self.populate(*op);
                        dependencies.push(dep);
                        KernelOperand::Address(dep)
                    }
                    op_tree::Operand::Gate(gate) => KernelOperand::from(gate.as_ref().matrix()),
                };
                let right = match right {
                    op_tree::Operand::Operation(op) => {
                        let dep = self.populate(*op);
                        dependencies.push(dep);
                        KernelOperand::Address(dep)
                    }
                    op_tree::Operand::Gate(gate) => KernelOperand::from(gate.as_ref().matrix()),
                };
                let id = self.new_id();
                let mm = OperationInstruction {
                    id,
                    dependencies,
                    kernel: Kernel::MM { left, right },
                    left_format: if transposed_op {
                        SerializeFormat::ColumnMajor
                    } else {
                        SerializeFormat::RowMajor
                    },
                };
                self.instructions.push(mm);
                id
            }
        };
        address
    }

    fn build(self) -> OperationPlan {
        let instructions = self
            .instructions
            .into_iter()
            .map(|instr| (instr.id, instr))
            .collect::<HashMap<_, _>>();
        let waiting_dep: HashMap<usize, _> = instructions
            .iter()
            .map(|(id, instr)| (*id, instr.dependencies.clone()))
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
        OperationPlan {
            instructions,
            waiting_dep,
            dependants,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OperationInstruction {
    pub id: usize,
    pub dependencies: Vec<usize>,
    pub kernel: Kernel,
    pub left_format: SerializeFormat,
}

impl PartialEq for OperationInstruction {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for OperationInstruction {}

impl std::fmt::Display for OperationInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:03}: {}", self.id, self.kernel)
    }
}

#[derive(Debug, Clone)]
pub enum SerializeFormat {
    ColumnMajor,
    RowMajor,
}

#[derive(Debug, Clone)]
pub enum Kernel {
    TE {
        left: KernelOperand,
        right: KernelOperand,
    },
    MM {
        left: KernelOperand,
        right: KernelOperand,
    },
}

impl std::fmt::Display for Kernel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::TE { left, right } => write!(f, "{}⊗ {}", left, right),
            Self::MM { left, right } => write!(f, "{}⊙ {}", left, right),
        }
    }
}

#[derive(Debug, Clone)]
pub enum KernelOperand {
    Address(usize),
    Matrix(DMatrix<Complex<f64>>),
}

impl From<DMatrix<Complex<f64>>> for KernelOperand {
    fn from(matrix: DMatrix<Complex<f64>>) -> Self {
        Self::Matrix(matrix)
    }
}

impl From<usize> for KernelOperand {
    fn from(id: usize) -> Self {
        Self::Address(id)
    }
}

impl std::fmt::Display for KernelOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Matrix(matrix) => write!(f, "M({:02}x{:02})", matrix.nrows(), matrix.ncols()),
            Self::Address(id) => write!(f, "I({:05})", id),
        }
    }
}
