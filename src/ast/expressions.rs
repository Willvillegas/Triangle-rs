//! expression asts

use super::CommonState;

pub enum Expression {
    IntegerExpression(Box<IntegerExpressionState>),
    CharacterExpression(Box<CharacterExpressionState>),
    VnameExpression(Box<VnameExpressionState>),
    CallExpression(Box<CallExpressionState>),
    IfExpression(Box<IfExpressionState>),
    LetExpression(Box<LetExpressionState>),
    UnaryExpression(Box<UnaryExpressionState>),
    BinaryExpression(Box<BinaryExpressionState>),
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Eq for Expression {}

pub struct IntegerExpressionState {}

pub struct CharacterExpressionState {}

pub struct VnameExpressionState {}

pub struct CallExpressionState {}

pub struct IfExpressionState {}

pub struct LetExpressionState {}

pub struct UnaryExpressionState {}

pub struct BinaryExpressionState {}
