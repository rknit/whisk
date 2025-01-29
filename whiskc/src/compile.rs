use std::{fs, path::PathBuf};

use crate::{
    ast::{self, AST},
    codegen::codegen_wsk_vm,
    lowering::{self, Module},
};

#[derive(Debug)]
pub struct Compilation {
    name: String,
    path: PathBuf,
    ast: Option<AST>,
    resolved_ast: Option<Module>,
}
impl Compilation {
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
        self.ast.as_ref()
    }

    pub fn resolve_ast(&mut self) -> Option<&Module> {
        let Some(ast) = &self.ast else {
            return None;
        };
        self.resolved_ast = match lowering::resolve(ast) {
            Ok(ast) => Some(ast),
            Err(errs) => {
                dbg!(&errs);
                return None;
            }
        };
        self.resolved_ast.as_ref()
    }

    pub fn codegen(&self) {
        let Some(ast) = &self.resolved_ast else {
            return;
        };

        let prog = match codegen_wsk_vm(ast) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{:?}", e);
                return;
            }
        };

        let mut bin_path = self.path.clone();
        bin_path.set_extension("wc");

        println!("wrote binary to {}", bin_path.display());
        let bin = prog.to_bin();
        fs::write(bin_path, bin).unwrap();
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
