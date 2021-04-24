//! primitive/terminal asts

use super::declarations::Declaration;
use super::typedenoters::TypeDenoter;
use super::CommonState;
use crate::scanner::SourcePosition;
use std::fmt;

#[derive(Debug, Clone)]
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

    pub fn new_with_position(spelling: &str, position: SourcePosition) -> Self {
        let mut il = IntegerLiteral::new(spelling);
        il.common_state.position = position;
        il
    }
}

impl PartialEq for IntegerLiteral {
    fn eq(&self, other: &Self) -> bool {
        self.spelling == other.spelling
    }
}

impl Eq for IntegerLiteral {}

impl fmt::Display for IntegerLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IntegerLiteral::new({:?})", self.spelling)
    }
}

#[derive(Debug, Clone)]
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

    pub fn new_with_position(spelling: &str, position: SourcePosition) -> Self {
        let mut cl = CharacterLiteral::new(spelling);
        cl.common_state.position = position;
        cl
    }
}

impl PartialEq for CharacterLiteral {
    fn eq(&self, other: &Self) -> bool {
        self.spelling == other.spelling
    }
}

impl Eq for CharacterLiteral {}

impl fmt::Display for CharacterLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CharacterLiteral::new({:?})", self.spelling)
    }
}

#[derive(Debug, Clone)]
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

    pub fn new_with_position(spelling: &str, position: SourcePosition) -> Self {
        let mut id = Identifier::new(spelling);
        id.common_state.position = position;
        id
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.spelling == other.spelling
    }
}

impl Eq for Identifier {}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Identifier::new({:?})", self.spelling)
    }
}

#[derive(Debug, Clone)]
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

    pub fn new_with_position(spelling: &str, position: SourcePosition) -> Self {
        let mut op = Operator::new(spelling);
        op.common_state.position = position;
        op
    }
}

impl PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        self.spelling == other.spelling
    }
}

impl Eq for Operator {}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Operator::new({:?})", self.spelling)
    }
}
