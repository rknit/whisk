use crate::cfg::nodes::basic_block::{BasicBlock, BasicBlockID};

use super::DisplayCFG;

impl DisplayCFG for (BasicBlockID, &BasicBlock) {
    fn display<W: std::fmt::Write>(&self, w: &mut W, ctx: &mut super::DisplayContext) {
        let bb_id = ctx.get_bb(self.0);

        write!(w, "  bb_{}:", bb_id,).unwrap();
        if let Some(desc) = &self.1.desc {
            writeln!(w, " // {}", desc).unwrap();
        } else {
            writeln!(w, "").unwrap();
        }

        for inst in self.1.get_insts() {
            inst.display(w, ctx);
        }

        writeln!(w, "").unwrap();
    }
}
