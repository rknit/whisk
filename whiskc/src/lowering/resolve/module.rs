use crate::{
    ast::AST,
    lowering::{
        errors::ResolveError,
        nodes::{item::Item, module::Module},
        Resolve, ResolveContext,
    },
    symbol,
    symbol_table::SymbolTable,
};

pub fn resolve(ast: &AST) -> Result<Module, Vec<ResolveError>> {
    let sym_table = symbol::SymbolTable::default();

    let mut global_table = SymbolTable::default();
    let mut ctx = ResolveContext::new(&mut global_table);

    for item in &ast.items {
        use crate::ast::nodes::item::Item;
        match item {
            Item::Function(function) => function.sig.resolve(&mut ctx),
            Item::ExternFunction(function) => function.sig.resolve(&mut ctx),
            _ => todo!(),
        };
    }

    let mut items = Vec::new();

    for item in &ast.items {
        let Some(item): Option<Item> = ({
            use crate::ast::nodes::item::Item;
            match item {
                Item::Function(function) => function.resolve(&mut ctx).map(|v| v.into()),
                Item::ExternFunction(function) => function.resolve(&mut ctx).map(|v| v.into()),
                _ => todo!(),
            }
        }) else {
            continue;
        };
        items.push(item);
    }

    ctx.finalize()?;
    Ok(Module {
        sym_table_old: global_table,
        sym_table,
        items,
    })
}
