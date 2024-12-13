use crate::{
    ast::{self, location::Located, nodes::func::LocatedParam},
    ast_resolved::{
        errors::{ControlFlowError, IdentResolveError},
        nodes::func::{ExternFunction, Function, FunctionSig},
        ControlFlow, Resolve, ResolveContext,
    },
    symbol_table::{FuncSymbol, SymbolAttribute, SymbolKind, VarSymbol},
    ty::PrimType,
};

impl Resolve<Function> for ast::nodes::func::Function {
    fn resolve(&self, ctx: &mut ResolveContext) -> Option<Function> {
        let func_sym_id = ctx
            .get_symbol_id_by_name(&self.sig.name.0)
            .expect("resolved function signature id");

        ctx.set_func_symbol_id(func_sym_id);
        let table_id = ctx.push_local();

        for LocatedParam(param_name, Located(param_ty, _)) in &self.sig.params.items {
            let symbol = VarSymbol::new(param_name.clone(), *param_ty);
            if ctx.new_symbol(&param_name.0, symbol.into()).is_none() {
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
                        dup_origin: (*param_ty, param_name.1),
                    }
                    .into(),
                );
            }
        }

        let (body, flow) = self.body.resolve(ctx).unwrap();
        if flow != ControlFlow::Return && self.sig.ret_ty.0 != PrimType::Unit.into() {
            ctx.push_error(ControlFlowError::NotAllFuncPathReturned(self.sig.name.clone()).into());
        }

        ctx.pop_local();
        ctx.unset_func_symbol_id();

        Some(Function {
            table_id,
            sig: FunctionSig {
                sym_id: func_sym_id,
                name: self.sig.name.clone(),
                params: self
                    .sig
                    .params
                    .items
                    .iter()
                    .map(|v| (v.0.clone(), v.1 .0))
                    .collect(),
                ret_ty: self.sig.ret_ty.0,
            },
            body: body?,
        })
    }
}

impl Resolve<ExternFunction> for ast::nodes::func::ExternFunction {
    fn resolve(&self, ctx: &mut ResolveContext) -> Option<ExternFunction> {
        let sym_id = ctx
            .get_symbol_id_by_name(&self.sig.name.0)
            .expect("resolved function signature id");
        Some(ExternFunction(FunctionSig {
            sym_id,
            name: self.sig.name.clone(),
            params: self
                .sig
                .params
                .items
                .iter()
                .map(|v| (v.0.clone(), v.1 .0))
                .collect(),
            ret_ty: self.sig.ret_ty.0,
        }))
    }
}

impl Resolve<FunctionSig> for ast::nodes::func::FunctionSig {
    fn resolve(&self, ctx: &mut ResolveContext) -> Option<FunctionSig> {
        let params = self
            .params
            .items
            .iter()
            .map(|v| (v.0.clone(), v.1 .0))
            .collect::<Vec<_>>();

        let mut func_sym = FuncSymbol::new(self.name.clone(), params.clone(), self.ret_ty.0);

        let attributes = self.attributes.resolve(ctx, &[SymbolAttribute::Public]);
        func_sym.add_attributes(attributes);

        let Some(sym_id) = ctx.new_symbol(&self.name.0, func_sym.into()) else {
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

        Some(FunctionSig {
            sym_id,
            name: self.name.clone(),
            params,
            ret_ty: self.ret_ty.0,
        })
    }
}
