use crate::ast::commands::*;
use crate::ast::declarations::*;
use crate::ast::expressions::*;
use crate::ast::parameters::*;
use crate::ast::primitives::*;
use crate::ast::typedenoters::*;
use std::default::Default;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref STANDARD_ENVIRONMENT: Mutex<StdEnvironment> =
        Mutex::new(StdEnvironment::default());
}

/// The Standard Environment provides pre-defined functionality in the form of constants, types,
/// operators, procedures, and function declarations. It is what is known as the "Prelude" in some
/// other languages
pub struct StdEnvironment {
    pub any_type: Arc<Mutex<TypeDenoter>>,
    pub error_type: Arc<Mutex<TypeDenoter>>,
    pub int_type: Arc<Mutex<TypeDenoter>>,
    pub char_type: Arc<Mutex<TypeDenoter>>,
    pub bool_type: Arc<Mutex<TypeDenoter>>,

    pub int_decl: Arc<Mutex<Declaration>>,
    pub char_decl: Arc<Mutex<Declaration>>,
    pub bool_decl: Arc<Mutex<Declaration>>,
    pub false_decl: Arc<Mutex<Declaration>>,
    pub true_decl: Arc<Mutex<Declaration>>,

    pub id_decl: Arc<Mutex<Declaration>>,
    pub not_decl: Arc<Mutex<Declaration>>,
    pub and_decl: Arc<Mutex<Declaration>>,
    pub or_decl: Arc<Mutex<Declaration>>,
    pub succ_decl: Arc<Mutex<Declaration>>,
    pub pred_decl: Arc<Mutex<Declaration>>,
    pub neg_decl: Arc<Mutex<Declaration>>,
    pub add_decl: Arc<Mutex<Declaration>>,
    pub sub_decl: Arc<Mutex<Declaration>>,
    pub mult_decl: Arc<Mutex<Declaration>>,
    pub div_decl: Arc<Mutex<Declaration>>,
    pub mod_decl: Arc<Mutex<Declaration>>,
    pub lt_decl: Arc<Mutex<Declaration>>,
    pub le_decl: Arc<Mutex<Declaration>>,
    pub ge_decl: Arc<Mutex<Declaration>>,
    pub gt_decl: Arc<Mutex<Declaration>>,
    pub eq_decl: Arc<Mutex<Declaration>>,
    pub ne_decl: Arc<Mutex<Declaration>>,
    pub eol_decl: Arc<Mutex<Declaration>>,
    pub eof_decl: Arc<Mutex<Declaration>>,
    pub get_decl: Arc<Mutex<Declaration>>,
    pub put_decl: Arc<Mutex<Declaration>>,
    pub geteol_decl: Arc<Mutex<Declaration>>,
    pub puteol_decl: Arc<Mutex<Declaration>>,
    pub getint_decl: Arc<Mutex<Declaration>>,
    pub putint_decl: Arc<Mutex<Declaration>>,
    pub new_decl: Arc<Mutex<Declaration>>,
    pub dispose_decl: Arc<Mutex<Declaration>>,
}

impl StdEnvironment {
    fn declare_std_type(id: &str, td: Arc<Mutex<TypeDenoter>>) -> Arc<Mutex<Declaration>> {
        Arc::new(Mutex::new(Declaration::TypeDeclaration(
            TypeDeclarationState::new(Identifier::new(id), td),
        )))
    }

    fn declare_std_const(id: &str, val: &str) -> Arc<Mutex<Declaration>> {
        Arc::new(Mutex::new(Declaration::ConstDeclaration(
            ConstDeclarationState::new(
                Identifier::new(id),
                Expression::IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new(
                    val,
                ))),
            ),
        )))
    }

    fn declare_std_unary_operator(
        id: &str,
        arg_type: Arc<Mutex<TypeDenoter>>,
        res_type: Arc<Mutex<TypeDenoter>>,
    ) -> Arc<Mutex<Declaration>> {
        let decl = Arc::new(Mutex::new(Declaration::UnaryOperatorDeclaration(
            UnaryOperatorDeclarationState::new(Operator::new(id), arg_type, res_type),
        )));

        if let &mut Declaration::UnaryOperatorDeclaration(ref mut state) =
            &mut *decl.lock().unwrap()
        {
            state.op.decl = Some(decl.clone());
        }

        decl
    }

    fn declare_std_binary_operator(
        id: &str,
        arg1_type: Arc<Mutex<TypeDenoter>>,
        arg2_type: Arc<Mutex<TypeDenoter>>,
        res_type: Arc<Mutex<TypeDenoter>>,
    ) -> Arc<Mutex<Declaration>> {
        let decl = Arc::new(Mutex::new(Declaration::BinaryOperatorDeclaration(
            BinaryOperatorDeclarationState::new(arg1_type, Operator::new(id), arg2_type, res_type),
        )));

        if let &mut Declaration::BinaryOperatorDeclaration(ref mut state) =
            &mut *decl.lock().unwrap()
        {
            state.op.decl = Some(decl.clone());
        }

        decl
    }

    fn declare_std_procedure(
        id: &str,
        fps: FormalParameterSequence,
        cmd: Command,
    ) -> Arc<Mutex<Declaration>> {
        let decl = Arc::new(Mutex::new(Declaration::ProcDeclaration(
            ProcDeclarationState::new(Identifier::new(id), fps, cmd),
        )));

        if let &mut Declaration::ProcDeclaration(ref mut state) = &mut *decl.lock().unwrap() {
            state.id.decl = Some(decl.clone());
        }

        decl
    }

    fn declare_std_function(
        id: &str,
        fps: FormalParameterSequence,
        td: Arc<Mutex<TypeDenoter>>,
        expr: Expression,
    ) -> Arc<Mutex<Declaration>> {
        let decl = Arc::new(Mutex::new(Declaration::FuncDeclaration(
            FuncDeclarationState::new(Identifier::new(id), fps, td, expr),
        )));

        if let &mut Declaration::FuncDeclaration(ref mut state) = &mut *decl.lock().unwrap() {
            state.id.decl = Some(decl.clone());
        }

        decl
    }
}

impl Default for StdEnvironment {
    fn default() -> Self {
        let any_type = Arc::new(Mutex::new(TypeDenoter::AnyTypeDenoter(
            AnyTypeDenoterState::new(),
        )));

        let error_type = Arc::new(Mutex::new(TypeDenoter::ErrorTypeDenoter(
            ErrorTypeDenoterState::new(),
        )));

        let int_type = Arc::new(Mutex::new(TypeDenoter::IntTypeDenoter(
            IntTypeDenoterState::new(),
        )));

        let char_type = Arc::new(Mutex::new(TypeDenoter::CharTypeDenoter(
            CharTypeDenoterState::new(),
        )));

        let bool_type = Arc::new(Mutex::new(TypeDenoter::BoolTypeDenoter(
            BoolTypeDenoterState::new(),
        )));

        let char_decl = StdEnvironment::declare_std_type("Char", char_type.clone());
        let bool_decl = StdEnvironment::declare_std_type("Boolean", bool_type.clone());
        let int_decl = StdEnvironment::declare_std_type("Integer", int_type.clone());

        let false_decl = StdEnvironment::declare_std_const("false", "0");
        let true_decl = StdEnvironment::declare_std_const("true", "1");
        let not_decl =
            StdEnvironment::declare_std_unary_operator("\\", bool_type.clone(), bool_type.clone());

        let and_decl = StdEnvironment::declare_std_binary_operator(
            "/\\",
            bool_type.clone(),
            bool_type.clone(),
            bool_type.clone(),
        );

        let or_decl = StdEnvironment::declare_std_binary_operator(
            "\\/",
            bool_type.clone(),
            bool_type.clone(),
            bool_type.clone(),
        );

        let neg_decl =
            StdEnvironment::declare_std_unary_operator("-", int_type.clone(), int_type.clone());
        let add_decl = StdEnvironment::declare_std_binary_operator(
            "+",
            int_type.clone(),
            int_type.clone(),
            int_type.clone(),
        );
        let sub_decl = StdEnvironment::declare_std_binary_operator(
            "-",
            int_type.clone(),
            int_type.clone(),
            int_type.clone(),
        );
        let mult_decl = StdEnvironment::declare_std_binary_operator(
            "*",
            int_type.clone(),
            int_type.clone(),
            int_type.clone(),
        );
        let div_decl = StdEnvironment::declare_std_binary_operator(
            "/",
            int_type.clone(),
            int_type.clone(),
            int_type.clone(),
        );
        let mod_decl = StdEnvironment::declare_std_binary_operator(
            "//",
            int_type.clone(),
            int_type.clone(),
            int_type.clone(),
        );
        let lt_decl = StdEnvironment::declare_std_binary_operator(
            "<",
            int_type.clone(),
            int_type.clone(),
            bool_type.clone(),
        );
        let le_decl = StdEnvironment::declare_std_binary_operator(
            "<=",
            int_type.clone(),
            int_type.clone(),
            bool_type.clone(),
        );
        let gt_decl = StdEnvironment::declare_std_binary_operator(
            ">",
            int_type.clone(),
            int_type.clone(),
            bool_type.clone(),
        );
        let ge_decl = StdEnvironment::declare_std_binary_operator(
            ">=",
            int_type.clone(),
            int_type.clone(),
            bool_type.clone(),
        );
        let eq_decl = StdEnvironment::declare_std_binary_operator(
            "=",
            any_type.clone(),
            any_type.clone(),
            bool_type.clone(),
        );
        let ne_decl = StdEnvironment::declare_std_binary_operator(
            "/=",
            any_type.clone(),
            any_type.clone(),
            bool_type.clone(),
        );

        let put_decl = todo!();
        let get_decl = todo!();
        let geteol_decl = todo!();
        let getint_decl = todo!();
        let puteol_decl = todo!();
        let putint_decl = todo!();

        let id_decl = todo!();
        let succ_decl = todo!();
        let pred_decl = todo!();
        let eol_decl = todo!();
        let eof_decl = todo!();
        let new_decl = todo!();
        let dispose_decl = todo!();

        todo!()
    }
}
