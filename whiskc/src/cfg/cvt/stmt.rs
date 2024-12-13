use crate::{
    ast_resolved::nodes::stmt::{AssignStmt, Block, ExprStmt, IfStmt, LetStmt, ReturnStmt, Stmt},
    cfg::{
        builder::{BuildVisitor, Builder},
        nodes::{
            inst::{BranchCondInst, BranchInst, Inst, InstKind, ReturnInst, StoreInst},
            value::{InstValue, Value, ValueKind},
        },
        BuildContext,
    },
    symbol_table::Symbol,
    ty::PrimType,
};

impl BuildVisitor<()> for Stmt {
    fn visit(&self, ctx: &mut BuildContext, builder: &mut Builder) -> () {
        match self {
            Stmt::Block(stmt) => stmt.visit(ctx, builder),
            Stmt::Expr(stmt) => stmt.visit(ctx, builder),
            Stmt::Assign(stmt) => stmt.visit(ctx, builder),
            Stmt::Let(stmt) => stmt.visit(ctx, builder),
            Stmt::If(stmt) => stmt.visit(ctx, builder),
            Stmt::Return(stmt) => stmt.visit(ctx, builder),
        }
    }
}

impl BuildVisitor<()> for Block {
    fn visit(&self, ctx: &mut BuildContext, builder: &mut Builder) -> () {
        ctx.push_local(self.table_id);
        for stmt in &self.stmts {
            stmt.visit(ctx, builder);
        }
        ctx.pop_local();
    }
}

impl BuildVisitor<()> for ExprStmt {
    fn visit(&self, ctx: &mut BuildContext, builder: &mut Builder) -> () {
        self.expr.visit(ctx, builder);
    }
}

impl BuildVisitor<()> for AssignStmt {
    fn visit(&self, ctx: &mut BuildContext, builder: &mut Builder) -> () {
        let value = self.value.visit(ctx, builder);
        let target = self.target.visit(ctx, builder);
        builder.push_inst(Inst {
            kind: InstKind::Store(StoreInst { target, value }),
            ty: PrimType::Unit.into(),
        });
    }
}

impl BuildVisitor<()> for LetStmt {
    fn visit(&self, ctx: &mut BuildContext, builder: &mut Builder) -> () {
        let value = self.value.visit(ctx, builder);

        let Symbol::Variable(symbol) = ctx.get_symbol_by_name_mut(&self.name.0).unwrap() else {
            panic!("symbol is not a var symbol");
        };

        let alloca = builder.push_inst(Inst {
            kind: InstKind::Alloca,
            ty: self.ty,
        });

        let inst = Value {
            kind: ValueKind::Inst(InstValue {
                bb: *builder.get_current_block_id(),
                inst: alloca,
            }),
            ty: self.ty,
        };
        symbol.set_value(inst);

        builder.push_inst(Inst {
            kind: InstKind::Store(StoreInst {
                target: inst,
                value,
            }),
            ty: self.ty,
        });
    }
}

impl BuildVisitor<()> for IfStmt {
    fn visit(&self, ctx: &mut BuildContext, builder: &mut Builder) -> () {
        let cond = self.cond.visit(ctx, builder);

        let then_branch = builder.add_block("then".to_owned().into());
        let merge_branch = builder.add_block("merge".to_owned().into());
        let else_branch = if self.else_body.is_some() {
            builder.add_block("else".to_owned().into())
        } else {
            merge_branch
        };
        builder.push_inst(Inst {
            kind: InstKind::BranchCond(BranchCondInst {
                cond,
                then_branch,
                else_branch,
            }),
            ty: PrimType::Unit.into(),
        });

        builder.set_current_block(then_branch);
        self.body.visit(ctx, builder);
        builder.push_inst(Inst {
            kind: InstKind::Branch(BranchInst {
                branch: merge_branch,
            }),
            ty: PrimType::Unit.into(),
        });

        if let Some(else_block) = &self.else_body {
            builder.set_current_block(else_branch);
            else_block.visit(ctx, builder);
            builder.push_inst(Inst {
                kind: InstKind::Branch(BranchInst {
                    branch: merge_branch,
                }),
                ty: PrimType::Unit.into(),
            });
        }

        builder.set_current_block(merge_branch);
    }
}

impl BuildVisitor<()> for ReturnStmt {
    fn visit(&self, ctx: &mut BuildContext, builder: &mut Builder) -> () {
        let value = self.expr.as_ref().map(|v| v.visit(ctx, builder));
        builder.push_inst(Inst {
            kind: InstKind::Return(ReturnInst { value }),
            ty: self
                .expr
                .as_ref()
                .map(|v| v.ty)
                .unwrap_or(PrimType::Unit.into()),
        });
    }
}
