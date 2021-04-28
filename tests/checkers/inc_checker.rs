use triangle_rs::ast::commands::*;
use triangle_rs::ast::expressions::*;
use triangle_rs::ast::parameters::ActualParameter::*;
use triangle_rs::ast::parameters::ActualParameterSequence::*;
use triangle_rs::ast::parameters::*;
use triangle_rs::ast::Program;
use triangle_rs::checker::std_env::STANDARD_ENVIRONMENT;

pub fn check(program: &Program) -> bool {
    true
}
