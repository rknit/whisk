use crate::{ast::nodes as ast, lowering::nodes::ty::TypeDecl, symbol::TypeId};

use super::{Record, Resolve, ResolveContext};

impl Record for ast::ty::TypeDecl {
    fn record(&self, ctx: &mut ResolveContext, _: ()) {
        if ctx.table.new_type(self.name.0.clone()).is_none() {
            todo!("report error");
        }
    }
}

impl Resolve<(), Option<TypeDecl>> for ast::ty::TypeDecl {
    fn resolve(&self, _ctx: &mut ResolveContext, _: ()) -> Option<TypeDecl> {
        todo!()
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
            ast::ty::Type::Struct(_) => todo!(),
        }
    }
}
