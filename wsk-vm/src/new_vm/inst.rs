use crate::{value::OpError, Value};

use super::{
    abi::Register,
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
    PUSH: 0x01 => (vm, reg: Register) {
        let v = *vm.get_reg(reg);
        vm.push_value(v)?;
        Ok(())
    },
    POP: 0x02 => (vm, dest: Register) {
        let v = vm.pop_value()?;
        *vm.get_reg_mut(dest) = v;
        Ok(())
    },
    MOV: 0x03 => (vm, dest: Register, org: Register) {
        *vm.get_reg_mut(dest) = *vm.get_reg(org);
        Ok(())
    },
    MOVV: 0x04 => (vm, dest: Register, value: Value) {
        *vm.get_reg_mut(dest) = value;
        Ok(())
    },

    /*
    *   binary operation insts
    */

    ADD: 0x10 => (vm, dest: Register, p0: Register, p1: Register) {
        let res = *vm.get_reg(p0) + *vm.get_reg(p1);
        *vm.get_reg_mut(dest) = res?;
        Ok(())
    },
    SUB: 0x11 => (vm, dest: Register, p0: Register, p1: Register) {
        let res = *vm.get_reg(p0) - *vm.get_reg(p1);
        *vm.get_reg_mut(dest) = res?;
        Ok(())
    },
    MUL: 0x12 => (vm, dest: Register, p0: Register, p1: Register) {
        let res = *vm.get_reg(p0) * *vm.get_reg(p1);
        *vm.get_reg_mut(dest) = res?;
        Ok(())
    },
    DIV: 0x13 => (vm, dest: Register, p0: Register, p1: Register) {
        let res = *vm.get_reg(p0) / *vm.get_reg(p1);
        *vm.get_reg_mut(dest) = res?;
        Ok(())
    },

    AND: 0x14 => (vm, dest: Register, p0: Register, p1: Register) {
        let res = *vm.get_reg(p0) & *vm.get_reg(p1);
        *vm.get_reg_mut(dest) = res?;
        Ok(())
    },
    OR: 0x15 => (vm, dest: Register, p0: Register, p1: Register) {
        let res = *vm.get_reg(p0) | *vm.get_reg(p1);
        *vm.get_reg_mut(dest) = res?;
        Ok(())
    },
    CMPEQ: 0x16 => (vm, dest: Register, p0: Register, p1: Register) {
        let res = *vm.get_reg(p0) == *vm.get_reg(p1);
        *vm.get_reg_mut(dest) = res.into();
        Ok(())
    },
    CMPNE: 0x17 => (vm, dest: Register, p0: Register, p1: Register) {
        let res = *vm.get_reg(p0) != *vm.get_reg(p1);
        *vm.get_reg_mut(dest) = res.into();
        Ok(())
    },
    CMPLT: 0x18 => (vm, dest: Register, p0: Register, p1: Register) {
        let res = *vm.get_reg(p0) < *vm.get_reg(p1);
        *vm.get_reg_mut(dest) = res.into();
        Ok(())
    },
    CMPGT: 0x19 => (vm, dest: Register, p0: Register, p1: Register) {
        let res = *vm.get_reg(p0) > *vm.get_reg(p1);
        *vm.get_reg_mut(dest) = res.into();
        Ok(())
    },

    /*
    *   unary operation insts
    */

    NOT: 0x20 => (vm, dest: Register, p0: Register) {
        let res = !(*vm.get_reg(p0));
        *vm.get_reg_mut(dest) = res?;
        Ok(())
    },
    NEG: 0x21 => (vm, dest: Register, p0: Register) {
        let res = -(*vm.get_reg(p0));
        *vm.get_reg_mut(dest) = res?;
        Ok(())
    },

    /*
    *   control flow insts
    */

    JMP: 0x30 => (vm, offset: isize) {
        vm.jump(offset);
        Ok(())
    },
    JTR: 0x31 => (vm, reg: Register, offset: isize) {
        let Value::Bool(v) = vm.get_reg(reg) else {
            return Err(OpError::InvalidTypeForOp.into());
        };
        if *v {
            vm.jump(offset);
        }
        Ok(())
    },
    JFL: 0x32 => (vm, reg: Register, offset: isize) {
        let Value::Bool(v) = vm.get_reg(reg) else {
            return Err(OpError::InvalidTypeForOp.into());
        };
        if !(*v) {
            vm.jump(offset);
        }
        Ok(())
    },
    CALL: 0x33 => (vm, fi: usize) {
        vm.call(fi);
        Ok(())
    },
    RET: 0x34 => (vm) {
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
            $name:ident : $code:literal => ( $vm:ident $(,)? $( $($param_name:ident: $param_ty:ty),+ )? ) $body:block
        ),+ $(,)?) => {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            #[repr(u8)]
            pub enum Inst {
                $($name $({ $($param_name: $param_ty),+ } )? = $code),+
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
                fn [<run_ $name:lower _inst>] ($vm: &mut VM, $( $($param_name: $param_ty),+ )? ) -> Result<(), RunError> $body
            )*}
        };
    }

    pub use insts;
}
