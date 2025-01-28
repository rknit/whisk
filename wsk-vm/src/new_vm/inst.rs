use crate::{bin_op, unary_op, value::OpError, Value};

use super::{
    abi::Reg,
    vm::{VMError, VM},
};
use paste::paste;

macro_utils::insts!(
    /*
    *   VM manipulation insts
    */

    HLT: 0x00 => (vm) {
        vm.halt();
        Ok(())
    },
    PUSH => (vm, reg: Reg) {
        let v = *vm.get_reg(reg);
        vm.push_value(v)?;
        Ok(())
    },
    PUSHV => (vm, value: Value) {
        vm.push_value(value)?;
        Ok(())
    },
    POP => (vm, dest: Reg) {
        let v = vm.pop_value()?;
        *vm.get_reg_mut(dest) = v;
        Ok(())
    },
    MOV => (vm, dest: Reg, org: Reg) {
        *vm.get_reg_mut(dest) = *vm.get_reg(org);
        Ok(())
    },
    MOVV => (vm, dest: Reg, value: Value) {
        *vm.get_reg_mut(dest) = value;
        Ok(())
    },

    /*
    *   binary operation insts
    */

    ADD => (vm, dest: Reg, p0: Reg, p1: Reg) bin_op!(vm, dest, p0 +? p1),
    ADDV => (vm, dest: Reg, p0: Reg, p1: Value) bin_op!(vm, dest, p0 +? v:p1),
    SUB => (vm, dest: Reg, p0: Reg, p1: Reg) bin_op!(vm, dest, p0 -? p1),
    SUBV => (vm, dest: Reg, p0: Reg, p1: Value) bin_op!(vm, dest, p0 -? v:p1),
    MUL => (vm, dest: Reg, p0: Reg, p1: Reg) bin_op!(vm, dest, p0 *? p1),
    MULV => (vm, dest: Reg, p0: Reg, p1: Value) bin_op!(vm, dest, p0 *? v:p1),
    DIV => (vm, dest: Reg, p0: Reg, p1: Reg) bin_op!(vm, dest, p0 /? p1),
    DIVV => (vm, dest: Reg, p0: Reg, p1: Value) bin_op!(vm, dest, p0 /? v:p1),

    AND => (vm, dest: Reg, p0: Reg, p1: Reg) bin_op!(vm, dest, p0 &? p1),
    ANDV => (vm, dest: Reg, p0: Reg, p1: Value) bin_op!(vm, dest, p0 &? v:p1),
    OR => (vm, dest: Reg, p0: Reg, p1: Reg) bin_op!(vm, dest, p0 |? p1),
    ORV => (vm, dest: Reg, p0: Reg, p1: Value) bin_op!(vm, dest, p0 |? v:p1),

    CMPEQ => (vm, dest: Reg, p0: Reg, p1: Reg) bin_op!(vm, dest, p0 == p1),
    CMPEQV => (vm, dest: Reg, p0: Reg, p1: Value) bin_op!(vm, dest, p0 == v:p1),
    CMPNE => (vm, dest: Reg, p0: Reg, p1: Reg) bin_op!(vm, dest, p0 != p1),
    CMPNEV => (vm, dest: Reg, p0: Reg, p1: Value) bin_op!(vm, dest, p0 != v:p1),
    CMPLT => (vm, dest: Reg, p0: Reg, p1: Reg) bin_op!(vm, dest, p0 < p1),
    CMPLTV => (vm, dest: Reg, p0: Reg, p1: Value) bin_op!(vm, dest, p0 < v:p1),
    CMPGT => (vm, dest: Reg, p0: Reg, p1: Reg) bin_op!(vm, dest, p0 > p1),
    CMPGTV => (vm, dest: Reg, p0: Reg, p1: Value) bin_op!(vm, dest, p0 > v:p1),

    /*
    *   unary operation insts
    */

    NOT => (vm, dest: Reg, p0: Reg) unary_op!(vm, dest, !?p0),
    NEG => (vm, dest: Reg, p0: Reg) unary_op!(vm, dest, -?p0),

    /*
    *   control flow insts
    */

    JMP => (vm, offset: isize) {
        vm.jump(offset);
        Ok(())
    },
    JTR => (vm, reg: Reg, offset: isize) {
        let Value::Bool(v) = vm.get_reg(reg) else {
            return Err(OpError::InvalidTypeForOp.into());
        };
        if *v {
            vm.jump(offset);
        }
        Ok(())
    },
    JFL => (vm, reg: Reg, offset: isize) {
        let Value::Bool(v) = vm.get_reg(reg) else {
            return Err(OpError::InvalidTypeForOp.into());
        };
        if !(*v) {
            vm.jump(offset);
        }
        Ok(())
    },
    CALL => (vm, fi: usize) {
        vm.call(fi);
        Ok(())
    },
    RET => (vm) {
        vm.ret()?;
        Ok(())
    },
);

#[derive(Debug)]
pub enum RunError {
    VMError(VMError),
    OpError(OpError),
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

mod macro_utils {
    #[macro_export]
    macro_rules! insts {
        ($(
            $name:ident $(: $code:literal)? => ( $vm:ident $(,)? $( $($param_name:ident: $param_ty:ty),+ )? ) $body:expr
        ),+ $(,)?) => {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            #[repr(u8)]
            pub enum Inst {
                $($name $({ $($param_name: $param_ty),+ })? $(= $code)?),+
            }

            impl Inst {
                pub fn run(self, vm: &mut VM) -> Result<(), RunError> {
                    paste!{
                    match self {
                        $(Inst::$name $({ $( $param_name ),+ })? => [<run_ $name:lower _inst>] (vm, $( $($param_name),+ )? ) ),+
                    }
                    }
                }
            }

            impl std::fmt::Display for Inst {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let inst_name;
                    let mut inst_str = String::new();

                    paste!{
                    match self {
                        $(Inst::$name $({ $( $param_name ),+ })? => {
                            inst_name = stringify!($name);
                            $($(inst_str += ", "; inst_str += $param_name.to_string().as_str() );+ )?
                        }),+
                    }
                    }

                    if inst_str.len() >= 2 {
                        write!(f, "{} {}", inst_name, &inst_str[2..])
                    } else {
                        write!(f, "{}", inst_name)
                    }
                }
            }

            paste!{$(
                fn [<run_ $name:lower _inst>] ($vm: &mut VM, $( $($param_name: $param_ty),+ )? ) -> Result<(), RunError> {
                    $body
                }
            )*}
        };
    }

    #[macro_export]
    macro_rules! bin_op {
        ($vm:ident, $dest:ident, $p0:ident $op:tt? $p1:ident) => {{
            let res = *$vm.get_reg($p0) $op *$vm.get_reg($p1);
            *$vm.get_reg_mut($dest) = (res?).into();
            Ok(())
        }};

        ($vm:ident, $dest:ident, $p0:ident $op:tt? v: $p1:ident) => {{
            let res = *$vm.get_reg($p0) $op ($p1);
            *$vm.get_reg_mut($dest) = (res?).into();
            Ok(())
        }};

        ($vm:ident, $dest:ident, $p0:ident $op:tt $p1:ident) => {{
            let res = *$vm.get_reg($p0) $op *$vm.get_reg($p1);
            *$vm.get_reg_mut($dest) = (res).into();
            Ok(())
        }};
        ($vm:ident, $dest:ident, $p0:ident $op:tt v: $p1:ident) => {{
            let res = *$vm.get_reg($p0) $op ($p1);
            *$vm.get_reg_mut($dest) = (res).into();
            Ok(())
        }};
    }

    #[macro_export]
    macro_rules! unary_op {
        ($vm:ident, $dest:ident, $op:tt? $p0:ident) => {{
            let res = $op(*$vm.get_reg($p0));
            *$vm.get_reg_mut($dest) = (res?).into();
            Ok(())
        }};
    }

    #[allow(unused_imports)]
    pub use bin_op;
    pub use insts;
    #[allow(unused_imports)]
    pub use unary_op;
}
