//! type-denoter asts

use super::primitives::{Identifier, IntegerLiteral};
use super::{Ast, AstObject, AstVisitor, CommonState};

#[derive(Debug)]
pub enum TypeDenoter {
    BoolTypeDenoter,
    IntTypeDenoter,
    CharTypeDenoter,
    AnyTypeDenoter,
    ErrorTypeDenoter,
    SimpleTypeDenoter(SimpleTypeDenoterState),
    ArrayTypeDenoter(ArrayTypeDenoterState),
    RecordTypeDenoter(RecordTypeDenoterState),
}

impl Ast for TypeDenoter {
    fn accept(&mut self, visitor: &dyn AstVisitor) -> AstObject {
        visitor.visit_type_denoter(self)
    }
}

impl PartialEq for TypeDenoter {
    fn eq(&self, other: &Self) -> bool {
        use TypeDenoter::*;
        match (self, other) {
            (BoolTypeDenoter, BoolTypeDenoter) => true,
            (IntTypeDenoter, IntTypeDenoter) => true,
            (CharTypeDenoter, CharTypeDenoter) => true,
            (AnyTypeDenoter, AnyTypeDenoter) => true,
            (ErrorTypeDenoter, ErrorTypeDenoter) => true,
            (SimpleTypeDenoter(ref std1), SimpleTypeDenoter(std2)) => std1 == std2,
            (ArrayTypeDenoter(ref atd1), ArrayTypeDenoter(ref atd2)) => atd1 == atd2,
            (RecordTypeDenoter(ref rtd1), RecordTypeDenoter(ref rtd2)) => rtd1 == rtd2,
            (_, _) => false,
        }
    }
}

impl Eq for TypeDenoter {}

#[derive(Debug)]
pub enum FieldTypeDenoter {
    SingleFieldTypeDenoter(SingleFieldTypeDenoterState),
    MultipleFieldTypeDenoter(MultipleFieldTypeDenoterState),
}

impl PartialEq for FieldTypeDenoter {
    fn eq(&self, other: &Self) -> bool {
        use FieldTypeDenoter::*;
        match (self, other) {
            (SingleFieldTypeDenoter(ref sftd1), SingleFieldTypeDenoter(ref sftd2)) => {
                sftd1 == sftd2
            }
            (MultipleFieldTypeDenoter(ref mftd1), MultipleFieldTypeDenoter(ref mftd2)) => {
                mftd1 == mftd2
            }
            (_, __) => false,
        }
    }
}

#[derive(Debug)]
pub struct SimpleTypeDenoterState {
    id: Identifier,
    common_state: CommonState,
}

impl SimpleTypeDenoterState {
    pub fn new(id: Identifier) -> Self {
        SimpleTypeDenoterState {
            id: id,
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for SimpleTypeDenoterState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for SimpleTypeDenoterState {}

#[derive(Debug)]
pub struct ArrayTypeDenoterState {
    il: IntegerLiteral,
    td: Box<TypeDenoter>,
    common_state: CommonState,
}

impl ArrayTypeDenoterState {
    pub fn new(il: IntegerLiteral, td: TypeDenoter) -> Self {
        ArrayTypeDenoterState {
            il: il,
            td: Box::new(td),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for ArrayTypeDenoterState {
    fn eq(&self, other: &Self) -> bool {
        self.il == other.il && self.td == other.td
    }
}

impl Eq for ArrayTypeDenoterState {}

#[derive(Debug)]
pub struct RecordTypeDenoterState {
    ftd: Box<FieldTypeDenoter>,
    common_state: CommonState,
}

impl RecordTypeDenoterState {
    pub fn new(ftd: FieldTypeDenoter) -> Self {
        RecordTypeDenoterState {
            ftd: Box::new(ftd),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for RecordTypeDenoterState {
    fn eq(&self, other: &Self) -> bool {
        self.ftd == other.ftd
    }
}

impl Eq for RecordTypeDenoterState {}

#[derive(Debug)]
pub struct SingleFieldTypeDenoterState {
    id: Identifier,
    td: Box<TypeDenoter>,
    common_state: CommonState,
}

impl SingleFieldTypeDenoterState {
    pub fn new(id: Identifier, td: TypeDenoter) -> Self {
        SingleFieldTypeDenoterState {
            id: id,
            td: Box::new(td),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for SingleFieldTypeDenoterState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.td == other.td
    }
}

impl Eq for SingleFieldTypeDenoterState {}

#[derive(Debug)]
pub struct MultipleFieldTypeDenoterState {
    id: Identifier,
    td: Box<TypeDenoter>,
    ftd: Box<FieldTypeDenoter>,
    common_state: CommonState,
}

impl MultipleFieldTypeDenoterState {
    pub fn new(id: Identifier, td: TypeDenoter, ftd: FieldTypeDenoter) -> Self {
        MultipleFieldTypeDenoterState {
            id: id,
            td: Box::new(td),
            ftd: Box::new(ftd),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for MultipleFieldTypeDenoterState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.td == other.td && self.ftd == other.ftd
    }
}

impl Eq for MultipleFieldTypeDenoterState {}
