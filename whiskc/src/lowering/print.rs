use std::fmt::{Display, Write};

use crate::symbol::{ty::TypeKind, SymbolTable};

use super::{visit::Visit, Module};

impl Module {
    pub fn pretty_print<W: Write>(&self, w: &mut W) {
        let mut v = PrintVisitor {
            table: &self.sym_table,
            w,
            item_stack: vec![],
            rename: None,
            prefix: None,
        };
        v.visit_module(self);
        v.finalize();
    }
}

struct PrintVisitor<'a, W: Write> {
    table: &'a SymbolTable,
    w: &'a mut W,
    item_stack: Vec<Item>,
    rename: Option<String>,
    prefix: Option<String>,
}
impl<W: Write> PrintVisitor<'_, W> {
    fn start_item(&mut self, name: impl Display) {
        let name = if let Some(name) = self.rename.take() {
            name
        } else {
            name.to_string()
        };
        let name = if let Some(prefix) = self.prefix.take() {
            format!("{}{}", prefix, name)
        } else {
            name
        };
        self.item_stack.push(Item {
            name,
            attribs: vec![],
            children: vec![],
        })
    }

    fn set_rename(&mut self, name: impl Display) {
        self.rename = Some(name.to_string());
    }

    fn set_prefix(&mut self, prefix: impl Display) {
        self.prefix = Some(prefix.to_string());
    }

    fn end_item(&mut self) {
        let item = self.item_stack.pop().unwrap();
        self.item_stack.last_mut().unwrap().children.push(item);
    }

    fn add_attrib(&mut self, name: &str, value: impl Display) {
        self.item_stack
            .last_mut()
            .unwrap()
            .attribs
            .push(format!("{}: {}", name, value));
    }

    fn add_attrib_to_last_pop(&mut self, name: &str, value: impl Display) {
        self.item_stack
            .last_mut()
            .unwrap()
            .children
            .last_mut()
            .unwrap()
            .attribs
            .push(format!("{}: {}", name, value));
    }

    fn finalize(mut self) {
        let items: Vec<_> = self.item_stack.drain(..).collect();
        for item in &items {
            self.print_tree(item, "".to_owned(), true);
        }
    }

    fn print_tree(&mut self, item: &Item, mut indent: String, is_last: bool) {
        write!(self.w, "{}", indent).unwrap();

        if is_last {
            write!(self.w, "\u{2514}-").unwrap();
            indent += "  ";
        } else {
            write!(self.w, "|-").unwrap();
            indent += "| ";
        }

        writeln!(self.w, "{}", item.name).unwrap();

        for (i, attrib) in item.attribs.iter().enumerate() {
            self.print_attrib(
                attrib,
                indent.clone(),
                item.children.is_empty() && i == item.attribs.len() - 1,
            );
        }

        for (i, child) in item.children.iter().enumerate() {
            self.print_tree(child, indent.clone(), i == item.children.len() - 1);
        }
    }

    fn print_attrib(&mut self, attrib: &str, mut indent: String, is_last: bool) {
        write!(self.w, "{}", indent).unwrap();

        if is_last {
            write!(self.w, "\u{2514}-").unwrap();
            indent += "  ";
        } else {
            write!(self.w, "|-").unwrap();
            indent += "| ";
        }

        writeln!(self.w, "{}", attrib).unwrap();
    }
}

struct Item {
    name: String,
    attribs: Vec<String>,
    children: Vec<Item>,
}

impl<W: Write> Visit for PrintVisitor<'_, W> {
    fn visit_module(&mut self, node: &Module) {
        self.start_item("module");

        self.add_attrib("name", &node.name);

        for item in &node.items {
            self.visit_item(item);
        }
    }

    fn visit_binary_expr(&mut self, node: &super::nodes::expr::BinaryExpr) {
        self.start_item("binary");

        self.add_attrib("op", node.op);

        self.set_prefix("left: ");
        self.visit_expr(&node.left);

        self.set_prefix("right: ");
        self.visit_expr(&node.right);

        self.end_item();
    }

    fn visit_block_expr(&mut self, node: &super::nodes::expr::BlockExpr) {
        self.start_item("block");

        for (i, stmt) in node.stmts.iter().enumerate() {
            self.set_prefix(format!("stmt {}: ", i));
            self.visit_stmt(stmt);
        }

        if let Some(eval_expr) = &node.eval_expr {
            self.set_prefix("eval: ");
            self.visit_expr(eval_expr);
        }

        self.end_item();
    }

    fn visit_bool_expr(&mut self, value: bool) {
        self.start_item(format!("boolean: {}", value).as_str());
        self.end_item();
    }

    fn visit_call_expr(&mut self, node: &super::nodes::expr::CallExpr) {
        self.start_item("call");

        self.set_prefix("caller");
        self.visit_expr(&node.caller);

        self.start_item("args");
        for arg in &node.args {
            self.visit_expr(arg);
        }
        self.end_item();

        self.end_item();
    }

    fn visit_expr(&mut self, node: &super::nodes::expr::Expr) {
        super::visit::visit_expr(self, node);
        self.add_attrib_to_last_pop("type", &node.ty.sym(self.table).name);
    }

    fn visit_extern_func(&mut self, node: &super::nodes::func::ExternFunction) {
        self.start_item("extern_func_decl");

        let sym = node.0.sym(self.table);

        self.add_attrib("name", &sym.name);
        self.add_attrib("return_type", &sym.ret_ty.sym(self.table).name);

        if !sym.params.is_empty() {
            self.start_item("params");
            for param in &sym.params {
                let sym_param = param.sym(self.table);
                self.add_attrib(&sym_param.name, &sym_param.ty.sym(self.table).name);
            }
            self.end_item();
        }

        self.end_item();
    }

    fn visit_func(&mut self, node: &super::nodes::func::Function) {
        self.start_item("func_decl");

        let sym = node.func_id.sym(self.table);

        self.add_attrib("name", &sym.name);
        self.add_attrib("return_type", &sym.ret_ty.sym(self.table).name);

        if !sym.params.is_empty() {
            self.start_item("params");
            for param in &sym.params {
                let sym_param = param.sym(self.table);
                self.add_attrib(&sym_param.name, &sym_param.ty.sym(self.table).name);
            }
            self.end_item();
        }

        self.set_prefix("body: ");
        self.visit_block_expr(&node.body);

        self.end_item();
    }

    fn visit_var_ident_expr(&mut self, node: &super::nodes::expr::VarIdentExpr) {
        self.start_item("var_ident");

        let sym = node.id.sym(self.table);

        self.add_attrib("name", &sym.name);

        self.end_item();
    }

    fn visit_func_ident_expr(&mut self, node: &super::nodes::expr::FuncIdentExpr) {
        self.start_item("func_ident");

        let sym = node.id.sym(self.table);

        self.add_attrib("name", &sym.name);

        self.end_item();
    }

    fn visit_if_expr(&mut self, node: &super::nodes::expr::IfExpr) {
        self.start_item("if");

        self.set_prefix("cond: ");
        self.visit_expr(&node.cond);

        self.set_prefix("then: ");
        self.visit_block_expr(&node.then);

        if let Some(else_) = &node.else_ {
            self.set_prefix("else: ");
            self.visit_block_expr(else_);
        }

        self.end_item();
    }

    fn visit_int_expr(&mut self, value: i64) {
        self.start_item(format!("integer: {}", value).as_str());
        self.end_item();
    }

    fn visit_let_stmt(&mut self, node: &super::nodes::stmt::LetStmt) {
        self.start_item("let_stmt");

        let sym = node.var_id.sym(self.table);

        self.add_attrib("name", &sym.name);
        self.add_attrib("type", &sym.ty.sym(self.table).name);

        self.set_prefix("value: ");
        self.visit_expr(&node.value);

        self.end_item();
    }

    fn visit_loop_expr(&mut self, node: &super::nodes::expr::LoopExpr) {
        self.set_rename("loop");
        super::visit::visit_loop_expr(self, node);
    }

    fn visit_return_expr(&mut self, node: &super::nodes::expr::ReturnExpr) {
        self.set_prefix("return: ");
        super::visit::visit_return_expr(self, node);
    }

    fn visit_type_decl(&mut self, node: &super::nodes::ty::TypeDecl) {
        self.start_item("type_decl");

        let sym = node.0.sym(self.table);

        self.add_attrib("name", &sym.name);

        if let Some(kind) = &sym.kind {
            self.start_item("kind");
            match kind {
                TypeKind::Primitive(v) => self.add_attrib("primitive", v),
                TypeKind::Struct(v) => {
                    self.start_item("struct");
                    for (name, ty) in &v.fields {
                        self.add_attrib(name, &ty.sym(self.table).name)
                    }
                    self.end_item();
                }
                TypeKind::Ident(v) => self.add_attrib("underlying", &v.sym(self.table).name),
            };
            self.end_item();
        } else {
            self.add_attrib("kind", "unknown");
        }

        self.end_item();
    }

    fn visit_unary_expr(&mut self, node: &super::nodes::expr::UnaryExpr) {
        self.start_item("unary");

        self.add_attrib("op", node.op);

        self.visit_expr(&node.expr);

        self.end_item();
    }

    fn visit_unit_expr(&mut self) {
        self.start_item("unit");
        self.end_item();
    }
}
