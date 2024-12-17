use wsk_vm::Inst;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub insts: Vec<Inst>,
}
