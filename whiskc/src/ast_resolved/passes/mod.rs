use self::constant_fold::constant_fold;

use super::ResolvedAST;

pub mod constant_fold;

pub fn run_passes(ast: &mut ResolvedAST) {
    constant_fold(ast);
}
