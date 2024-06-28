use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use crate::{
    model::blocks::Block,
    scheduler::{
        operation::{Kernel, MatrixFormat, OperationInstruction},
        ExecutionOperand,
    },
};
use nalgebra::{Complex, DMatrix, Normed};

const ZERO_THRESHOLD: f64 = 1e-10;

pub struct BinaryConfig {
    matrix_format: MatrixFormat,
}

impl BinaryConfig {
    pub fn column_major() -> Self {
        Self {
            matrix_format: MatrixFormat::ColumnMajor,
        }
    }

    pub fn row_major() -> Self {
        Self {
            matrix_format: MatrixFormat::RowMajor,
        }
    }

    fn invert(&self) -> Self {
        match self.matrix_format {
            MatrixFormat::RowMajor => Self::column_major(),
            MatrixFormat::ColumnMajor => Self::row_major(),
        }
    }
}

impl Default for BinaryConfig {
    fn default() -> Self {
        Self {
            matrix_format: MatrixFormat::RowMajor,
        }
    }
}

pub trait ToBinary {
    fn to_binary(&self, config: &BinaryConfig) -> Vec<u8>;
}

fn count_non_zero(matrix: &DMatrix<Complex<f64>>) -> usize {
    matrix
        .iter()
        .fold(0, |a, b| if b.norm() > ZERO_THRESHOLD { a + 1 } else { a })
}

/// Binary format for matrices (sparse COO format):
/// - 4 bytes: number of non zero elements (u32)
/// - for each element:
///     - 4 bytes: row index (u32)
///     - 4 bytes: column index (u32)
///     - 8 bytes: real part (f64)
///     - 8 bytes: imaginary part (f64)
impl ToBinary for DMatrix<Complex<f64>> {
    fn to_binary(&self, config: &BinaryConfig) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&(count_non_zero(self) as u32).to_le_bytes());
        match config.matrix_format {
            MatrixFormat::RowMajor => {
                self.row_iter().enumerate().for_each(|(i, row)| {
                    row.iter()
                        .enumerate()
                        .filter(|(_, c)| c.norm() > ZERO_THRESHOLD)
                        .for_each(|(j, c)| {
                            bytes.extend_from_slice(&(i as u32).to_le_bytes());
                            bytes.extend_from_slice(&(j as u32).to_le_bytes());
                            bytes.extend_from_slice(&c.re.to_le_bytes());
                            bytes.extend_from_slice(&c.im.to_le_bytes());
                        });
                });
            }
            MatrixFormat::ColumnMajor => {
                self.column_iter().enumerate().for_each(|(j, col)| {
                    col.iter()
                        .enumerate()
                        .filter(|(_, c)| c.norm() > 1e-10)
                        .for_each(|(i, c)| {
                            bytes.extend_from_slice(&(i as u32).to_le_bytes());
                            bytes.extend_from_slice(&(j as u32).to_le_bytes());
                            bytes.extend_from_slice(&c.re.to_le_bytes());
                            bytes.extend_from_slice(&c.im.to_le_bytes());
                        });
                });
            }
        }
        bytes
    }
}

impl ToBinary for Block {
    fn to_binary(&self, config: &BinaryConfig) -> Vec<u8> {
        self.as_ref().to_binary(config)
    }
}

/// Binary format for a quantum gate:
/// - 4 bytes: id of operation (u32)
/// - 1 byte: kind of operation (u8):
///   - 0x00: TE(MxM)
///   - 0x01: TE(MxA)
///   - 0x02: TE(AxM)
///   - 0x03: TE(AxA)
///   - 0x04: MM(MxM)
///   - 0x05: MM(MxA)
///   - 0x06: MM(AxM)
///   - 0x07: MM(AxA)
/// - left operand: can be a sparse COO matrix or an id of an operation
///   - if kind is matrix:
///     - matrix in sparse COO format
///   - if kind is operation:
///     - 4 bytes: id of operation (u32)
/// - right operand in sparse COO format
///   - if kind is matrix:
///     - matrix in sparse COO format
///   - if kind is operation:
///     - 4 bytes: id of operation (u32)
impl ToBinary for OperationInstruction {
    fn to_binary(&self, _: &BinaryConfig) -> Vec<u8> {
        let mut bytes = Vec::new();
        let OperationInstruction {
            id,
            kernel,
            left_format,
            ..
        } = self;
        let config = BinaryConfig {
            matrix_format: left_format.to_owned(),
        };
        bytes.extend((*id as u32).to_le_bytes());
        match kernel {
            Kernel::TE { left, right } => match (left, right) {
                (ExecutionOperand::Block(b1), ExecutionOperand::Block(b2)) => {
                    bytes.push(0x00);
                    bytes.extend(b1.to_binary(&config));
                    bytes.extend(b2.to_binary(&config));
                }
                (ExecutionOperand::Block(b1), ExecutionOperand::Address(a2)) => {
                    bytes.push(0x01);
                    bytes.extend(b1.to_binary(&config));
                    bytes.extend((*a2 as u32).to_le_bytes());
                }
                (ExecutionOperand::Address(a1), ExecutionOperand::Block(b2)) => {
                    bytes.push(0x02);
                    bytes.extend((*a1 as u32).to_le_bytes());
                    bytes.extend(b2.to_binary(&config));
                }
                (ExecutionOperand::Address(a1), ExecutionOperand::Address(a2)) => {
                    bytes.push(0x03);
                    bytes.extend((*a1 as u32).to_le_bytes());
                    bytes.extend((*a2 as u32).to_le_bytes());
                }
            },
            Kernel::MM { left, right } => match (left, right) {
                (ExecutionOperand::Block(b1), ExecutionOperand::Block(b2)) => {
                    bytes.push(0x04);
                    bytes.extend(b1.to_binary(&config));
                    bytes.extend(b2.to_binary(&config.invert()));
                }
                (ExecutionOperand::Block(b1), ExecutionOperand::Address(a2)) => {
                    bytes.push(0x05);
                    bytes.extend(b1.to_binary(&config));
                    bytes.extend((*a2 as u32).to_le_bytes());
                }
                (ExecutionOperand::Address(a1), ExecutionOperand::Block(b2)) => {
                    bytes.push(0x06);
                    bytes.extend((*a1 as u32).to_le_bytes());
                    bytes.extend(b2.to_binary(&config.invert()));
                }
                (ExecutionOperand::Address(a1), ExecutionOperand::Address(a2)) => {
                    bytes.push(0x07);
                    bytes.extend((*a1 as u32).to_le_bytes());
                    bytes.extend((*a2 as u32).to_le_bytes());
                }
            },
        }

        bytes
    }
}

pub struct BinaryFile {
    file: BufWriter<File>,
}

impl BinaryFile {
    pub fn new(path: PathBuf) -> std::io::Result<Self> {
        let file = File::create(path)?;
        Ok(Self {
            file: BufWriter::new(file),
        })
    }

    pub fn add_operation_instruction(&mut self, op: &OperationInstruction) -> std::io::Result<()> {
        self.file.write_all(&op.to_binary(&Default::default()))
    }
}
