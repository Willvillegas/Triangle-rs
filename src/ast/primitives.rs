//! primitive/terminal asts

use super::declarations::Declaration;
use super::typedenoters::TypeDenoter;
use super::CommonState;

pub struct IntegerLiteral {}

impl IntegerLiteral {}

pub struct CharacterLiteral {}

impl CharacterLiteral {}

// TODO: factor out the type denoter and decl
pub struct Identifier {
    pub spelling: String,
    pub td: TypeDenoter,
    pub decl: Declaration,
    pub common_state: CommonState,
}

impl Identifier {
    pub fn new(spelling: &str, td: TypeDenoter, decl: Declaration) -> Self {
        Identifier {
            spelling: String::from(spelling),
            td: td,
            decl: decl,
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.spelling == other.spelling
    }
}

impl Eq for Identifier {}

pub struct Operator {}

impl Operator {}
