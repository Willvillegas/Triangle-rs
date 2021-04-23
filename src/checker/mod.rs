//! The Checker module.

use crate::ast::aggregates::*;
use crate::ast::commands::*;
use crate::ast::declarations::*;
use crate::ast::expressions::*;
use crate::ast::parameters::*;
use crate::ast::primitives::*;
use crate::ast::typedenoters::*;
use crate::ast::vnames::*;
use crate::ast::*;

pub struct Checker {}

impl Checker {
    pub fn new() -> Self {
        Checker {}
    }
}

impl AstVisitor for Checker {
    fn visit_program(&self, program: &mut Program, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_empty_command(&self, cmd: &mut EmptyCommandState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_assign_command(&self, cmd: &mut AssignCommandState, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_call_command(&self, cmd: &mut CallCommandState, arg: AstObject) -> AstObject {
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

    fn visit_identifier(&self, id: Identifier, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_integer_literal(&self, il: IntegerLiteral, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_character_literal(&self, cl: CharacterLiteral, arg: AstObject) -> AstObject {
        AstObject::Null
    }

    fn visit_operator(&self, op: Operator, arg: AstObject) -> AstObject {
        AstObject::Null
    }
}