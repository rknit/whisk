use uuid::Uuid;

use crate::ty::Type;

use super::{basic_block::BasicBlockID, value::Value};

#[derive(Debug, Clone)]
pub struct TaggedInst {
    pub id: InstID,
    pub inst: Inst,
}

pub type InstID = Uuid;

#[derive(Debug, Clone)]
pub struct Inst {
    pub kind: InstKind,
    pub ty: Type,
}
impl Inst {
    pub fn is_terminate_inst(&self) -> bool {
        self.kind.is_terminate_inst()
    }
}

#[derive(Debug, Clone)]
pub enum InstKind {
    Alloca,
    Load(LoadInst),
    Store(StoreInst),
    Branch(BranchInst),
    BranchCond(BranchCondInst),
    Return(ReturnInst),

    Negate(NegateInst),
    Not(NotInst),

    Add(AddInst),
    Sub(SubInst),
    And(AndInst),
    Or(OrInst),
    Compare(CompareInst),

    Call(CallInst),
}
impl InstKind {
    pub fn is_terminate_inst(&self) -> bool {
        match self {
            Self::Branch(_) | Self::BranchCond(_) | Self::Return(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CallInst {
    pub callee: Value,
    pub args: Vec<Value>,
}

#[derive(Debug, Clone, Copy)]
pub struct CompareInst {
    pub cond: CompareCond,
    pub lhs: Value,
    pub rhs: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareCond {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Debug, Clone, Copy)]
pub struct AddInst {
    pub lhs: Value,
    pub rhs: Value,
}

#[derive(Debug, Clone, Copy)]
pub struct SubInst {
    pub lhs: Value,
    pub rhs: Value,
}

#[derive(Debug, Clone, Copy)]
pub struct AndInst {
    pub lhs: Value,
    pub rhs: Value,
}

#[derive(Debug, Clone, Copy)]
pub struct OrInst {
    pub lhs: Value,
    pub rhs: Value,
}

#[derive(Debug, Clone, Copy)]
pub struct NotInst {
    pub value: Value,
}

#[derive(Debug, Clone, Copy)]
pub struct NegateInst {
    pub value: Value,
}

#[derive(Debug, Clone, Copy)]
pub struct LoadInst {
    pub value: Value,
}

#[derive(Debug, Clone, Copy)]
pub struct StoreInst {
    pub target: Value,
    pub value: Value,
}

#[derive(Debug, Clone, Copy)]
pub struct BranchInst {
    pub branch: BasicBlockID,
}

#[derive(Debug, Clone, Copy)]
pub struct BranchCondInst {
    pub cond: Value,
    pub then_branch: BasicBlockID,
    pub else_branch: BasicBlockID,
}

#[derive(Debug, Clone, Copy)]
pub struct ReturnInst {
    pub value: Option<Value>,
}
