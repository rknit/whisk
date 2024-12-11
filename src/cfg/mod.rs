use builder::{BuildContext, BuildVisitor, Builder};
use nodes::{
    basic_block::BasicBlock,
    function::Function,
    value::{Value, ValueKind},
};

use crate::{
    ast_resolved::{nodes::item::Item, ResolvedAST},
    symbol_table::{Symbol, SymbolID, SymbolTable},
};

pub mod builder;
mod cvt;
pub mod display;
pub mod nodes;

#[derive(Debug)]
pub struct CFG {
    funcs: Vec<Function>,
}
impl CFG {
    pub fn new(ast: &ResolvedAST, sym_table: &mut SymbolTable) -> Self {
        let mut funcs = Vec::new();

        let ctx = &mut BuildContext::new(sym_table);

        for item in &ast.items {
            match item {
                Item::Function(ast_func) => {
                    ctx.set_func_symbol_id(ast_func.sig.sym_id);
                    ctx.push_local(ast_func.table_id);

                    let mut func = Function::new(ast_func.sig.sym_id, ast_func.table_id);
                    let entry = func.add_block(BasicBlock::new(Some("entry".to_owned())));

                    for param in &ast_func.sig.params {
                        let Symbol::Variable(param) =
                            ctx.get_symbol_by_name_mut(&param.0 .0).unwrap()
                        else {
                            panic!("symbol is not a var symbol");
                        };
                        param.set_value(Value {
                            kind: ValueKind::Parameter(param.get_id()),
                            ty: param.get_type(),
                        });
                    }

                    ast_func
                        .body
                        .visit(ctx, &mut Builder::new(&mut func, entry));
                    funcs.push(func);

                    ctx.pop_local();
                    ctx.unset_func_symbol_id();
                }
                Item::ExternFunction(func) => {
                    funcs.push(Function::new(func.0.sym_id, SymbolID::nil()));
                }
            };
        }

        CFG { funcs }
    }
}
