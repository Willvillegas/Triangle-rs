//! primitive/terminal asts

use super::declarations::Declaration;
use super::typedenoters::TypeDenoter;
use super::CommonState;

#[derive(Debug)]
pub struct IntegerLiteral {
    pub spelling: String,
    pub common_state: CommonState,
}

impl IntegerLiteral {
    pub fn new(spelling: &str) -> Self {
        IntegerLiteral {
            spelling: String::from(spelling),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for IntegerLiteral {
    fn eq(&self, other: &Self) -> bool {
        self.spelling == other.spelling
    }
}

impl Eq for IntegerLiteral {}

#[derive(Debug)]
pub struct CharacterLiteral {
    pub spelling: String,
    pub common_state: CommonState,
}

impl CharacterLiteral {
    pub fn new(spelling: &str) -> Self {
        CharacterLiteral {
            spelling: String::from(spelling),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for CharacterLiteral {
    fn eq(&self, other: &Self) -> bool {
        self.spelling == other.spelling
    }
}

impl Eq for CharacterLiteral {}

#[derive(Debug)]
pub struct Identifier {
    pub spelling: String,
    pub td: Option<Box<TypeDenoter>>,
    pub decl: Option<Box<Declaration>>,
    pub common_state: CommonState,
}

impl Identifier {
    pub fn new(spelling: &str) -> Self {
        Identifier {
            spelling: String::from(spelling),
            td: None,
            decl: None,
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

#[derive(Debug)]
pub struct Operator {
    pub spelling: String,
    pub decl: Option<Box<Declaration>>,
    pub common_state: CommonState,
}

impl Operator {
    pub fn new(spelling: &str) -> Self {
        Operator {
            spelling: String::from(spelling),
            decl: None,
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        self.spelling == other.spelling
    }
}

impl Eq for Operator {}
