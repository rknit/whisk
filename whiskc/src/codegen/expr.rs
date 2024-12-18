use wsk_vm::{Cmp, Inst};

use crate::{
    ast::parsing::token::Operator,
    ast_resolved::nodes::expr::{BinaryExpr, CallExpr, Expr, ExprKind, IdentExpr, UnaryExpr},
    ty::Type,
};

use super::{Codegen, CodegenError, Context};

impl Codegen for Expr {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError> {
        match self.get_kind() {
            ExprKind::Integer(v) => v.codegen(ctx),
            ExprKind::Bool(v) => v.codegen(ctx),
            ExprKind::Identifier(v) => v.codegen(ctx),
            ExprKind::Unary(v) => v.codegen(ctx),
            ExprKind::Binary(v) => v.codegen(ctx),
            ExprKind::Call(v) => v.codegen(ctx),
        }
    }
}

impl ExprCodegen for i64 {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError> {
        ctx.get_current_fi_mut()
            .push_inst(Inst::Push((*self).into()));
        ctx.push_local(None);
        Ok(())
    }
}

impl ExprCodegen for bool {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError> {
        ctx.get_current_fi_mut()
            .push_inst(Inst::Push((*self).into()));
        ctx.push_local(None);
        Ok(())
    }
}

impl ExprCodegen for IdentExpr {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError> {
        if ctx.get_fi(self.sym_id).is_some() {
            return Ok(());
        }
        let offset = ctx.get_local(self.sym_id).expect("valid offset");
        ctx.get_current_fi_mut().push_inst(Inst::Load(offset));
        ctx.push_local(None);
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

        ctx.pop_local();

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

trait ExprCodegen {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError>;
}
