mod tests;

use super::encoding::{copy, varint};

pub struct Delta {
    source_size: usize,
    target_size: usize,
    ops: Vec<Op>,
}

impl Delta {
    pub fn new(source_size: usize, target_size: usize, ops: Vec<Op>) -> Self {
        Delta {
            source_size,
            target_size,
            ops,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        let source_len = varint::encode(self.source_size).len();
        let target_len = varint::encode(self.target_size).len();
        let ops_len = self.ops.iter().map(|op| op.len()).sum::<usize>();

        source_len + target_len + ops_len
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend(varint::encode(self.source_size));
        buffer.extend(varint::encode(self.target_size));

        for op in &self.ops {
            buffer.extend(op.to_vec());
        }

        buffer
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Op {
    Copy(usize, usize),
    Insert(Vec<u8>),
}

impl Op {
    fn len(&self) -> usize {
        match self {
            Op::Copy(offset, size) => copy::encode(*offset, *size).len(),
            Op::Insert(buffer) => 1 + buffer.len(),
        }
    }

    fn to_vec(&self) -> Vec<u8> {
        match self {
            Op::Copy(offset, size) => copy::encode(*offset, *size),
            Op::Insert(buffer) => Op::prepend_length(buffer),
        }
    }

    fn prepend_length(buffer: &[u8]) -> Vec<u8> {
        let mut bytes = vec![buffer.len() as u8];
        bytes.extend(buffer);
        bytes
    }
}
