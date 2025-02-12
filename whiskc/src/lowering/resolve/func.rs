use super::{Flow, FlowObj, Record, Resolve, ResolveContext};

use crate::{
    ast::nodes as ast,
    lowering::nodes::{
        expr::ExprKind,
        func::{ExternFunction, Function},
    },
    symbol::{BlockId, FuncId, VarId},
};

impl Record for ast::func::FunctionSig {
    fn record(&self, ctx: &mut ResolveContext, _: ()) {
        if ctx.table.new_function(self.name.0.clone()).is_none() {
            todo!("report error");
        };
    }
}

impl Resolve<(), Option<(FuncId, BlockId)>> for ast::func::FunctionSig {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> Option<(FuncId, BlockId)> {
        let fid = ctx.table.get_function_id(&self.name.0).unwrap();
        let bid = ctx.table.new_block(fid);

        let mut params: Vec<VarId> = Vec::new();
        for ast_param in &self.params.items {
            let Some(param_ty) = ast_param.1.resolve(ctx, ()) else {
                // assumed an error is reported by the resolve call.
                continue;
            };
            let Some(param_id) = ctx.table.new_variable(ast_param.0 .0.clone(), bid) else {
                todo!("report error")
            };
            param_id.sym_mut(ctx.table).ty = param_ty;
            params.push(param_id)
        }

        let Some(ret_ty) = self.ret_ty.resolve(ctx, ()) else {
            // assumed an error is reported by the resolve call.
            return None;
        };

        let sym = fid.sym_mut(ctx.table);
        sym.params = params;
        sym.ret_ty = ret_ty;
        sym.entry_block = bid;
        Some((fid, bid))
    }
}

impl Resolve<(), Option<Function>> for ast::func::Function {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> Option<Function> {
        let (fid, bid) = self.sig.resolve(ctx, ())?;
        ctx.set_func_id(fid);
        ctx.push_block(bid);

        let FlowObj { value: body, flow } = self.body.resolve(ctx, ());
        let ExprKind::Block(body) = body?.kind else {
            unreachable!()
        };

        let ret_ty = body
            .eval_expr
            .as_ref()
            .map(|v| v.ty)
            .unwrap_or(ctx.table.common_type().unit);
        let expect_ret_ty = fid.sym(ctx.table).ret_ty;
        if flow != Flow::Break && !ctx.table.is_type_coercible(ret_ty, expect_ret_ty) {
            todo!("report error");
        }

        ctx.pop_block();
        ctx.unset_func_id();
        Some(Function { func_id: fid, body })
    }
}

impl Resolve<(), Option<ExternFunction>> for ast::func::ExternFunction {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> Option<ExternFunction> {
        let (fid, _) = self.sig.resolve(ctx, ())?;
        Some(ExternFunction(fid))
    }
}
