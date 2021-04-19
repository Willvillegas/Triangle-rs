//! type-denoter asts

use super::primitives::{Identifier, IntegerLiteral};
use super::scanner::SourcePosition;
use super::{Ast, AstObject, AstVisitor, CommonState};

#[derive(Debug)]
pub enum TypeDenoter {
    BoolTypeDenoter(BoolTypeDenoterState),
    IntTypeDenoter(IntTypeDenoterState),
    CharTypeDenoter(CharTypeDenoterState),
    AnyTypeDenoter(AnyTypeDenoterState),
    ErrorTypeDenoter(ErrorTypeDenoterState),
    SimpleTypeDenoter(SimpleTypeDenoterState),
    ArrayTypeDenoter(ArrayTypeDenoterState),
    RecordTypeDenoter(RecordTypeDenoterState),
}

impl PartialEq for TypeDenoter {
    fn eq(&self, other: &Self) -> bool {
        use TypeDenoter::*;
        match (self, other) {
            (BoolTypeDenoter(_), BoolTypeDenoter(_)) => true,
            (IntTypeDenoter(_), IntTypeDenoter(_)) => true,
            (CharTypeDenoter(_), CharTypeDenoter(_)) => true,
            (AnyTypeDenoter(_), AnyTypeDenoter(_)) => true,
            (ErrorTypeDenoter(_), ErrorTypeDenoter(_)) => true,
            (SimpleTypeDenoter(ref std1), SimpleTypeDenoter(std2)) => std1 == std2,
            (ArrayTypeDenoter(ref atd1), ArrayTypeDenoter(ref atd2)) => atd1 == atd2,
            (RecordTypeDenoter(ref rtd1), RecordTypeDenoter(ref rtd2)) => rtd1 == rtd2,
            (_, _) => false,
        }
    }
}

impl Eq for TypeDenoter {}

impl Ast for TypeDenoter {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        use TypeDenoter::*;

        match *self {
            BoolTypeDenoter(ref mut btd) => btd.accept(visitor, arg),
            CharTypeDenoter(ref mut ctd) => ctd.accept(visitor, arg),
            IntTypeDenoter(ref mut itd) => itd.accept(visitor, arg),
            AnyTypeDenoter(ref mut anytd) => anytd.accept(visitor, arg),
            ErrorTypeDenoter(ref mut etd) => etd.accept(visitor, arg),
            SimpleTypeDenoter(ref mut std) => std.accept(visitor, arg),
            ArrayTypeDenoter(ref mut atd) => atd.accept(visitor, arg),
            RecordTypeDenoter(ref mut rtd) => rtd.accept(visitor, arg),
        }
    }
}

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

#[derive(Debug, PartialEq, Eq)]
pub struct BoolTypeDenoterState;

impl Ast for BoolTypeDenoterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_bool_type_denoter(self, arg)
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct CharTypeDenoterState;

impl Ast for CharTypeDenoterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_char_type_denoter(self, arg)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct IntTypeDenoterState;

impl Ast for IntTypeDenoterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_int_type_denoter(self, arg)
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct AnyTypeDenoterState;

impl Ast for AnyTypeDenoterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_any_type_denoter(self, arg)
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct ErrorTypeDenoterState;

impl Ast for ErrorTypeDenoterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_error_type_denoter(self, arg)
    }
}
#[derive(Debug)]
pub struct SimpleTypeDenoterState {
    pub id: Identifier,
    pub common_state: CommonState,
}

impl SimpleTypeDenoterState {
    pub fn new(id: Identifier) -> Self {
        SimpleTypeDenoterState {
            id: id,
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(id: Identifier, position: SourcePosition) -> Self {
        let mut std = SimpleTypeDenoterState::new(id);
        std.common_state.position = position;
        std
    }
}

impl PartialEq for SimpleTypeDenoterState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for SimpleTypeDenoterState {}

impl Ast for SimpleTypeDenoterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_simple_type_denoter(self, arg)
    }
}
#[derive(Debug)]
pub struct ArrayTypeDenoterState {
    pub il: IntegerLiteral,
    pub td: Box<TypeDenoter>,
    pub common_state: CommonState,
}

impl ArrayTypeDenoterState {
    pub fn new(il: IntegerLiteral, td: TypeDenoter) -> Self {
        ArrayTypeDenoterState {
            il: il,
            td: Box::new(td),
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(
        il: IntegerLiteral,
        td: TypeDenoter,
        position: SourcePosition,
    ) -> Self {
        let mut atd = ArrayTypeDenoterState::new(il, td);
        atd.common_state.position = position;
        atd
    }
}

impl PartialEq for ArrayTypeDenoterState {
    fn eq(&self, other: &Self) -> bool {
        self.il == other.il && self.td == other.td
    }
}

impl Eq for ArrayTypeDenoterState {}

impl Ast for ArrayTypeDenoterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_array_type_denoter(self, arg)
    }
}

#[derive(Debug)]
pub struct RecordTypeDenoterState {
    pub ftd: Box<FieldTypeDenoter>,
    pub common_state: CommonState,
}

impl RecordTypeDenoterState {
    pub fn new(ftd: FieldTypeDenoter) -> Self {
        RecordTypeDenoterState {
            ftd: Box::new(ftd),
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(ftd: FieldTypeDenoter, position: SourcePosition) -> Self {
        let mut rtd = RecordTypeDenoterState::new(ftd);
        rtd.common_state.position = position;
        rtd
    }
}

impl PartialEq for RecordTypeDenoterState {
    fn eq(&self, other: &Self) -> bool {
        self.ftd == other.ftd
    }
}

impl Eq for RecordTypeDenoterState {}

impl Ast for RecordTypeDenoterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_record_type_denoter(self, arg)
    }
}

#[derive(Debug)]
pub struct SingleFieldTypeDenoterState {
    pub id: Identifier,
    pub td: Box<TypeDenoter>,
    pub common_state: CommonState,
}

impl SingleFieldTypeDenoterState {
    pub fn new(id: Identifier, td: TypeDenoter) -> Self {
        SingleFieldTypeDenoterState {
            id: id,
            td: Box::new(td),
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(id: Identifier, td: TypeDenoter, position: SourcePosition) -> Self {
        let mut sftd = SingleFieldTypeDenoterState::new(id, td);
        sftd.common_state.position = position;
        sftd
    }
}

impl PartialEq for SingleFieldTypeDenoterState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.td == other.td
    }
}

impl Eq for SingleFieldTypeDenoterState {}

impl Ast for SingleFieldTypeDenoterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_single_field_type_denoter(self, arg)
    }
}

#[derive(Debug)]
pub struct MultipleFieldTypeDenoterState {
    pub id: Identifier,
    pub td: Box<TypeDenoter>,
    pub ftd: Box<FieldTypeDenoter>,
    pub common_state: CommonState,
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

    pub fn new_with_position(
        id: Identifier,
        td: TypeDenoter,
        ftd: FieldTypeDenoter,
        position: SourcePosition,
    ) -> Self {
        let mut mftd = MultipleFieldTypeDenoterState::new(id, td, ftd);
        mftd.common_state.position = position;
        mftd
    }
}

impl PartialEq for MultipleFieldTypeDenoterState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.td == other.td && self.ftd == other.ftd
    }
}

impl Eq for MultipleFieldTypeDenoterState {}

impl Ast for MultipleFieldTypeDenoterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_multiple_field_type_denoter(self, arg)
    }
}
