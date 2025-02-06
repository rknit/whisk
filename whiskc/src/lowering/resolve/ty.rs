use std::collections::HashMap;

use crate::ast::location::Located;
use crate::lowering::nodes::ty::Type;

use crate::ast::nodes::ty as ast_ty;

use super::{Resolve, ResolveContext};

impl Resolve<Type> for ast_ty::Type {
    fn resolve(&self, ctx: &mut ResolveContext) -> Option<Type> {
        match self {
            ast_ty::Type::Primitive(v) => v.resolve(ctx),
            ast_ty::Type::Struct(v) => v.resolve(ctx),
            ast_ty::Type::Ident(v) => v.resolve(ctx),
        }
    }
}

impl Resolve<Type> for Located<ast_ty::PrimType> {
    fn resolve(&self, _ctx: &mut ResolveContext) -> Option<Type> {
        Some(match self.0 {
            ast_ty::PrimType::Unit => Type::Unit,
            ast_ty::PrimType::Int => Type::Int,
            ast_ty::PrimType::Bool => Type::Bool,
        })
    }
}

impl Resolve<Type> for ast_ty::Struct {
    fn resolve(&self, ctx: &mut ResolveContext) -> Option<Type> {
        let mut fields = HashMap::new();
        for field in &self.fields.items {
            let ty = match field.ty.resolve(ctx) {
                Some(v) => v,
                None => Type::Never,
            };
            fields.insert(field.name.0.clone(), ty);
        }
        let struct_id = ctx
            .get_global_table_mut()
            .get_struct_id_by_fields_or_insert_new(fields);
        Some(Type::Struct(struct_id))
    }
}

impl Resolve<Type> for Located<String> {
    fn resolve(&self, _ctx: &mut ResolveContext) -> Option<Type> {
        todo!()
    }
}
