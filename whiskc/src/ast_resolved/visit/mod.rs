use super::{
    nodes::{
        func::{ExternFunction, Function, FunctionSig, Param},
        item::Item,
    },
    ResolvedAST,
};

pub trait Visit: Sized {
    fn visit_ast(&mut self, node: &ResolvedAST) {
        visit_ast(self, node);
    }

    fn visit_extern_func(&mut self, node: &ExternFunction) {
        visit_extern_func(self, node);
    }

    fn visit_func(&mut self, node: &Function) {
        visit_func(self, node);
    }

    fn visit_func_sig(&mut self, node: &FunctionSig) {
        visit_func_sig(self, node);
    }

    fn visit_item(&mut self, node: &Item) {
        visit_item(self, node);
    }

    fn visit_param(&mut self, _node: &Param) {}
}

pub fn visit_ast(v: &mut impl Visit, node: &ResolvedAST) {
    for item in &node.items {
        v.visit_item(item);
    }
}

pub fn visit_extern_func(v: &mut impl Visit, node: &ExternFunction) {
    v.visit_func_sig(&node.0);
}

pub fn visit_func(v: &mut impl Visit, node: &Function) {
    v.visit_func_sig(&node.sig);
    todo!()
}

pub fn visit_func_sig(v: &mut impl Visit, node: &FunctionSig) {
    for param in &node.params {
        v.visit_param(param);
    }
}

pub fn visit_item(v: &mut impl Visit, node: &Item) {
    match node {
        Item::Function(node) => v.visit_func(node),
        Item::ExternFunction(node) => v.visit_extern_func(node),
    }
}
