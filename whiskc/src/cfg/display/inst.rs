use crate::cfg::nodes::inst::{InstKind, TaggedInst};

use super::DisplayCFG;

impl DisplayCFG for TaggedInst {
    fn display<W: std::fmt::Write>(&self, w: &mut W, ctx: &mut super::DisplayContext) {
        write!(w, "    ").unwrap();

        if !matches!(
            self.inst.kind,
            InstKind::Store(_)
                | InstKind::Branch(_)
                | InstKind::BranchCond(_)
                | InstKind::Return(_)
        ) {
            let reg = ctx.get_reg_inst_id(self.id);
            let ty_str = self.inst.ty.to_string(ctx.sym_table);
            write!(w, "%{} = {} ", reg, ty_str).unwrap();
        }

        match &self.inst.kind {
            InstKind::Alloca => {
                write!(w, "alloca {}", self.inst.ty.to_string(ctx.sym_table)).unwrap()
            }
            InstKind::Load(inst) => {
                write!(w, "load ").unwrap();
                inst.value.display(w, ctx);
            }
            InstKind::Store(inst) => {
                write!(w, "store ").unwrap();
                inst.value.display(w, ctx);
                write!(w, ", ").unwrap();
                inst.target.display(w, ctx);
            }
            InstKind::Branch(inst) => {
                let bb_id = ctx.get_bb(inst.branch);
                write!(w, "branch bb_{}", bb_id).unwrap();
            }
            InstKind::BranchCond(inst) => {
                let then_id = ctx.get_bb(inst.then_branch);
                let else_id = ctx.get_bb(inst.else_branch);
                write!(w, "branch ").unwrap();
                inst.cond.display(w, ctx);
                write!(w, ", bb_{}, bb_{}", then_id, else_id).unwrap();
            }
            InstKind::Return(inst) => {
                write!(w, "ret ").unwrap();
                if let Some(value) = &inst.value {
                    value.display(w, ctx);
                } else {
                    write!(w, "()").unwrap();
                }
            }
            InstKind::Negate(inst) => {
                write!(w, "neg ").unwrap();
                inst.value.display(w, ctx);
            }
            InstKind::Not(inst) => {
                write!(w, "not ").unwrap();
                inst.value.display(w, ctx);
            }
            InstKind::Add(inst) => {
                write!(w, "add ").unwrap();
                inst.lhs.display(w, ctx);
                write!(w, ", ").unwrap();
                inst.rhs.display(w, ctx);
            }
            InstKind::Sub(inst) => {
                write!(w, "sub ").unwrap();
                inst.lhs.display(w, ctx);
                write!(w, ", ").unwrap();
                inst.rhs.display(w, ctx);
            }
            InstKind::And(inst) => {
                write!(w, "and ").unwrap();
                inst.lhs.display(w, ctx);
                write!(w, ", ").unwrap();
                inst.rhs.display(w, ctx);
            }
            InstKind::Or(inst) => {
                write!(w, "or ").unwrap();
                inst.lhs.display(w, ctx);
                write!(w, ", ").unwrap();
                inst.rhs.display(w, ctx);
            }
            InstKind::Compare(inst) => {
                write!(w, "cmp ").unwrap();
                inst.lhs.display(w, ctx);
                write!(w, " {} ", format!("{:?}", inst.cond).to_lowercase()).unwrap();
                inst.rhs.display(w, ctx);
            }
            InstKind::Call(inst) => {
                write!(w, "call ").unwrap();
                inst.callee.display(w, ctx);
                write!(w, "(").unwrap();
                for (i, arg) in inst.args.iter().enumerate() {
                    arg.display(w, ctx);
                    if i != inst.args.len() - 1 {
                        write!(w, ", ").unwrap();
                    }
                }
                write!(w, ")").unwrap();
            }
        };
        writeln!(w, "").unwrap()
    }
}
