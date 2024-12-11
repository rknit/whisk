use std::{
    fs::{self},
    path::PathBuf,
};

use crate::{
    ast::{self, AST},
    ast_resolved::{self, ResolvedAST},
    cfg::{display::display_cfg, CFG},
    symbol_table::SymbolTable,
};

#[derive(Debug)]
pub struct Module {
    name: String,
    path: PathBuf,
    ast: Option<AST>,
    resolved_ast: Option<ResolvedAST>,
    symbol_table: Option<SymbolTable>,
    cfg: Option<CFG>,
}
impl Module {
    pub fn new(path: PathBuf) -> Self {
        Self {
            name: path
                .file_stem()
                .expect("file name")
                .to_str()
                .expect("valid string")
                .to_string(),
            path,
            ast: None,
            resolved_ast: None,
            symbol_table: None,
            cfg: None,
        }
    }

    pub fn parse_ast(&mut self) -> Option<&AST> {
        self.ast = match ast::parse(&self.path) {
            Ok(ast) => Some(ast),
            Err(errors) => {
                dbg!(&errors);
                return None;
            }
        };
        dbg!(&self.ast);
        self.ast.as_ref()
    }

    pub fn resolve_ast(&mut self) -> Option<&ResolvedAST> {
        let Some(ast) = &self.ast else {
            return None;
        };
        (self.resolved_ast, self.symbol_table) = match ast_resolved::resolve(ast) {
            Ok((ast, sym_table)) => (Some(ast), Some(sym_table)),
            Err(errs) => {
                dbg!(&errs);
                return None;
            }
        };
        dbg!(&self.resolved_ast);
        self.resolved_ast.as_ref()
    }

    pub fn gen_cfg(&mut self) -> Option<&CFG> {
        self.cfg = Some(CFG::new(
            self.resolved_ast.as_ref()?,
            self.symbol_table.as_mut()?,
        ));
        dbg!(&self.cfg);
        self.cfg.as_ref()
    }

    pub fn display_cfg(&self) {
        let Some(sym_table) = &self.symbol_table else {
            return;
        };
        let Some(cfg) = &self.cfg else {
            return;
        };
        let mut s = String::new();
        display_cfg(&mut s, cfg, sym_table);

        let mut disp_path = self.path.clone();
        disp_path.set_extension("wir");
        fs::write(disp_path.clone(), s)
            .expect(format!("unable to write to {}", disp_path.display()).as_str());
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
