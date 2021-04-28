//! The Checker module.
//! This module traverses the AST generated by the Parser, checks that the AST is well-formed,
//! and decorates it with links from applied occurrences of identfiers to their corresponding declarations,
//! as well as expressions, type-denoters, and vnames with their type.

use crate::ast::aggregates::*;
use crate::ast::commands::*;
use crate::ast::declarations::*;
use crate::ast::expressions::*;
use crate::ast::introspection::{get_of_type, is_of_type};
use crate::ast::parameters::*;
use crate::ast::primitives::*;
use crate::ast::typedenoters::*;
use crate::ast::vnames::*;
use crate::ast::*;
use crate::error::{report_error_and_exit, CheckerError, GenError};

mod id_table;
pub mod std_env;

use id_table::IdentificationTable;
use std_env::STANDARD_ENVIRONMENT;

pub struct Checker {
    id_table: IdentificationTable,
}

impl Checker {
    pub fn new() -> Self {
        let mut checker = Checker {
            id_table: IdentificationTable::new(),
        };
        checker.establish_standard_environment();
        checker
    }

    /// set up the standard environment (the prelude)
    fn establish_standard_environment(&mut self) {
        self.id_table.enter(
            String::from("Integer"),
            STANDARD_ENVIRONMENT.lock().unwrap().int_decl.clone(),
        );

        self.id_table.enter(
            String::from("Char"),
            STANDARD_ENVIRONMENT.lock().unwrap().char_decl.clone(),
        );

        self.id_table.enter(
            String::from("Boolean"),
            STANDARD_ENVIRONMENT.lock().unwrap().bool_decl.clone(),
        );
        self.id_table.enter(
            String::from("false"),
            STANDARD_ENVIRONMENT.lock().unwrap().false_decl.clone(),
        );
        self.id_table.enter(
            String::from("true"),
            STANDARD_ENVIRONMENT.lock().unwrap().true_decl.clone(),
        );

        self.id_table.enter(
            String::from("id"),
            STANDARD_ENVIRONMENT.lock().unwrap().id_decl.clone(),
        );
        self.id_table.enter(
            String::from("\\"),
            STANDARD_ENVIRONMENT.lock().unwrap().not_decl.clone(),
        );
        self.id_table.enter(
            String::from("/\\"),
            STANDARD_ENVIRONMENT.lock().unwrap().and_decl.clone(),
        );
        self.id_table.enter(
            String::from("\\/"),
            STANDARD_ENVIRONMENT.lock().unwrap().or_decl.clone(),
        );
        self.id_table.enter(
            String::from("succ"),
            STANDARD_ENVIRONMENT.lock().unwrap().succ_decl.clone(),
        );
        self.id_table.enter(
            String::from("pred"),
            STANDARD_ENVIRONMENT.lock().unwrap().pred_decl.clone(),
        );
        self.id_table.enter(
            String::from("neg"),
            STANDARD_ENVIRONMENT.lock().unwrap().neg_decl.clone(),
        );
        self.id_table.enter(
            String::from("+"),
            STANDARD_ENVIRONMENT.lock().unwrap().add_decl.clone(),
        );
        self.id_table.enter(
            String::from("-"),
            STANDARD_ENVIRONMENT.lock().unwrap().sub_decl.clone(),
        );
        self.id_table.enter(
            String::from("*"),
            STANDARD_ENVIRONMENT.lock().unwrap().mult_decl.clone(),
        );
        self.id_table.enter(
            String::from("/"),
            STANDARD_ENVIRONMENT.lock().unwrap().div_decl.clone(),
        );
        self.id_table.enter(
            String::from("//"),
            STANDARD_ENVIRONMENT.lock().unwrap().mod_decl.clone(),
        );
        self.id_table.enter(
            String::from("<"),
            STANDARD_ENVIRONMENT.lock().unwrap().lt_decl.clone(),
        );
        self.id_table.enter(
            String::from("<="),
            STANDARD_ENVIRONMENT.lock().unwrap().le_decl.clone(),
        );
        self.id_table.enter(
            String::from(">="),
            STANDARD_ENVIRONMENT.lock().unwrap().ge_decl.clone(),
        );
        self.id_table.enter(
            String::from(">"),
            STANDARD_ENVIRONMENT.lock().unwrap().gt_decl.clone(),
        );
        self.id_table.enter(
            String::from("="),
            STANDARD_ENVIRONMENT.lock().unwrap().eq_decl.clone(),
        );
        self.id_table.enter(
            String::from("/="),
            STANDARD_ENVIRONMENT.lock().unwrap().ne_decl.clone(),
        );
        self.id_table.enter(
            String::from("eol"),
            STANDARD_ENVIRONMENT.lock().unwrap().eol_decl.clone(),
        );
        self.id_table.enter(
            String::from("eof"),
            STANDARD_ENVIRONMENT.lock().unwrap().eof_decl.clone(),
        );
        self.id_table.enter(
            String::from("get"),
            STANDARD_ENVIRONMENT.lock().unwrap().get_decl.clone(),
        );
        self.id_table.enter(
            String::from("put"),
            STANDARD_ENVIRONMENT.lock().unwrap().put_decl.clone(),
        );
        self.id_table.enter(
            String::from("geteol"),
            STANDARD_ENVIRONMENT.lock().unwrap().geteol_decl.clone(),
        );
        self.id_table.enter(
            String::from("puteol"),
            STANDARD_ENVIRONMENT.lock().unwrap().puteol_decl.clone(),
        );
        self.id_table.enter(
            String::from("getint"),
            STANDARD_ENVIRONMENT.lock().unwrap().getint_decl.clone(),
        );
        self.id_table.enter(
            String::from("putint"),
            STANDARD_ENVIRONMENT.lock().unwrap().putint_decl.clone(),
        );
        self.id_table.enter(
            String::from("chr"),
            STANDARD_ENVIRONMENT.lock().unwrap().chr_decl.clone(),
        );
        self.id_table.enter(
            String::from("ord"),
            STANDARD_ENVIRONMENT.lock().unwrap().ord_decl.clone(),
        );
        self.id_table.enter(
            String::from("new"),
            STANDARD_ENVIRONMENT.lock().unwrap().new_decl.clone(),
        );
        self.id_table.enter(
            String::from("dispose"),
            STANDARD_ENVIRONMENT.lock().unwrap().dispose_decl.clone(),
        );
    }

    /// Check that the AST is well-formed, link all applied occurrences of identifiers and
    /// operators to their declarations, check that all expressions and typedenoters have
    /// proper types.
    pub fn check(&self, program: &mut Program) {
        program.accept(self, AstObject::Null);
    }
}

impl AstVisitor for Checker {
    fn visit_program(&self, program: &mut Program, arg: AstObject) -> AstObject {
        program.cmd.accept(self, arg.clone());
        AstObject::Null
    }

    fn visit_empty_command(&self, cmd: &mut EmptyCommandState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_assign_command(&self, cmd: &mut AssignCommandState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_call_command(&self, cmd: &mut CallCommandState, arg: AstObject) -> AstObject {
        let id_decl_ast = cmd.id.accept(self, arg.clone());
        cmd.aps.accept(self, id_decl_ast);
        AstObject::Null
    }

    fn visit_let_command(&self, cmd: &mut LetCommandState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_if_command(&self, cmd: &mut IfCommandState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_while_command(&self, cmd: &mut WhileCommandState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_sequential_command(
        &self,
        cmd: &mut SequentialCommandState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_empty_expression(&self, expr: &mut EmptyExpressionState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_integer_expression(
        &self,
        expr: &mut IntegerExpressionState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_character_expression(
        &self,
        expr: &mut CharacterExpressionState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_vname_expression(&self, expr: &mut VnameExpressionState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_let_expression(&self, expr: &mut LetExpressionState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_call_expression(&self, expr: &mut CallExpressionState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_if_expression(&self, expr: &mut IfExpressionState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_unary_expression(&self, expr: &mut UnaryExpressionState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_binary_expression(
        &self,
        expr: &mut BinaryExpressionState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_array_expression(&self, expr: &mut ArrayExpressionState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_record_expression(
        &self,
        expr: &mut RecordExpressionState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_single_array_aggregate(
        &self,
        agg: &mut SingleArrayAggregateState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_multiple_array_aggregate(
        &self,
        agg: &mut MultipleArrayAggregateState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_single_record_aggregate(
        &self,
        agg: &mut SingleRecordAggregateState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_multiple_record_aggregate(
        &self,
        agg: &mut MultipleRecordAggregateState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_const_declaration(
        &self,
        decl: &mut ConstDeclarationState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_var_declaration(&self, decl: &mut VarDeclarationState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_proc_declaration(&self, decl: &mut ProcDeclarationState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_func_declaration(&self, decl: &mut FuncDeclarationState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_type_declaration(&self, decl: &mut TypeDeclarationState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_unary_operator_declaration(
        &self,
        decl: &mut UnaryOperatorDeclarationState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_binary_operator_declaration(
        &self,
        decl: &mut BinaryOperatorDeclarationState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_sequential_declaration(
        &self,
        decl: &mut SequentialDeclarationState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_any_type_denoter(&self, td: &mut AnyTypeDenoterState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_error_type_denoter(
        &self,
        td: &mut ErrorTypeDenoterState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_bool_type_denoter(&self, td: &mut BoolTypeDenoterState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_char_type_denoter(&self, td: &mut CharTypeDenoterState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_int_type_denoter(&self, td: &mut IntTypeDenoterState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_array_type_denoter(
        &self,
        td: &mut ArrayTypeDenoterState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_simple_type_denoter(
        &self,
        td: &mut SimpleTypeDenoterState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_single_field_type_denoter(
        &self,
        td: &mut SingleFieldTypeDenoterState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_multiple_field_type_denoter(
        &self,
        td: &mut MultipleFieldTypeDenoterState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_record_type_denoter(
        &self,
        td: &mut RecordTypeDenoterState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_empty_formal_parameter_sequence(
        &self,
        fps: &mut EmptyFormalParameterSequenceState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_single_formal_parameter_sequence(
        &self,
        fps: &mut SingleFormalParameterSequenceState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_multiple_formal_parameter_sequence(
        &self,
        fps: &mut MultipleFormalParameterSequenceState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_const_formal_parameter(
        &self,
        fp: &mut ConstFormalParameterState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_var_formal_parameter(
        &self,
        fp: &mut VarFormalParameterState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_proc_formal_parameter(
        &self,
        fp: &mut ProcFormalParameterState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_func_formal_parameter(
        &self,
        fp: &mut FuncFormalParameterState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_empty_actual_parameter_sequence(
        &self,
        aps: &mut EmptyActualParameterSequenceState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_single_actual_parameter_sequence(
        &self,
        aps: &mut SingleActualParameterSequenceState,
        arg: AstObject,
    ) -> AstObject {
        // use intropection methods along with retrieval methods in the enums
        AstObject::Null
    }

    fn visit_multiple_actual_parameter_sequence(
        &self,
        aps: &mut MultipleActualParameterSequenceState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_const_actual_parameter(
        &self,
        ap: &mut ConstActualParameterState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_var_actual_parameter(
        &self,
        ap: &mut VarActualParameterState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_proc_actual_parameter(
        &self,
        ap: &mut ProcActualParameterState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_func_actual_parameter(
        &self,
        ap: &mut FuncActualParameterState,
        arg: AstObject,
    ) -> AstObject {
        AstObject::Null
    }

    fn visit_simple_vname(&self, vname: &mut SimpleVnameState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_dot_vname(&self, vname: &mut DotVnameState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_subscript_vname(&self, vname: &mut SubscriptVnameState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_identifier(&self, id: &mut Identifier, arg: AstObject) -> AstObject {
        if let Some(decl) = self.id_table.retrieve(&id.spelling) {
            id.decl = Some(Box::new(*decl.clone()));
            return AstObject::Declaration(id.decl.clone().unwrap());
        }
        AstObject::Null
    }

    fn visit_integer_literal(&self, il: &mut IntegerLiteral, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_character_literal(&self, cl: &mut CharacterLiteral, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_operator(&self, op: &mut Operator, arg: AstObject) -> AstObject {
        AstObject::Null
    }
}
