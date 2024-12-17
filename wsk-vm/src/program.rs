use std::{io::Read, mem::size_of};

use crate::Inst;

#[derive(Debug, Clone)]
pub struct Program {
    funcs: Vec<Function>,
    entry_point: usize,
}
impl Program {
    pub fn new() -> Self {
        Self {
            funcs: vec![],
            entry_point: 0,
        }
    }

    pub fn from_bytes(mut bytes: &[u8]) -> Result<Self, ProgramParseError> {
        const U64_SIZE: usize = size_of::<u64>();
        let (fn_cnt, entry_fi) = {
            let mut header_bytes: [u8; U64_SIZE * 2] = Default::default();
            bytes
                .read(&mut header_bytes)
                .map_err(|_| ProgramParseError::InsufficientBytes)?;
            (
                u64::from_le_bytes(header_bytes[0..U64_SIZE].try_into().unwrap()),
                u64::from_le_bytes(header_bytes[U64_SIZE..].try_into().unwrap()),
            )
        };

        let mut funcs = Vec::new();
        for _ in 0..fn_cnt {
            funcs.push(Function::from_bytes(&mut bytes)?);
        }

        Ok(Self {
            funcs,
            entry_point: entry_fi as usize,
        })
    }

    pub fn set_entry_point(&mut self, index: usize) {
        debug_assert!(index < self.funcs.len(), "function index out of bound");
        self.entry_point = index;
    }

    pub fn add_func(&mut self, func: Function) -> usize {
        let id = self.funcs.len();
        self.funcs.push(func);
        id
    }

    pub fn get(&self, index: usize) -> Option<&Function> {
        self.funcs.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Function> {
        self.funcs.get_mut(index)
    }

    pub fn get_entry_point(&self) -> usize {
        self.entry_point
    }

    pub fn to_bin(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // func count
        bytes.extend((self.funcs.len() as u64).to_le_bytes());

        // entry fi
        bytes.extend((self.entry_point as u64).to_le_bytes());

        for func in &self.funcs {
            func.to_bin(&mut bytes);
        }

        bytes
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    insts: Vec<Inst>,
}
impl Function {
    pub fn new() -> Self {
        Self { insts: vec![] }
    }

    pub fn from_insts(insts: impl IntoIterator<Item = Inst>) -> Self {
        Self {
            insts: Vec::from_iter(insts),
        }
    }

    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, ProgramParseError> {
        const U64_SIZE: usize = size_of::<u64>();
        let inst_cnt = {
            let mut header_bytes: [u8; U64_SIZE] = Default::default();
            bytes
                .read(&mut header_bytes)
                .map_err(|_| ProgramParseError::InsufficientBytes)?;
            u64::from_le_bytes(header_bytes)
        };

        let mut insts = Vec::new();

        for _ in 0..inst_cnt {
            let inst = Inst::decode(bytes)?;
            insts.push(inst);
        }

        Ok(Self { insts })
    }

    pub fn push_inst(&mut self, inst: impl Into<Inst>) {
        self.insts.push(inst.into());
    }

    pub fn push_insts(&mut self, insts: impl IntoIterator<Item = Inst>) {
        self.insts.extend(insts);
    }

    pub fn insert_inst(&mut self, idx: usize, inst: impl Into<Inst>) {
        self.insts.insert(idx, inst.into());
    }

    pub fn len(&self) -> usize {
        self.insts.len()
    }

    pub fn get(&self, index: usize) -> Option<&Inst> {
        self.insts.get(index)
    }

    pub fn to_bin(&self, out: &mut Vec<u8>) {
        // inst count
        out.extend((self.insts.len() as u64).to_le_bytes());

        for inst in &self.insts {
            inst.encode(out);
        }
    }
}

#[derive(Debug)]
pub enum ProgramParseError {
    InsufficientBytes,
}
