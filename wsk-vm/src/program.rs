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

    pub fn set_entry_point(&mut self, index: usize) {
        debug_assert!(index < self.funcs.len(), "function index out of bound");
        self.entry_point = index;
    }

    pub fn push_func(&mut self, func: Function) -> usize {
        let id = self.funcs.len();
        self.funcs.push(func);
        id
    }

    pub fn get(&self, index: usize) -> Option<&Function> {
        self.funcs.get(index)
    }

    pub fn get_entry_point(&self) -> usize {
        self.entry_point
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

    pub fn push_inst(&mut self, inst: impl Into<Inst>) {
        self.insts.push(inst.into());
    }

    pub fn push_insts(&mut self, insts: impl IntoIterator<Item = Inst>) {
        self.insts.extend(insts);
    }

    pub fn get(&self, index: usize) -> Option<&Inst> {
        self.insts.get(index)
    }
}
