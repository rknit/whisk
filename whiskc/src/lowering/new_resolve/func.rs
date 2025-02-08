use super::{Record, Resolve, ResolveContext};

use crate::{
    ast::nodes as ast,
    lowering::nodes::{
        expr::BlockExpr,
        func::{ExternFunction, Function},
    },
    symbol::{FuncId, Param},
};

impl Record for ast::func::FunctionSig {
    fn record(&self, ctx: &mut ResolveContext, _: ()) {
        if ctx.table.new_function(self.name.0.clone()).is_none() {
            todo!("report error");
        };
    }
}

impl Resolve<(), Option<FuncId>> for ast::func::FunctionSig {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> Option<FuncId> {
        let mut params: Vec<Param> = Vec::new();
        for ast_param in &self.params.items {
            if params.iter().any(|v| v.name == ast_param.0 .0) {
                params.push(Param::default());
                todo!("report error");
            }
            let Some(param_ty) = ast_param.1.resolve(ctx, ()) else {
                // assumed an error is reported by the resolve call.
                continue;
            };
            params.push(Param {
                name: ast_param.0 .0.clone(),
                ty: param_ty,
            })
        }

        let Some(ret_ty) = self.ret_ty.resolve(ctx, ()) else {
            // assumed an error is reported by the resolve call.
            return None;
        };

        let mut sym = ctx.table.get_function_by_name_mut(&self.name.0).unwrap();
        sym.set_params(params).set_return_type(ret_ty);
        Some(sym.get_id())
    }
}

impl Resolve<(), Option<Function>> for ast::func::Function {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> Option<Function> {
        let fid = self.sig.resolve(ctx, ())?;

        Some(Function {
            func_id: fid,
            body: BlockExpr {
                block_id: Default::default(),
                stmts: vec![],
                eval_expr: None,
            },
        })
    }
}

impl Resolve<(), Option<ExternFunction>> for ast::func::ExternFunction {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> Option<ExternFunction> {
        let fid = self.sig.resolve(ctx, ())?;
        Some(ExternFunction(fid))
    }
}
