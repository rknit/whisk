use super::{
    ty::{Primitive, TypeKind},
    SymbolTable, TypeId,
};

#[derive(Default, Debug, Clone, Copy)]
pub struct Common {
    pub ty: CommonType,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct CommonType {
    pub never: TypeId,
    pub unit: TypeId,
    pub int: TypeId,
    pub bool: TypeId,
}

pub fn inject_symbol_table(table: &mut SymbolTable) -> Common {
    let common_ty = inject_primitive_types(table);
    Common { ty: common_ty }
}

fn inject_primitive_types(table: &mut SymbolTable) -> CommonType {
    let mut f = |ty: Primitive| {
        let id = table.new_type(ty.to_string()).unwrap();
        let sym = id.sym_mut(table);
        sym.kind = Some(TypeKind::Primitive(ty));
        id
    };
    CommonType {
        never: f(Primitive::Never),
        unit: f(Primitive::Unit),
        int: f(Primitive::Int),
        bool: f(Primitive::Bool),
    }
}
