use super::func::Function;

#[derive(Debug, Clone)]
pub struct Program {
    pub funcs: Vec<Function>,
}
