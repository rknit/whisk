use std::{
    fmt::Display,
    mem::discriminant,
    ops::{Add, BitAnd, BitOr, Div, Mul, Neg, Not, Rem, Sub},
};

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Int(i64),
    Bool(bool),
}

#[derive(Debug)]
pub enum OpError {
    TypeMismatched,
    InvalidTypeForOp,
}

impl_macros::impl_math_bin_op!(Add, add, +);
impl_macros::impl_math_bin_op!(Sub, sub, -);
impl_macros::impl_math_bin_op!(Mul, mul, *);
impl_macros::impl_math_bin_op!(Div, div, /);
impl_macros::impl_math_bin_op!(Rem, rem, %);
impl_macros::impl_logic_bin_op!(BitAnd, bitand, &&);
impl_macros::impl_logic_bin_op!(BitOr, bitor, ||);

impl_macros::impl_math_unary_op!(Neg, neg, -);
impl_macros::impl_logic_unary_op!(Not, not, !);

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(lhs), Self::Int(rhs)) => lhs == rhs,
            (Self::Bool(lhs), Self::Bool(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Int(lhs), Self::Int(rhs)) => PartialOrd::partial_cmp(lhs, rhs),
            _ => None,
        }
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}
impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(v) => write!(f, "{}", v),
            Value::Bool(v) => write!(f, "{}", v),
        }
    }
}

mod impl_macros {
    macro_rules! impl_math_bin_op {
    ($op:ident, $op_fn:ident, $sym:tt) => {
        impl $op for Value {
            type Output = Result<Self, OpError>;

            fn $op_fn(self, rhs: Self) -> Self::Output {
                if discriminant(&self) != discriminant(&rhs) {
                    return Err(OpError::TypeMismatched);
                }

                Ok(match self {
                    Self::Int(lhs) => {
                        let Self::Int(rhs) = rhs else { unreachable!() };
                        Self::Int(lhs $sym rhs)
                    }
                    Self::Bool(_) => return Err(OpError::InvalidTypeForOp),
                })
            }
        }
    };
}
    macro_rules! impl_logic_bin_op {
    ($op:ident, $op_fn:ident, $sym:tt) => {
        impl $op for Value {
            type Output = Result<Self, OpError>;

            fn $op_fn(self, rhs: Self) -> Self::Output {
                if discriminant(&self) != discriminant(&rhs) {
                    return Err(OpError::TypeMismatched);
                }

                Ok(match self {
                    Self::Int(_) => return Err(OpError::InvalidTypeForOp),
                    Self::Bool(lhs) => {
                        let Self::Bool(rhs) = rhs else { unreachable!() };
                        Self::Bool(lhs $sym rhs)
                    }
                })
            }
        }
    };
}

    macro_rules! impl_math_unary_op {
    ($op:ident, $op_fn:ident, $sym:tt) => {
        impl $op for Value {
            type Output = Result<Self, OpError>;

            fn $op_fn(self) -> Self::Output {
                Ok(match self {
                    Self::Int(val) => Self::Int($sym val),
                    Self::Bool(_) => return Err(OpError::InvalidTypeForOp),
                })
            }
        }
    };
}
    macro_rules! impl_logic_unary_op {
    ($op:ident, $op_fn:ident, $sym:tt) => {
        impl $op for Value {
            type Output = Result<Self, OpError>;

            fn $op_fn(self) -> Self::Output {
                Ok(match self {
                    Self::Int(_) => return Err(OpError::InvalidTypeForOp),
                    Self::Bool(val) => Self::Bool($sym val),
                })
            }
        }
    };
}

    pub(super) use impl_logic_bin_op;
    pub(super) use impl_logic_unary_op;
    pub(super) use impl_math_bin_op;
    pub(super) use impl_math_unary_op;
}
