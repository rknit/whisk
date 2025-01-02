use crate::{
    ast::{
        self,
        location::{Locatable, Located},
    },
    ast_resolved::{
        errors::{ControlFlowError, IdentResolveError, TypeResolveError},
        nodes::{
            expr::Expr,
            func::{ExternFunction, Function, FunctionSig, Param},
            ty::Type,
        },
        ControlFlow, Resolve, ResolveContext,
    },
    symbol_table::{FuncSymbol, Symbol, SymbolAttribute, SymbolID, SymbolKind, VarSymbol},
};

use super::expr::{ExprFlow, ExprResolve};

impl Resolve<Function> for ast::nodes::func::Function {
    fn resolve(&self, ctx: &mut ResolveContext) -> Option<Function> {
        let func_sym_id = ctx
            .get_symbol_id_by_name(&self.sig.name.0)
            .expect("resolved function signature id");

        ctx.set_func_symbol_id(func_sym_id);
        let table_id = ctx.push_local();

        let mut params = Vec::new();
        for crate::ast::nodes::func::Param(param_name, param_ty) in &self.sig.params.items {
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
            params.push(Param {
                sym_id: sym_id.unwrap_or_else(SymbolID::nil),
                name: param_name.0.clone(),
            });
        }

        let ExprFlow(body, mut flow) = self.body.resolve(ctx);

        let Expr::Block(body) = body? else {
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

        ctx.pop_local();
        ctx.unset_func_symbol_id();

        Some(Function {
            table_id,
            sig: FunctionSig {
                sym_id: func_sym_id,
                name: self.sig.name.clone(),
                params,
                ret_ty,
            },
            body,
        })
    }
}

impl Resolve<ExternFunction> for ast::nodes::func::ExternFunction {
    fn resolve(&self, ctx: &mut ResolveContext) -> Option<ExternFunction> {
        let sym_id = ctx
            .get_symbol_id_by_name(&self.sig.name.0)
            .expect("resolved function signature id");
        let ret_ty = self.sig.ret_ty.resolve(ctx)?;
        Some(ExternFunction(FunctionSig {
            sym_id,
            name: self.sig.name.clone(),
            params: self
                .sig
                .params
                .items
                .iter()
                .map(|p| Param {
                    sym_id: SymbolID::nil(),
                    name: p.0 .0.clone(),
                })
                .collect(),
            ret_ty,
        }))
    }
}

impl Resolve<()> for ast::nodes::func::FunctionSig {
    fn resolve(&self, ctx: &mut ResolveContext) -> Option<()> {
        let mut params = Vec::new();
        for param in &self.params.items {
            let ty = param.1.resolve(ctx)?;
            params.push((param.0.clone(), ty));
        }

        let ret_ty = self.ret_ty.resolve(ctx)?;
        let mut func_sym = FuncSymbol::new(self.name.clone(), params, ret_ty);

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
