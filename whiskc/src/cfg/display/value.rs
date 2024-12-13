use crate::{
    cfg::nodes::value::{ConstantValue, Value, ValueKind},
    symbol_table::Symbol,
};

use super::DisplayCFG;

impl DisplayCFG for Value {
    fn display<W: std::fmt::Write>(&self, w: &mut W, ctx: &mut super::DisplayContext) {
        if !matches!(self.kind, ValueKind::Function(_)) {
            write!(w, "{} ", self.ty.to_string(ctx.sym_table)).unwrap();
        }

        match &self.kind {
            ValueKind::Constant(value) => match value {
                ConstantValue::Bool(v) => write!(w, "{}", if *v { "true" } else { "false" }),
                ConstantValue::Integer(v) => write!(w, "{}", v),
            }
            .unwrap(),
            ValueKind::Inst(value) => {
                let reg = ctx.get_reg_inst_id(value.inst);
                write!(w, "%{}", reg).unwrap();
            }
            ValueKind::Parameter(value) => {
                let reg = ctx.get_reg_sym_id(*value);
                write!(w, "%{}", reg).unwrap();
            }
            ValueKind::Function(func_sym_id) => {
                let Symbol::Function(func_sym) = ctx.sym_table.get_symbol(*func_sym_id).unwrap()
                else {
                    panic!("symbol is not a function");
                };
                write!(w, "{}", func_sym.get_name()).unwrap();
            }
        }
    }
}
