use super::ResolvedAST;

pub mod constant_fold;

pub fn run_passes(_ast: &mut ResolvedAST) {
    // disabled for now since there is no dependencies checking
    // constant_fold(ast);
}
