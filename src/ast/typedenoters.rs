//! type-denoter asts

use super::{Ast, AstObject, AstVisitor, CommonState};

#[derive(Debug)]
pub enum TypeDenoter {}

impl PartialEq for TypeDenoter {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for TypeDenoter {}
