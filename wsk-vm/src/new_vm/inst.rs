use crate::value::OpError;

use super::{
    abi::Register,
    vm::{VMError, VM},
};
use paste::paste;

macro_utils::insts!(
    HLT: 0 => (vm) {
        vm.halt();
        Ok(())
    },
    ADD: 1 => (vm, dest: Register, p0: Register, p1: Register) {
        let res = *vm.get_reg(p0) + *vm.get_reg(p1);
        *vm.get_reg_mut(dest) = res?;
        Ok(())
    },
);

#[derive(Debug)]
pub enum InstError {
    VMError(VMError),
    OpError(OpError),
}

impl From<VMError> for InstError {
    fn from(value: VMError) -> Self {
        Self::VMError(value)
    }
}
impl From<OpError> for InstError {
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
            pub enum Inst {
                $($name $({ $($param_name: $param_ty),+ } )?),+
            }

            impl Inst {
                pub fn run(self, vm: &mut VM) -> Result<(), InstError> {
                    paste!{
                    match self {
                        $(Inst::$name $({ $( $param_name ),+ })? => [<run_ $name:lower _inst>] (vm, $( $($param_name),+ )? ) ),+
                    }
                    }
                }
            }

            paste!{$(
                fn [<run_ $name:lower _inst>] ($vm: &mut VM, $( $($param_name: $param_ty),+ )? ) -> Result<(), InstError> $body
            )*}
        };
    }

    pub use insts;
}
