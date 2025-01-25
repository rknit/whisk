use std::fmt::Display;

use super::inst::Inst;

#[derive(Debug, Clone)]
pub struct Program {
    funcs: Vec<Function>,
    entry_point: usize,
}
impl Default for Program {
    fn default() -> Self {
        Self::new(0)
    }
}
impl Program {
    pub fn new(entry_point: usize) -> Self {
        Self {
            funcs: vec![],
            entry_point,
        }
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
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "entry: ${}\n", self.entry_point)?;
        for (i, func) in self.funcs.iter().enumerate() {
            writeln!(f, "func ${}:\n{}", i, func)?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct Function {
    insts: Vec<Inst>,
}
impl Function {
    pub fn from_insts(insts: impl IntoIterator<Item = Inst>) -> Self {
        Self {
            insts: Vec::from_iter(insts),
        }
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

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<&Inst> {
        self.insts.get(index)
    }

    pub fn get_insts(&self) -> &Vec<Inst> {
        &self.insts
    }
}
impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, inst) in self.insts.iter().enumerate() {
            writeln!(f, "\t{}: {}", i, inst)?;
        }
        Ok(())
    }
}
