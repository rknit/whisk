use crate::{
    ast::{
        self,
        location::{Locatable, Located},
    },
    lowering::{
        errors::{ControlFlowError, IdentResolveError, TypeResolveError},
        nodes::{
            expr::Expr,
            func::{ExternFunction, Function},
            ty::Type,
        },
    },
    old_symbol_table::{FuncSymbol, Symbol, SymbolAttribute, SymbolID, SymbolKind, VarSymbol},
};

use super::{
    expr::{ExprFlow, ExprResolve},
    ControlFlow, Resolve, ResolveContext,
};

impl Resolve<Function> for ast::nodes::func::Function {
    fn resolve(&self, ctx: &mut ResolveContext) -> Option<Function> {
        let func_sym_id = ctx
            .get_symbol_id_by_name(&self.sig.name.0)
            .expect("resolved function signature id");
        ctx.set_func_symbol_id(func_sym_id);

        let ExprFlow(body, mut flow) = self.body.resolve(ctx);

        let Some(body) = body else {
            ctx.unset_func_symbol_id();
            return None;
        };
        let Expr::Block(body) = body else {
            unreachable!()
        };

        let ret_ty = {
            let Some(Symbol::Function(func_sym)) = ctx.get_symbol(func_sym_id) else {
                unreachable!();
            };
            func_sym.get_return_type()
        };

        if let Some(eval_expr) = &body.eval_expr {
            flow = ControlFlow::Return;

            if eval_expr.get_type() != ret_ty {
                ctx.push_error(
                    TypeResolveError::ReturnTypeMismatch {
                        function_name: self.sig.name.0.clone(),
                        expected_type: ret_ty,
                        actual_type: Located(
                            eval_expr.get_type(),
                            self.body
                                .eval_expr
                                .as_ref()
                                .unwrap()
                                .as_ref()
                                .get_location(),
                        ),
                    }
                    .into(),
                );
            }
        }

        if flow != ControlFlow::Return && ret_ty != Type::Unit {
            ctx.push_error(ControlFlowError::NotAllFuncPathReturned(self.sig.name.clone()).into());
        }

        ctx.unset_func_symbol_id();
        Some(Function {
            sym_id: func_sym_id,
            func_id: Default::default(),
            body,
        })
    }
}

impl Resolve<ExternFunction> for ast::nodes::func::ExternFunction {
    fn resolve(&self, ctx: &mut ResolveContext) -> Option<ExternFunction> {
        let sym_id = ctx
            .get_symbol_id_by_name(&self.sig.name.0)
            .expect("resolved function signature id");
        Some(ExternFunction(sym_id, Default::default()))
    }
}

impl Resolve<()> for ast::nodes::func::FunctionSig {
    fn resolve(&self, ctx: &mut ResolveContext) -> Option<()> {
        let table_id = ctx.push_local();

        let mut params = Vec::new();
        for crate::ast::nodes::func::Param(param_name, param_ty) in &self.params.items {
            let Some(param_ty) = param_ty.resolve(ctx) else {
                continue;
            };
            let symbol = VarSymbol::new(param_name.clone(), param_ty);
            let sym_id = ctx.new_symbol(&param_name.0, symbol.into());
            if sym_id.is_none() {
                let first_origin = {
                    let dup_symbol = ctx
                        .get_current_table_mut()
                        .get_symbol_by_name(&param_name.0)
                        .unwrap();
                    (dup_symbol.get_type(), dup_symbol.get_origin())
                };
                ctx.push_error(
                    IdentResolveError::VarNameAlreadyUsed {
                        ident: param_name.0.clone(),
                        first_origin,
                        dup_origin: (param_ty, param_name.1),
                    }
                    .into(),
                );
            };
            params.push(sym_id.unwrap_or_else(SymbolID::nil));
        }

        ctx.pop_local();

        let ret_ty = self.ret_ty.resolve(ctx)?;
        let mut func_sym = FuncSymbol::new(table_id, self.name.clone(), params, ret_ty);

        let attributes = self.attributes.resolve(ctx, &[SymbolAttribute::Public]);
        func_sym.add_attributes(attributes);

        if ctx.new_symbol(&self.name.0, func_sym.into()).is_none() {
            let first_origin = {
                let dup_symbol = ctx
                    .get_current_table_mut()
                    .get_symbol_by_name(&self.name.0)
                    .unwrap();
                (dup_symbol.get_kind(), dup_symbol.get_origin())
            };
            ctx.push_error(
                IdentResolveError::GlobalIdentAlreadyUsed {
                    ident: self.name.0.clone(),
                    first_origin,
                    dup_origin: (SymbolKind::Function, self.name.1),
                }
                .into(),
            );
            return None;
        };

        Some(())
    }
}
