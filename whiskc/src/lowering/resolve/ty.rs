use crate::{
    ast::nodes as ast,
    lowering::nodes::ty::TypeDecl,
    symbol::{
        ty::{StructType, TypeKind},
        TypeId,
    },
};

use super::{Record, Resolve, ResolveContext};

impl Record<(), bool> for ast::ty::TypeDecl {
    fn record(&self, ctx: &mut ResolveContext, _: ()) -> bool {
        if ctx.table.get_function_by_name(&self.name.0).is_some() {
            todo!("report error");
        }
        if ctx.table.new_type(self.name.0.clone()).is_none() {
            todo!("report error");
        } else {
            true
        }
    }
}

impl Resolve<(), Option<TypeDecl>> for ast::ty::TypeDecl {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> Option<TypeDecl> {
        match &self.kind {
            ast::ty::TypeDeclKind::Type(ast_ty) => {
                let underlying_ty_id = ast_ty.resolve(ctx, ())?;
                let ty_id = ctx
                    .table
                    .get_type_id(&self.name.0)
                    .expect("recorded type name");
                ty_id.sym_mut(ctx.table).kind = Some(TypeKind::Ident(underlying_ty_id));

                let con_func_id = ctx
                    .table
                    .new_function(self.name.0.clone())
                    .expect("valid construct func");
                let con_var_id = {
                    let con_block = ctx.table.new_block(con_func_id);
                    let con_var_id = ctx
                        .table
                        .new_variable("_0".to_owned(), con_block)
                        .expect("temp con param");
                    con_var_id.sym_mut(ctx.table).ty = underlying_ty_id;
                    con_var_id
                };
                let con_sym = con_func_id.sym_mut(ctx.table);
                con_sym.ret_ty = ty_id;
                con_sym.params.push(con_var_id);
            }
            ast::ty::TypeDeclKind::Struct(ast_struct) => {
                let mut fields: Vec<(String, TypeId)> = Vec::new();
                for ast_field in &ast_struct.fields.items {
                    if fields.iter().any(|(name, _)| *name == ast_field.name.0) {
                        todo!("report error");
                    }
                    let ty_id = match ast_field.ty.resolve(ctx, ()) {
                        Some(id) => id,
                        // TODO: maybe it should be 'unknown' type.
                        None => ctx.table.common_type().never,
                    };
                    fields.push((ast_field.name.0.clone(), ty_id));
                }

                let sym = ctx
                    .table
                    .get_type_by_name_mut(&self.name.0)
                    .expect("recorded type name");
                sym.kind = Some(TypeKind::Struct(StructType { fields }));
            }
        };

        Some(TypeDecl(
            ctx.table
                .get_type_id(&self.name.0)
                .expect("recorded type name"),
        ))
    }
}

impl Resolve<(), Option<TypeId>> for ast::ty::Type {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> Option<TypeId> {
        match self {
            ast::ty::Type::Primitive(v) => Some(match v.0 {
                ast::ty::PrimType::Unit => ctx.table.common_type().unit,
                ast::ty::PrimType::Int => ctx.table.common_type().int,
                ast::ty::PrimType::Bool => ctx.table.common_type().bool,
            }),
            ast::ty::Type::Ident(v) => ctx.table.get_type_id(&v.0),
        }
    }
}
