use crate::ast::commands::*;
use crate::ast::declarations::*;
use crate::ast::expressions::*;
use crate::ast::parameters::*;
use crate::ast::primitives::*;
use crate::ast::typedenoters::*;
use std::default::Default;
use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref STANDARD_ENVIRONMENT: Mutex<StdEnvironment> =
        Mutex::new(StdEnvironment::default());
}

/// The Standard Environment provides pre-defined functionality in the form of constants, types,
/// operators, procedures, and function declarations. It is what is known as the "Prelude" in some
/// other languages
#[derive(Debug)]
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
    pub chr_decl: Declaration,
    pub ord_decl: Declaration,
    pub new_decl: Declaration,
    pub dispose_decl: Declaration,
}

impl StdEnvironment {
    fn declare_std_type(id: &str, td: TypeDenoter) -> Declaration {
        Declaration::TypeDeclaration(TypeDeclarationState::new(Identifier::new(id), td))
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
        let mut decl = Declaration::UnaryOperatorDeclaration(UnaryOperatorDeclarationState::new(
            Operator::new(id),
            arg_type,
            res_type,
        ));

        let decl_clone = decl.clone();
        if let Declaration::UnaryOperatorDeclaration(ref mut state) = decl {
            state.op.decl = Some(Box::new(decl_clone));
        }

        decl
    }

    fn declare_std_binary_operator(
        id: &str,
        arg1_type: TypeDenoter,
        arg2_type: TypeDenoter,
        res_type: TypeDenoter,
    ) -> Declaration {
        let mut decl = Declaration::BinaryOperatorDeclaration(BinaryOperatorDeclarationState::new(
            arg1_type,
            Operator::new(id),
            arg2_type,
            res_type,
        ));

        let decl_clone = decl.clone();
        if let Declaration::BinaryOperatorDeclaration(ref mut state) = decl {
            state.op.decl = Some(Box::new(decl_clone));
        }

        decl
    }

    fn declare_std_procedure(id: &str, fps: FormalParameterSequence, cmd: Command) -> Declaration {
        let mut decl =
            Declaration::ProcDeclaration(ProcDeclarationState::new(Identifier::new(id), fps, cmd));

        let decl_clone = decl.clone();
        if let Declaration::ProcDeclaration(ref mut state) = decl {
            state.id.decl = Some(Box::new(decl_clone));
        }

        decl
    }

    fn declare_std_function(
        id: &str,
        fps: FormalParameterSequence,
        td: TypeDenoter,
        expr: Expression,
    ) -> Declaration {
        let mut decl = Declaration::FuncDeclaration(FuncDeclarationState::new(
            Identifier::new(id),
            fps,
            td,
            expr,
        ));

        let decl_clone = decl.clone();
        if let Declaration::FuncDeclaration(ref mut state) = decl {
            state.id.decl = Some(Box::new(decl_clone));
        }

        decl
    }
}

impl Default for StdEnvironment {
    fn default() -> Self {
        // primitive types

        let any_type = TypeDenoter::AnyTypeDenoter(AnyTypeDenoterState::new());

        let error_type = TypeDenoter::ErrorTypeDenoter(ErrorTypeDenoterState::new());

        let int_type = TypeDenoter::IntTypeDenoter(IntTypeDenoterState::new());

        let char_type = TypeDenoter::CharTypeDenoter(CharTypeDenoterState::new());

        let bool_type = TypeDenoter::BoolTypeDenoter(BoolTypeDenoterState::new());

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
            StdEnvironment::declare_std_unary_operator("neg", int_type.clone(), int_type.clone());

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
    use super::STANDARD_ENVIRONMENT;

    #[test]
    fn test_print_default_std_env() {
        println!("{:#?}", unsafe { STANDARD_ENVIRONMENT.lock().unwrap() });
    }
}
