//! type-denoter asts

use super::primitives::{Identifier, IntegerLiteral};
use super::scanner::SourcePosition;
use super::{Ast, AstObject, AstVisitor, CommonState};
use std::fmt;

#[derive(Debug)]
pub enum TypeDenoter {
    AnyTypeDenoter(AnyTypeDenoterState),
    ArrayTypeDenoter(ArrayTypeDenoterState),
    BoolTypeDenoter(BoolTypeDenoterState),
    CharTypeDenoter(CharTypeDenoterState),
    ErrorTypeDenoter(ErrorTypeDenoterState),
    IntTypeDenoter(IntTypeDenoterState),
    RecordTypeDenoter(RecordTypeDenoterState),
    SimpleTypeDenoter(SimpleTypeDenoterState),
}

impl PartialEq for TypeDenoter {
    fn eq(&self, other: &Self) -> bool {
        use TypeDenoter::*;
        match (self, other) {
            (AnyTypeDenoter(_), AnyTypeDenoter(_)) => true,
            (ArrayTypeDenoter(ref atd1), ArrayTypeDenoter(ref atd2)) => atd1 == atd2,
            (BoolTypeDenoter(_), BoolTypeDenoter(_)) => true,
            (CharTypeDenoter(_), CharTypeDenoter(_)) => true,
            (ErrorTypeDenoter(_), ErrorTypeDenoter(_)) => true,
            (IntTypeDenoter(_), IntTypeDenoter(_)) => true,
            (RecordTypeDenoter(ref rtd1), RecordTypeDenoter(ref rtd2)) => rtd1 == rtd2,
            (SimpleTypeDenoter(ref std1), SimpleTypeDenoter(std2)) => std1 == std2,
            (_, _) => false,
        }
    }
}

impl Eq for TypeDenoter {}

impl fmt::Display for TypeDenoter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TypeDenoter::*;

        match *self {
            AnyTypeDenoter(ref td) => write!(f, "AnyTypeDenoter({})", td),
            ArrayTypeDenoter(ref td) => write!(f, "ArrayTypeDenoter({})", td),
            BoolTypeDenoter(ref td) => write!(f, "BoolTypeDenoter({})", td),
            CharTypeDenoter(ref td) => write!(f, "CharTypeDenoter({})", td),
            ErrorTypeDenoter(ref td) => write!(f, "ErrorTypeDenoter({})", td),
            IntTypeDenoter(ref td) => write!(f, "IntTypeDenoter({})", td),
            RecordTypeDenoter(ref td) => write!(f, "RecordTypeDenoter({})", td),
            SimpleTypeDenoter(ref td) => write!(f, "SimpleTypeDenoter({})", td),
        }
    }
}

impl Ast for TypeDenoter {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        use TypeDenoter::*;

        match *self {
            AnyTypeDenoter(ref mut anytd) => anytd.accept(visitor, arg),
            ArrayTypeDenoter(ref mut atd) => atd.accept(visitor, arg),
            BoolTypeDenoter(ref mut btd) => btd.accept(visitor, arg),
            CharTypeDenoter(ref mut ctd) => ctd.accept(visitor, arg),
            ErrorTypeDenoter(ref mut etd) => etd.accept(visitor, arg),
            IntTypeDenoter(ref mut itd) => itd.accept(visitor, arg),
            RecordTypeDenoter(ref mut rtd) => rtd.accept(visitor, arg),
            SimpleTypeDenoter(ref mut std) => std.accept(visitor, arg),
        }
    }
}

#[derive(Debug)]
pub struct BoolTypeDenoterState {
    common_state: CommonState,
}

impl BoolTypeDenoterState {
    pub fn new() -> Self {
        BoolTypeDenoterState {
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(position: SourcePosition) -> Self {
        let mut btd = BoolTypeDenoterState::new();
        btd.common_state.position = position;
        btd
    }
}

impl PartialEq for BoolTypeDenoterState {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for BoolTypeDenoterState {}

impl fmt::Display for BoolTypeDenoterState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BoolTypeDenoterState::new()",)
    }
}

impl Ast for BoolTypeDenoterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_bool_type_denoter(self, arg)
    }
}
#[derive(Debug)]
pub struct CharTypeDenoterState {
    common_state: CommonState,
}

impl CharTypeDenoterState {
    pub fn new() -> Self {
        CharTypeDenoterState {
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(position: SourcePosition) -> Self {
        let mut btd = CharTypeDenoterState::new();
        btd.common_state.position = position;
        btd
    }
}

impl PartialEq for CharTypeDenoterState {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for CharTypeDenoterState {}

impl fmt::Display for CharTypeDenoterState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CharTypeDenoterState::new()",)
    }
}

impl Ast for CharTypeDenoterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_char_type_denoter(self, arg)
    }
}

#[derive(Debug)]
pub struct IntTypeDenoterState {
    common_state: CommonState,
}

impl IntTypeDenoterState {
    pub fn new() -> Self {
        IntTypeDenoterState {
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(position: SourcePosition) -> Self {
        let mut btd = IntTypeDenoterState::new();
        btd.common_state.position = position;
        btd
    }
}

impl PartialEq for IntTypeDenoterState {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for IntTypeDenoterState {}

impl fmt::Display for IntTypeDenoterState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IntTypeDenoterState::new()")
    }
}

impl Ast for IntTypeDenoterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_int_type_denoter(self, arg)
    }
}
#[derive(Debug)]
pub struct AnyTypeDenoterState {
    common_state: CommonState,
}

impl AnyTypeDenoterState {
    pub fn new() -> Self {
        AnyTypeDenoterState {
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(position: SourcePosition) -> Self {
        let mut btd = AnyTypeDenoterState::new();
        btd.common_state.position = position;
        btd
    }
}

impl PartialEq for AnyTypeDenoterState {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for AnyTypeDenoterState {}

impl fmt::Display for AnyTypeDenoterState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AnyTypeDenoterState::new()")
    }
}

impl Ast for AnyTypeDenoterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_any_type_denoter(self, arg)
    }
}
#[derive(Debug)]
pub struct ErrorTypeDenoterState {
    common_state: CommonState,
}

impl ErrorTypeDenoterState {
    pub fn new() -> Self {
        ErrorTypeDenoterState {
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(position: SourcePosition) -> Self {
        let mut btd = ErrorTypeDenoterState::new();
        btd.common_state.position = position;
        btd
    }
}

impl PartialEq for ErrorTypeDenoterState {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for ErrorTypeDenoterState {}

impl fmt::Display for ErrorTypeDenoterState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ErrorTypeDenoterState::new()")
    }
}

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

impl fmt::Display for SimpleTypeDenoterState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SimpleTypeDenoterState::new({})", self.id,)
    }
}

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

impl fmt::Display for ArrayTypeDenoterState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ArrayTypeDenoterState::new({}, {})", self.il, self.td)
    }
}

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

impl fmt::Display for RecordTypeDenoterState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RecordTypeDenoterState::new({})", self.ftd)
    }
}

impl Ast for RecordTypeDenoterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_record_type_denoter(self, arg)
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

impl Eq for FieldTypeDenoter {}

impl fmt::Display for FieldTypeDenoter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use FieldTypeDenoter::*;

        match *self {
            SingleFieldTypeDenoter(ref ftd) => write!(f, "SingleFieldTypeDenoter({})", ftd),
            MultipleFieldTypeDenoter(ref ftd) => write!(f, "MultipleFieldTypeDenoter({})", ftd),
        }
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

impl fmt::Display for SingleFieldTypeDenoterState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SingleFieldTypeDenoterState::new({}, {})",
            self.id, self.td
        )
    }
}

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

impl fmt::Display for MultipleFieldTypeDenoterState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MultipleFieldTypeDenoterState::new({}, {}, {})",
            self.id, self.td, self.ftd
        )
    }
}

impl Ast for MultipleFieldTypeDenoterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_multiple_field_type_denoter(self, arg)
    }
}
