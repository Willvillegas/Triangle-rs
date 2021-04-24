use crate::ast::declarations::Declaration;
use crate::ast::declarations::*;
use crate::ast::expressions::*;
use crate::ast::primitives::*;
use crate::ast::typedenoters::*;
use std::default::Default;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref STANDARD_ENVIRONMENT: Mutex<StdEnvironment> =
        Mutex::new(StdEnvironment::default());
}

pub struct StdEnvironment {
    pub any_type: TypeDenoter,
    pub error_type: TypeDenoter,
    pub int_type: TypeDenoter,
    pub char_type: TypeDenoter,
    pub bool_type: TypeDenoter,

    pub int_decl: Declaration,
    pub char_decl: Declaration,
    pub bool_decl: Declaration,
    pub false_decl: Declaration,
    pub true_decl: Declaration,

    pub id_decl: Declaration,
    pub not_decl: Declaration,
    pub and_decl: Declaration,
    pub or_decl: Declaration,
    pub succ_decl: Declaration,
    pub pred_decl: Declaration,
    pub neg_decl: Declaration,
    pub add_decl: Declaration,
    pub sub_decl: Declaration,
    pub mult_decl: Declaration,
    pub div_decl: Declaration,
    pub mod_decl: Declaration,
    pub lt_decl: Declaration,
    pub le_decl: Declaration,
    pub ge_decl: Declaration,
    pub gt_decl: Declaration,
    pub eq_decl: Declaration,
    pub ne_decl: Declaration,
    pub eol_decl: Declaration,
    pub eof_decl: Declaration,
    pub get_decl: Declaration,
    pub put_decl: Declaration,
    pub geteol_decl: Declaration,
    pub puteol_decl: Declaration,
    pub getint_decl: Declaration,
    pub putint_decl: Declaration,
    pub new_decl: Declaration,
    pub dispose_decl: Declaration,
}

impl StdEnvironment {
    fn declare_std_type(id: &str, typ: TypeDenoter) -> Declaration {
        Declaration::TypeDeclaration(TypeDeclarationState::new(Identifier::new(id), typ))
    }

    fn declare_std_const(id: &str, val: &str) -> Declaration {
        Declaration::ConstDeclaration(ConstDeclarationState::new(
            Identifier::new(id),
            Expression::IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new(val))),
        ))
    }

    fn declare_std_unary_operator(
        id: &str,
        arg_type: TypeDenoter,
        res_type: TypeDenoter,
    ) -> Declaration {
        Declaration::UnaryOperatorDeclaration(UnaryOperatorDeclarationState::new(
            Operator::new(id),
            arg_type,
            res_type,
        ))
    }
}

impl Default for StdEnvironment {
    fn default() -> Self {
        let any_type = TypeDenoter::AnyTypeDenoter(AnyTypeDenoterState::new());
        let error_type = TypeDenoter::ErrorTypeDenoter(ErrorTypeDenoterState::new());
        let int_type = TypeDenoter::IntTypeDenoter(IntTypeDenoterState::new());
        let char_type = TypeDenoter::CharTypeDenoter(CharTypeDenoterState::new());
        let bool_type = TypeDenoter::BoolTypeDenoter(BoolTypeDenoterState::new());

        let char_decl = StdEnvironment::declare_std_type("Char", char_type.clone());
        let bool_decl = StdEnvironment::declare_std_type("Boolean", bool_type.clone());
        let int_decl = StdEnvironment::declare_std_type("Integer", int_type.clone());

        let false_decl = StdEnvironment::declare_std_const("false", "0");
        let true_decl = StdEnvironment::declare_std_const("true", "1");
        let not_decl =
            StdEnvironment::declare_std_unary_operator("\\", bool_type.clone(), bool_type.clone());
        let id_decl = todo!();

        let and_decl = todo!();
        let or_decl = todo!();
        let succ_decl = todo!();
        let pred_decl = todo!();
        let neg_decl = todo!();
        let add_decl = todo!();
        let sub_decl = todo!();
        let mult_decl = todo!();
        let div_decl = todo!();
        let mod_decl = todo!();
        let lt_decl = todo!();
        let le_decl = todo!();
        let ge_decl = todo!();
        let gt_decl = todo!();
        let eq_decl = todo!();
        let ne_decl = todo!();
        let eol_decl = todo!();
        let eof_decl = todo!();
        let get_decl = todo!();
        let put_decl = todo!();
        let geteol_decl = todo!();
        let puteol_decl = todo!();
        let getint_decl = todo!();
        let putint_decl = todo!();
        let new_decl = todo!();
        let dispose_decl = todo!();

        todo!()
    }
}
