use wsk_vm::{Cmp, Inst};

use crate::{
    ast::parsing::token::Operator,
    ast_resolved::nodes::expr::{
        BinaryExpr, BlockExpr, CallExpr, Expr, IdentExpr, IfExpr, LoopExpr, ReturnExpr, UnaryExpr,
    },
    ty::Type,
};

use super::{Codegen, CodegenError, Context};

impl Codegen for Expr {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError> {
        match self {
            Expr::Unit => todo!(),
            Expr::Integer(v) => v.codegen(ctx),
            Expr::Bool(v) => v.codegen(ctx),
            Expr::Identifier(v) => v.codegen(ctx),
            Expr::Unary(v) => v.codegen(ctx),
            Expr::Binary(v) => v.codegen(ctx),
            Expr::Call(v) => v.codegen(ctx),
            Expr::Block(v) => v.codegen(ctx),
            Expr::Return(_) => todo!(),
            Expr::If(_) => todo!(),
            Expr::Loop(_) => todo!(),
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

        if let Type::Function(func_ty) = self.callee.get_type() {
            let fi = ctx.get_fi(func_ty.0).expect("codegen fi");
            ctx.get_current_fi_mut().push_inst(Inst::Call(fi));
        } else {
            unimplemented!("unsupported function call type")
        }

        Ok(())
    }
}

impl ExprCodegen for BlockExpr {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError> {
        for stmt in &self.stmts {
            stmt.codegen(ctx)?;
        }

        if let Some(expr) = &self.eval_expr {
            expr.codegen(ctx)?;
        }

        Ok(())
    }
}

impl ExprCodegen for ReturnExpr {
    fn codegen(&self, _ctx: &mut Context) -> Result<(), CodegenError> {
        todo!()
    }
}

impl ExprCodegen for IfExpr {
    fn codegen(&self, _ctx: &mut Context) -> Result<(), CodegenError> {
        todo!()
    }
}

impl ExprCodegen for LoopExpr {
    fn codegen(&self, _ctx: &mut Context) -> Result<(), CodegenError> {
        todo!()
    }
}

pub trait ExprCodegen {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError>;
}
