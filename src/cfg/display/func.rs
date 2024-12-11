use std::fmt::Write;

use crate::{cfg::nodes::function::Function, symbol_table::Symbol};

use super::DisplayCFG;

impl DisplayCFG for Function {
    fn display<W: Write>(&self, w: &mut W, ctx: &mut super::DisplayContext) {
        let Symbol::Function(func_sym) = ctx.sym_table.get_symbol(self.get_symbol_id()).unwrap()
        else {
            panic!("symbol is not a function");
        };

        let mut params = Vec::new();
        for param in func_sym.get_params() {
            let Symbol::Variable(var_sym) = ctx
                .sym_table
                .get_table(self.get_table_id())
                .unwrap()
                .get_symbol_by_name(&param.0 .0)
                .unwrap()
            else {
                panic!("symbol is not a variable");
            };
            let reg = ctx.get_reg_sym_id(var_sym.get_id());
            params.push((reg, param.1));
        }

        if !self.has_entry() {
            write!(w, "extern ").unwrap();
        }

        write!(
            w,
            "func {}({}) {}",
            func_sym.get_name(),
            params
                .iter()
                .map(|(reg, ty)| format!("{} %{}", ty.to_string(ctx.sym_table), reg))
                .collect::<Vec<String>>()
                .join(", "),
            func_sym.get_return_type().to_string(ctx.sym_table)
        )
        .unwrap();

        if !self.has_entry() {
            writeln!(w, "").unwrap();
            return;
        } else {
            writeln!(w, " {{").unwrap();
        }

        let entry_id = self.get_entry_block();
        let entry = self.get_block(entry_id).unwrap();
        (entry_id, entry).display(w, ctx);

        for (bb_id, bb) in self.get_blocks() {
            if *bb_id == entry_id {
                continue;
            }
            (*bb_id, bb).display(w, ctx);
        }

        writeln!(w, "}}").unwrap();
    }
}
