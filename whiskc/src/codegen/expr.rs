use wsk_vm::{Cmp, Inst};

use crate::{
    ast::parsing::token::Operator,
    ast_resolved::nodes::{
        expr::{
            BinaryExpr, BlockExpr, CallExpr, Expr, IdentExpr, IfExpr, LoopExpr, ReturnExpr,
            UnaryExpr,
        },
        stmt::{ExprStmt, Stmt},
        ty::Type,
    },
};

use super::{Codegen, CodegenError, Context};

impl Codegen for Expr {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError> {
        match self {
            Expr::Unit => Ok(()),
            Expr::Integer(v) => v.codegen(ctx),
            Expr::Bool(v) => v.codegen(ctx),
            Expr::Identifier(v) => v.codegen(ctx),
            Expr::Unary(v) => v.codegen(ctx),
            Expr::Binary(v) => v.codegen(ctx),
            Expr::Call(v) => v.codegen(ctx),
            Expr::Block(v) => v.codegen(ctx),
            Expr::Return(v) => v.codegen(ctx),
            Expr::If(v) => v.codegen(ctx),
            Expr::Loop(v) => v.codegen(ctx),
        }
    }
}

impl ExprCodegen for i64 {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError> {
        ctx.get_current_fi_mut()
            .push_inst(Inst::Push((*self).into()));
        Ok(())
    }
}

impl ExprCodegen for bool {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError> {
        ctx.get_current_fi_mut()
            .push_inst(Inst::Push((*self).into()));
        Ok(())
    }
}

impl ExprCodegen for IdentExpr {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError> {
        if ctx.get_fi(self.sym_id).is_some() {
            unimplemented!("function ref codegen")
        }
        let id = ctx.get_local(self.sym_id);
        ctx.get_current_fi_mut().push_inst(Inst::Load(id));
        Ok(())
    }
}

impl ExprCodegen for UnaryExpr {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError> {
        self.expr.codegen(ctx)?;

        let func = ctx.get_current_fi_mut();
        func.push_inst(match self.op {
            Operator::Sub => Inst::Neg,
            Operator::Not => Inst::Not,
            _ => unimplemented!("codegen unary op {}", self.op),
        });

        Ok(())
    }
}

impl ExprCodegen for BinaryExpr {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError> {
        if self.op == Operator::Assign {
            self.right.codegen(ctx)?;

            // dont evaluate identifier
            // self.target.codegen(ctx)?;

            return if let Expr::Identifier(IdentExpr { sym_id, .. }) = self.left.as_ref() {
                let id = ctx.get_local(*sym_id);
                ctx.get_current_fi_mut().push_inst(Inst::Store(id));
                Ok(())
            } else {
                unimplemented!("unsupported assignment type")
            };
        }

        self.left.codegen(ctx)?;
        self.right.codegen(ctx)?;

        let func = ctx.get_current_fi_mut();
        match self.op {
            Operator::Add => func.push_inst(Inst::Add),
            Operator::Sub => func.push_inst(Inst::Sub),
            Operator::And => func.push_inst(Inst::And),
            Operator::Or => func.push_inst(Inst::Or),
            Operator::Equal => func.push_inst(Cmp::Equal),
            Operator::NotEqual => func.push_insts([Cmp::Equal.into(), Inst::Not]),
            Operator::Less => func.push_inst(Cmp::Less),
            Operator::LessEqual => func.push_insts([Cmp::Greater.into(), Inst::Not]),
            Operator::Greater => func.push_inst(Cmp::Greater),
            Operator::GreaterEqual => func.push_insts([Cmp::Less.into(), Inst::Not]),
            _ => unimplemented!("codegen binary op {}", self.op),
        };

        Ok(())
    }
}

impl ExprCodegen for CallExpr {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError> {
        self.callee.codegen(ctx)?;

        for arg in &self.args {
            arg.codegen(ctx)?;
        }

        if let Type::Func(func_ty) = self.callee.get_type() {
            let fi = ctx.get_fi(func_ty).expect("codegen fi");
            ctx.get_current_fi_mut().push_inst(Inst::Call(fi));
        } else {
            unimplemented!("unsupported function call type")
        }

        Ok(())
    }
}

impl ExprCodegen for BlockExpr {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError> {
        ctx.push_bound();

        for stmt in &self.stmts {
            stmt.codegen(ctx)?;
        }

        if let Some(expr) = &self.eval_expr {
            expr.codegen(ctx)?;
        }

        ctx.pop_bound();
        Ok(())
    }
}

impl ExprCodegen for ReturnExpr {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError> {
        if let Some(expr) = &self.expr {
            expr.codegen(ctx)?;
        }
        ctx.get_current_fi_mut().push_inst(Inst::Ret);
        Ok(())
    }
}

impl ExprCodegen for IfExpr {
    fn codegen(&self, ctx: &mut super::Context) -> Result<(), super::CodegenError> {
        self.cond.codegen(ctx)?;

        let branch_point = ctx.get_current_fi_mut().len();

        self.then.codegen(ctx)?;

        let then_to_merge_point = ctx.get_current_fi_mut().len() + 1;

        if let Some(body) = &self.else_ {
            body.codegen(ctx)?;
        }

        let func = ctx.get_current_fi_mut();

        let jmp_dist = func.len() - branch_point + 1;
        func.insert_inst(branch_point, Inst::JmpFalse(jmp_dist as isize));

        if !matches!(
            self.then.stmts.last(),
            Some(Stmt::Expr(ExprStmt {
                expr: Expr::Return(_)
            }))
        ) && self.else_.is_some()
        {
            let jmp_dist = func.len() - then_to_merge_point + 1;
            func.insert_inst(then_to_merge_point, Inst::Jmp(jmp_dist as isize));
        }

        Ok(())
    }
}

impl ExprCodegen for LoopExpr {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError> {
        let jmp_dest = ctx.get_current_fi_mut().len();
        self.body.codegen(ctx)?;
        let func = ctx.get_current_fi_mut();
        let jmp_src = func.len();
        func.push_inst(Inst::Jmp(jmp_dest as isize - jmp_src as isize));
        Ok(())
    }
}

pub trait ExprCodegen {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError>;
}
