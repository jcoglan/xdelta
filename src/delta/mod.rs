#[cfg(test)]
mod tests;

pub struct Delta {
    #[allow(unused)]
    ops: Vec<Op>,
}

impl Delta {
    pub fn new(ops: Vec<Op>) -> Self {
        Delta { ops }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Op {
    Copy(usize, usize),
    Insert(Vec<u8>),
}
