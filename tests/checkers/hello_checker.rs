use triangle_rs::ast::commands::*;
use triangle_rs::ast::expressions::*;
use triangle_rs::ast::parameters::ActualParameter::*;
use triangle_rs::ast::parameters::ActualParameterSequence::*;
use triangle_rs::ast::parameters::*;
use triangle_rs::ast::Program;
use triangle_rs::checker::std_env::STANDARD_ENVIRONMENT;

pub fn check(program: &Program) -> bool {
    let putint_decl = match *program.cmd {
        Command::CallCommand(ref cmd) => cmd.id.decl.clone(),
        _ => unreachable!(),
    };

    let putint_orig_decl = STANDARD_ENVIRONMENT.lock().unwrap().putint_decl.clone();
    assert_eq!(*putint_decl.unwrap(), putint_orig_decl);

    let iexpr1 = match *program.cmd {
        Command::CallCommand(ref cmd) => match *cmd.aps {
            ActualParameterSequence::SingleActualParameterSequence(ref aps) => match *aps.ap {
                ActualParameter::ConstActualParameter(ref ap) => match *ap.expr {
                    Expression::IntegerExpression(ref iexpr) => iexpr.clone(),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };

    assert_eq!(
        *iexpr1.td.unwrap(),
        STANDARD_ENVIRONMENT.lock().unwrap().int_type.clone()
    );

    true
}
