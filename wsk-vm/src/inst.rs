use crate::{
    program::ProgramParseError,
    value::{OpError, Value},
    vm::{VMError, VM},
};

#[derive(Debug, Clone, Copy)]
pub enum Inst {
    Halt,
    Push(Value),
    Pop,
    Load(usize),
    Store(usize),

    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Cmp(Cmp),

    Neg,
    Not,

    Jmp(isize),
    JmpTrue(isize),
    JmpFalse(isize),

    Call(usize),
    Ret,
}
impl RunInst for Inst {
    fn run(self, vm: &mut VM) -> Result<(), RunError> {
        Ok(match self {
            Self::Halt => vm.halt(),
            Inst::Push(v) => vm.push(v),
            Inst::Pop => vm.pop().map(|_| ())?,
            Inst::Load(idx) => {
                let v = vm.read_stack(idx)?;
                vm.push(*v);
            }
            Inst::Store(idx) => {
                let v = vm.read_stack(0)?;
                vm.replace_stack(idx, *v)?;
                vm.pop().map(|_| ())?;
            }

            Inst::Add => impl_macros::binary_op!(vm, +),
            Inst::Sub => impl_macros::binary_op!(vm, -),
            Inst::Mul => impl_macros::binary_op!(vm, *),
            Inst::Div => impl_macros::binary_op!(vm, /),
            Inst::And => impl_macros::binary_op!(vm, &),
            Inst::Or => impl_macros::binary_op!(vm, |),
            Inst::Cmp(cmp) => cmp.run(vm)?,

            Inst::Neg => impl_macros::unary_op!(vm, -),
            Inst::Not => impl_macros::unary_op!(vm, !),

            Inst::Jmp(offset) => vm.jump(offset),
            Inst::JmpTrue(offset) => {
                let Value::Bool(cond) = vm.pop()? else {
                    return Err(OpError::InvalidTypeForOp.into());
                };
                if cond {
                    vm.jump(offset);
                }
            }
            Inst::JmpFalse(offset) => {
                let Value::Bool(cond) = vm.pop()? else {
                    return Err(OpError::InvalidTypeForOp.into());
                };
                if !cond {
                    vm.jump(offset);
                }
            }

            Inst::Call(fi) => vm.call(fi),
            Inst::Ret => vm.ret()?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Cmp {
    Equal,
    Less,
    Greater,
}
impl RunInst for Cmp {
    fn run(self, vm: &mut VM) -> Result<(), RunError> {
        Ok(match self {
            Cmp::Equal => {
                let rhs = vm.pop()?;
                let lhs = vm.pop()?;
                vm.push((lhs == rhs).into());
            }
            Cmp::Less | Cmp::Greater => {
                let rhs = vm.pop()?;
                let lhs = vm.pop()?;
                let ord = match (lhs, rhs) {
                    (Value::Int(lhs), Value::Int(rhs)) => PartialOrd::partial_cmp(&lhs, &rhs),
                    _ => return Err(OpError::InvalidTypeForOp.into()),
                };
                let yes = match self {
                    Cmp::Less => matches!(ord, Some(std::cmp::Ordering::Less)),
                    Cmp::Greater => matches!(ord, Some(std::cmp::Ordering::Greater)),
                    _ => unreachable!(),
                };
                vm.push(yes.into());
            }
        })
    }
}

#[derive(Debug)]
pub enum RunError {
    VMError(VMError),
    OpError(OpError),
    ParseError(ProgramParseError),
    MissingSourcefile,
}

pub trait RunInst {
    fn run(self, vm: &mut VM) -> Result<(), RunError>;
}

impl From<Cmp> for Inst {
    fn from(value: Cmp) -> Self {
        Self::Cmp(value)
    }
}

impl From<VMError> for RunError {
    fn from(value: VMError) -> Self {
        Self::VMError(value)
    }
}
impl From<OpError> for RunError {
    fn from(value: OpError) -> Self {
        Self::OpError(value)
    }
}
impl From<ProgramParseError> for RunError {
    fn from(value: ProgramParseError) -> Self {
        Self::ParseError(value)
    }
}

mod impl_macros {
    macro_rules! binary_op {
    ($vm:expr, $sym:tt) => {{
        let rhs = $vm.pop()?;
        let lhs = $vm.pop()?;
        $vm.push((lhs $sym rhs)?);
    }};
}
    macro_rules! unary_op {
    ($vm:expr, $sym:tt) => {{
        let val = $vm.pop()?;
        $vm.push(($sym val)?);
    }};
}

    pub(super) use binary_op;
    pub(super) use unary_op;
}
