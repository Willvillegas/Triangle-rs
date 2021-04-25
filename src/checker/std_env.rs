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
    pub chr_decl: Arc<Mutex<Declaration>>,
    pub ord_decl: Arc<Mutex<Declaration>>,
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
        // primitive types

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

        // declarations for the primitive types

        let char_decl = StdEnvironment::declare_std_type("Char", char_type.clone());
        let bool_decl = StdEnvironment::declare_std_type("Boolean", bool_type.clone());
        let int_decl = StdEnvironment::declare_std_type("Integer", int_type.clone());

        // constants

        let false_decl = StdEnvironment::declare_std_const("false", "0");
        let true_decl = StdEnvironment::declare_std_const("true", "1");

        // operators

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

        // procedures

        let succ_decl = {
            let dummy_id = Identifier::default();
            let fps = FormalParameterSequence::SingleFormalParameterSequence(
                SingleFormalParameterSequenceState::new(FormalParameter::VarFormalParameter(
                    VarFormalParameterState::new(dummy_id, int_type.clone()),
                )),
            );
            let cmd = Command::EmptyCommand(EmptyCommandState::new());
            StdEnvironment::declare_std_procedure("succ", fps, cmd)
        };

        let pred_decl = {
            let dummy_id = Identifier::default();
            let fps = FormalParameterSequence::SingleFormalParameterSequence(
                SingleFormalParameterSequenceState::new(FormalParameter::VarFormalParameter(
                    VarFormalParameterState::new(dummy_id, int_type.clone()),
                )),
            );
            let cmd = Command::EmptyCommand(EmptyCommandState::new());
            StdEnvironment::declare_std_procedure("pred", fps, cmd)
        };

        let put_decl = {
            let dummy_id = Identifier::default();
            let fps = FormalParameterSequence::SingleFormalParameterSequence(
                SingleFormalParameterSequenceState::new(FormalParameter::ConstFormalParameter(
                    ConstFormalParameterState::new(dummy_id, char_type.clone()),
                )),
            );
            let cmd = Command::EmptyCommand(EmptyCommandState::new());
            StdEnvironment::declare_std_procedure("put", fps, cmd)
        };

        let get_decl = {
            let dummy_id = Identifier::default();
            let fps = FormalParameterSequence::SingleFormalParameterSequence(
                SingleFormalParameterSequenceState::new(FormalParameter::VarFormalParameter(
                    VarFormalParameterState::new(dummy_id, char_type.clone()),
                )),
            );
            let cmd = Command::EmptyCommand(EmptyCommandState::new());
            StdEnvironment::declare_std_procedure("get", fps, cmd)
        };

        let geteol_decl = {
            let fps = FormalParameterSequence::EmptyFormalParameterSequence(
                EmptyFormalParameterSequenceState::new(),
            );
            let cmd = Command::EmptyCommand(EmptyCommandState::new());
            StdEnvironment::declare_std_procedure("geteol", fps, cmd)
        };

        let getint_decl = {
            let dummy_id = Identifier::default();
            let fps = FormalParameterSequence::SingleFormalParameterSequence(
                SingleFormalParameterSequenceState::new(FormalParameter::VarFormalParameter(
                    VarFormalParameterState::new(dummy_id, int_type.clone()),
                )),
            );
            let cmd = Command::EmptyCommand(EmptyCommandState::new());
            StdEnvironment::declare_std_procedure("getint", fps, cmd)
        };

        let puteol_decl = {
            let fps = FormalParameterSequence::EmptyFormalParameterSequence(
                EmptyFormalParameterSequenceState::new(),
            );
            let cmd = Command::EmptyCommand(EmptyCommandState::new());
            StdEnvironment::declare_std_procedure("puteol", fps, cmd)
        };

        let putint_decl = {
            let dummy_id = Identifier::default();
            let fps = FormalParameterSequence::SingleFormalParameterSequence(
                SingleFormalParameterSequenceState::new(FormalParameter::ConstFormalParameter(
                    ConstFormalParameterState::new(dummy_id, int_type.clone()),
                )),
            );
            let cmd = Command::EmptyCommand(EmptyCommandState::new());
            StdEnvironment::declare_std_procedure("putint", fps, cmd)
        };

        let new_decl = {
            let dummy_id = Identifier::default();
            let fps = FormalParameterSequence::SingleFormalParameterSequence(
                SingleFormalParameterSequenceState::new(FormalParameter::ConstFormalParameter(
                    ConstFormalParameterState::new(dummy_id, int_type.clone()),
                )),
            );
            let cmd = Command::EmptyCommand(EmptyCommandState::new());
            StdEnvironment::declare_std_procedure("new", fps, cmd)
        };

        let dispose_decl = {
            let dummy_id1 = Identifier::default();
            let dummy_id2 = Identifier::default();
            let fps = FormalParameterSequence::MultipleFormalParameterSequence(
                MultipleFormalParameterSequenceState::new(
                    FormalParameter::ConstFormalParameter(ConstFormalParameterState::new(
                        dummy_id1,
                        int_type.clone(),
                    )),
                    FormalParameterSequence::SingleFormalParameterSequence(
                        SingleFormalParameterSequenceState::new(
                            FormalParameter::ConstFormalParameter(ConstFormalParameterState::new(
                                dummy_id2,
                                int_type.clone(),
                            )),
                        ),
                    ),
                ),
            );
            let cmd = Command::EmptyCommand(EmptyCommandState::new());
            StdEnvironment::declare_std_procedure("dispose", fps, cmd)
        };

        // functions

        let id_decl = {
            let dummy_id = Identifier::default();
            let fps = FormalParameterSequence::SingleFormalParameterSequence(
                SingleFormalParameterSequenceState::new(FormalParameter::ConstFormalParameter(
                    ConstFormalParameterState::new(dummy_id, any_type.clone()),
                )),
            );
            let expr = Expression::EmptyExpression(EmptyExpressionState::new());
            StdEnvironment::declare_std_function("id", fps, any_type.clone(), expr)
        };

        let chr_decl = {
            let dummy_id = Identifier::default();
            let fps = FormalParameterSequence::SingleFormalParameterSequence(
                SingleFormalParameterSequenceState::new(FormalParameter::ConstFormalParameter(
                    ConstFormalParameterState::new(dummy_id, int_type.clone()),
                )),
            );
            let expr = Expression::EmptyExpression(EmptyExpressionState::new());

            StdEnvironment::declare_std_function("chr", fps, char_type.clone(), expr)
        };

        let ord_decl = {
            let dummy_id = Identifier::default();
            let fps = FormalParameterSequence::SingleFormalParameterSequence(
                SingleFormalParameterSequenceState::new(FormalParameter::ConstFormalParameter(
                    ConstFormalParameterState::new(dummy_id, char_type.clone()),
                )),
            );
            let expr = Expression::EmptyExpression(EmptyExpressionState::new());
            StdEnvironment::declare_std_function("ord", fps, int_type.clone(), expr)
        };

        let eol_decl = {
            let fps = FormalParameterSequence::EmptyFormalParameterSequence(
                EmptyFormalParameterSequenceState::new(),
            );
            let expr = Expression::EmptyExpression(EmptyExpressionState::new());
            StdEnvironment::declare_std_function("eol", fps, bool_type.clone(), expr)
        };

        let eof_decl = {
            let fps = FormalParameterSequence::EmptyFormalParameterSequence(
                EmptyFormalParameterSequenceState::new(),
            );
            let expr = Expression::EmptyExpression(EmptyExpressionState::new());
            StdEnvironment::declare_std_function("eof", fps, bool_type.clone(), expr)
        };

        StdEnvironment {
            any_type,
            error_type,
            int_type,
            char_type,
            bool_type,

            int_decl,
            char_decl,
            bool_decl,
            false_decl,
            true_decl,

            id_decl,
            not_decl,
            and_decl,
            or_decl,
            succ_decl,
            pred_decl,
            neg_decl,
            add_decl,
            sub_decl,
            mult_decl,
            div_decl,
            mod_decl,
            lt_decl,
            le_decl,
            ge_decl,
            gt_decl,
            eq_decl,
            ne_decl,
            eol_decl,
            eof_decl,
            get_decl,
            put_decl,
            geteol_decl,
            puteol_decl,
            getint_decl,
            putint_decl,
            chr_decl,
            ord_decl,
            new_decl,
            dispose_decl,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_print_default_std_env() {
        println!("{:#?}", unsafe { STANDARD_ENVIRONMENT.lock().unwrap() });
    }
}
