//! parameter asts

use super::expressions::Expression;
use super::primitives::Identifier;
use super::typedenoters::TypeDenoter;
use super::vnames::Vname;
use super::{Ast, AstObject, AstVisitor, CommonState};
use crate::scanner::SourcePosition;
use std::fmt;

#[derive(Debug, Clone)]
pub enum FormalParameterSequence {
    EmptyFormalParameterSequence(EmptyFormalParameterSequenceState),
    SingleFormalParameterSequence(SingleFormalParameterSequenceState),
    MultipleFormalParameterSequence(MultipleFormalParameterSequenceState),
}

impl PartialEq for FormalParameterSequence {
    fn eq(&self, other: &Self) -> bool {
        use FormalParameterSequence::*;

        match (self, other) {
            (EmptyFormalParameterSequence(_), EmptyFormalParameterSequence(__)) => true,
            (SingleFormalParameterSequence(ref sfp1), SingleFormalParameterSequence(ref sfp2)) => {
                sfp1 == sfp2
            }
            (MultipleFormalParameterSequence(ref mfp1), MultipleFormalParameterSequence(mfp2)) => {
                mfp1 == mfp2
            }
            (_, __) => false,
        }
    }
}

impl Eq for FormalParameterSequence {}

impl fmt::Display for FormalParameterSequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use FormalParameterSequence::*;

        match *self {
            EmptyFormalParameterSequence(ref fps) => {
                write!(f, "EmptyFormalParameterSequence({})", fps)
            }
            SingleFormalParameterSequence(ref fps) => {
                write!(f, "SingleFormalParameterSequence({})", fps)
            }
            MultipleFormalParameterSequence(ref fps) => {
                write!(f, "MultipleFormalParameterSequence({})", fps)
            }
        }
    }
}

impl Ast for FormalParameterSequence {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        use FormalParameterSequence::*;

        match *self {
            EmptyFormalParameterSequence(ref mut efps) => efps.accept(visitor, arg),
            SingleFormalParameterSequence(ref mut sfps) => sfps.accept(visitor, arg),
            MultipleFormalParameterSequence(ref mut mfps) => mfps.accept(visitor, arg),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EmptyFormalParameterSequenceState {
    pub common_state: CommonState,
}

impl EmptyFormalParameterSequenceState {
    pub fn new() -> Self {
        EmptyFormalParameterSequenceState {
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(position: SourcePosition) -> Self {
        let mut efps = EmptyFormalParameterSequenceState::new();
        efps.common_state.position = position;
        efps
    }
}

impl PartialEq for EmptyFormalParameterSequenceState {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for EmptyFormalParameterSequenceState {}

impl fmt::Display for EmptyFormalParameterSequenceState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EmptyFormalParameterSequenceState::new()")
    }
}

impl Ast for EmptyFormalParameterSequenceState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_empty_formal_parameter_sequence(self, arg)
    }
}

#[derive(Debug, Clone)]
pub struct SingleFormalParameterSequenceState {
    pub fp: Box<FormalParameter>,
    pub common_state: CommonState,
}

impl SingleFormalParameterSequenceState {
    pub fn new(fp: FormalParameter) -> Self {
        SingleFormalParameterSequenceState {
            fp: Box::new(fp),
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(fp: FormalParameter, position: SourcePosition) -> Self {
        let mut sfps = SingleFormalParameterSequenceState::new(fp);
        sfps.common_state.position = position;
        sfps
    }
}

impl PartialEq for SingleFormalParameterSequenceState {
    fn eq(&self, other: &Self) -> bool {
        self.fp == other.fp
    }
}

impl Eq for SingleFormalParameterSequenceState {}

impl fmt::Display for SingleFormalParameterSequenceState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SingleFormalParameterSequenceState::new({})", self.fp)
    }
}

impl Ast for SingleFormalParameterSequenceState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_single_formal_parameter_sequence(self, arg)
    }
}

#[derive(Debug, Clone)]
pub struct MultipleFormalParameterSequenceState {
    pub fp: Box<FormalParameter>,
    pub fps: Box<FormalParameterSequence>,
    pub common_state: CommonState,
}

impl MultipleFormalParameterSequenceState {
    pub fn new(fp: FormalParameter, fps: FormalParameterSequence) -> Self {
        MultipleFormalParameterSequenceState {
            fp: Box::new(fp),
            fps: Box::new(fps),
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(
        fp: FormalParameter,
        fps: FormalParameterSequence,
        position: SourcePosition,
    ) -> Self {
        let mut mfps = MultipleFormalParameterSequenceState::new(fp, fps);
        mfps.common_state.position = position;
        mfps
    }
}

impl PartialEq for MultipleFormalParameterSequenceState {
    fn eq(&self, other: &Self) -> bool {
        self.fp == other.fp && self.fps == other.fps
    }
}

impl Eq for MultipleFormalParameterSequenceState {}

impl fmt::Display for MultipleFormalParameterSequenceState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MultipleFormalParameterSequenceState::new({}, {})",
            self.fp, self.fps
        )
    }
}

impl Ast for MultipleFormalParameterSequenceState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_multiple_formal_parameter_sequence(self, arg)
    }
}

#[derive(Debug, Clone)]
pub enum FormalParameter {
    ConstFormalParameter(ConstFormalParameterState),
    FuncFormalParameter(FuncFormalParameterState),
    ProcFormalParameter(ProcFormalParameterState),
    VarFormalParameter(VarFormalParameterState),
}

impl Ast for FormalParameter {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        use FormalParameter::*;

        match *self {
            ConstFormalParameter(ref mut cfp) => cfp.accept(visitor, arg),
            FuncFormalParameter(ref mut ffp) => ffp.accept(visitor, arg),
            ProcFormalParameter(ref mut pfp) => pfp.accept(visitor, arg),
            VarFormalParameter(ref mut vfp) => vfp.accept(visitor, arg),
        }
    }
}

impl PartialEq for FormalParameter {
    fn eq(&self, other: &Self) -> bool {
        use FormalParameter::*;

        match (self, other) {
            (ConstFormalParameter(ref cfp1), ConstFormalParameter(ref cfp2)) => cfp1 == cfp2,
            (FuncFormalParameter(ref ffp1), FuncFormalParameter(ref ffp2)) => ffp1 == ffp2,
            (ProcFormalParameter(ref pfp1), ProcFormalParameter(ref pfp2)) => pfp1 == pfp2,
            (VarFormalParameter(ref vfp1), VarFormalParameter(ref vfp2)) => vfp1 == vfp2,
            (_, _) => false,
        }
    }
}

impl Eq for FormalParameter {}

impl fmt::Display for FormalParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use FormalParameter::*;

        match *self {
            ConstFormalParameter(ref fp) => write!(f, "ConstFormalParameter({})", fp),
            FuncFormalParameter(ref fp) => write!(f, "FuncFormalParameter({})", fp),
            ProcFormalParameter(ref fp) => write!(f, "ProcFormalParameter({})", fp),
            VarFormalParameter(ref fp) => write!(f, "VarFormalParameter({})", fp),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VarFormalParameterState {
    pub id: Identifier,
    pub td: Box<TypeDenoter>,
    pub common_state: CommonState,
}

impl VarFormalParameterState {
    pub fn new(id: Identifier, td: TypeDenoter) -> Self {
        VarFormalParameterState {
            id: id,
            td: Box::new(td),
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(id: Identifier, td: TypeDenoter, position: SourcePosition) -> Self {
        let mut vfp = VarFormalParameterState::new(id, td);
        vfp.common_state.position = position;
        vfp
    }
}

impl PartialEq for VarFormalParameterState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.td == other.td
    }
}

impl Eq for VarFormalParameterState {}

impl fmt::Display for VarFormalParameterState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VarFormalParameterState::new({}, {})", self.id, self.td)
    }
}

impl Ast for VarFormalParameterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_var_formal_parameter(self, arg)
    }
}

#[derive(Debug, Clone)]
pub struct ConstFormalParameterState {
    pub id: Identifier,
    pub td: Box<TypeDenoter>,
    pub common_state: CommonState,
}

impl ConstFormalParameterState {
    pub fn new(id: Identifier, td: TypeDenoter) -> Self {
        ConstFormalParameterState {
            id: id,
            td: Box::new(td),
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(id: Identifier, td: TypeDenoter, position: SourcePosition) -> Self {
        let mut cfp = ConstFormalParameterState::new(id, td);
        cfp.common_state.position = position;
        cfp
    }
}

impl PartialEq for ConstFormalParameterState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.td == other.td
    }
}

impl Eq for ConstFormalParameterState {}

impl fmt::Display for ConstFormalParameterState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ConstFormalParameterState::new({}, {})",
            self.id, self.td
        )
    }
}

impl Ast for ConstFormalParameterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_const_formal_parameter(self, arg)
    }
}

#[derive(Debug, Clone)]
pub struct ProcFormalParameterState {
    pub id: Identifier,
    pub fps: Box<FormalParameterSequence>,
    pub common_state: CommonState,
}

impl ProcFormalParameterState {
    pub fn new(id: Identifier, fps: FormalParameterSequence) -> Self {
        ProcFormalParameterState {
            id: id,
            fps: Box::new(fps),
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(
        id: Identifier,
        fps: FormalParameterSequence,
        position: SourcePosition,
    ) -> Self {
        let mut pfp = ProcFormalParameterState::new(id, fps);
        pfp.common_state.position = position;
        pfp
    }
}

impl PartialEq for ProcFormalParameterState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.fps == other.fps
    }
}

impl Eq for ProcFormalParameterState {}

impl fmt::Display for ProcFormalParameterState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ProcFormalParameterState::new({}, {})",
            self.id, self.fps
        )
    }
}

impl Ast for ProcFormalParameterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_proc_formal_parameter(self, arg)
    }
}

#[derive(Debug, Clone)]
pub struct FuncFormalParameterState {
    pub id: Identifier,
    pub fps: Box<FormalParameterSequence>,
    pub td: Box<TypeDenoter>,
    pub common_state: CommonState,
}

impl FuncFormalParameterState {
    pub fn new(id: Identifier, fps: FormalParameterSequence, td: TypeDenoter) -> Self {
        FuncFormalParameterState {
            id: id,
            fps: Box::new(fps),
            td: Box::new(td),
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(
        id: Identifier,
        fps: FormalParameterSequence,
        td: TypeDenoter,
        position: SourcePosition,
    ) -> Self {
        let mut ffp = FuncFormalParameterState::new(id, fps, td);
        ffp.common_state.position = position;
        ffp
    }
}

impl PartialEq for FuncFormalParameterState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.fps == other.fps && self.td == other.td
    }
}

impl Eq for FuncFormalParameterState {}

impl fmt::Display for FuncFormalParameterState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FuncFormalParameterState::new({}, {}, {})",
            self.id, self.fps, self.td
        )
    }
}

impl Ast for FuncFormalParameterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_func_formal_parameter(self, arg)
    }
}

#[derive(Debug, Clone)]
pub enum ActualParameterSequence {
    EmptyActualParameterSequence(EmptyActualParameterSequenceState),
    SingleActualParameterSequence(SingleActualParameterSequenceState),
    MultipleActualParameterSequence(MultipleActualParameterSequenceState),
}

impl PartialEq for ActualParameterSequence {
    fn eq(&self, other: &Self) -> bool {
        use ActualParameterSequence::*;

        match (self, other) {
            (EmptyActualParameterSequence(_), EmptyActualParameterSequence(_)) => true,
            (
                SingleActualParameterSequence(ref saps1),
                SingleActualParameterSequence(ref saps2),
            ) => saps1 == saps2,
            (
                MultipleActualParameterSequence(ref maps1),
                MultipleActualParameterSequence(ref maps2),
            ) => maps1 == maps2,
            (_, _) => false,
        }
    }
}

impl Eq for ActualParameterSequence {}

impl fmt::Display for ActualParameterSequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ActualParameterSequence::*;

        match *self {
            EmptyActualParameterSequence(ref aps) => {
                write!(f, "EmptyActualParameterSequence({})", aps)
            }
            SingleActualParameterSequence(ref aps) => {
                write!(f, "SingleActualParameterSequence({})", aps)
            }
            MultipleActualParameterSequence(ref aps) => {
                write!(f, "MultipleActualParameterSequence({})", aps)
            }
        }
    }
}

impl Ast for ActualParameterSequence {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        use ActualParameterSequence::*;

        match *self {
            EmptyActualParameterSequence(ref mut eaps) => eaps.accept(visitor, arg),
            SingleActualParameterSequence(ref mut saps) => saps.accept(visitor, arg),
            MultipleActualParameterSequence(ref mut maps) => maps.accept(visitor, arg),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EmptyActualParameterSequenceState {
    pub common_state: CommonState,
}

impl EmptyActualParameterSequenceState {
    pub fn new() -> Self {
        EmptyActualParameterSequenceState {
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(position: SourcePosition) -> Self {
        let mut eaps = EmptyActualParameterSequenceState::new();
        eaps.common_state.position = position;
        eaps
    }
}

impl PartialEq for EmptyActualParameterSequenceState {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for EmptyActualParameterSequenceState {}

impl fmt::Display for EmptyActualParameterSequenceState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EmptyActualParameterSequenceState::new()")
    }
}

impl Ast for EmptyActualParameterSequenceState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_empty_actual_parameter_sequence(self, arg)
    }
}

#[derive(Debug, Clone)]
pub struct SingleActualParameterSequenceState {
    pub ap: Box<ActualParameter>,
    pub common_state: CommonState,
}

impl SingleActualParameterSequenceState {
    pub fn new(ap: ActualParameter) -> Self {
        SingleActualParameterSequenceState {
            ap: Box::new(ap),
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(ap: ActualParameter, position: SourcePosition) -> Self {
        let mut saps = SingleActualParameterSequenceState::new(ap);
        saps.common_state.position = position;
        saps
    }
}

impl PartialEq for SingleActualParameterSequenceState {
    fn eq(&self, other: &Self) -> bool {
        self.ap == other.ap
    }
}

impl Eq for SingleActualParameterSequenceState {}

impl fmt::Display for SingleActualParameterSequenceState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SingleActualParameterSequenceState::new({})", self.ap)
    }
}

impl Ast for SingleActualParameterSequenceState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_single_actual_parameter_sequence(self, arg)
    }
}

#[derive(Debug, Clone)]
pub struct MultipleActualParameterSequenceState {
    pub ap: Box<ActualParameter>,
    pub aps: Box<ActualParameterSequence>,
    pub common_state: CommonState,
}

impl MultipleActualParameterSequenceState {
    pub fn new(ap: ActualParameter, aps: ActualParameterSequence) -> Self {
        MultipleActualParameterSequenceState {
            ap: Box::new(ap),
            aps: Box::new(aps),
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(
        ap: ActualParameter,
        aps: ActualParameterSequence,
        position: SourcePosition,
    ) -> Self {
        let mut maps = MultipleActualParameterSequenceState::new(ap, aps);
        maps.common_state.position = position;
        maps
    }
}

impl PartialEq for MultipleActualParameterSequenceState {
    fn eq(&self, other: &Self) -> bool {
        self.ap == other.ap && self.aps == other.aps
    }
}

impl Eq for MultipleActualParameterSequenceState {}

impl fmt::Display for MultipleActualParameterSequenceState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MultipleActualParameterSequenceState::new({}, {})",
            self.ap, self.aps
        )
    }
}

impl Ast for MultipleActualParameterSequenceState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_multiple_actual_parameter_sequence(self, arg)
    }
}

#[derive(Debug, Clone)]
pub enum ActualParameter {
    ConstActualParameter(ConstActualParameterState),
    FuncActualParameter(FuncActualParameterState),
    ProcActualParameter(ProcActualParameterState),
    VarActualParameter(VarActualParameterState),
}

impl PartialEq for ActualParameter {
    fn eq(&self, other: &Self) -> bool {
        use ActualParameter::*;

        match (self, other) {
            (ConstActualParameter(ref cap1), ConstActualParameter(ref cap2)) => cap1 == cap2,
            (FuncActualParameter(ref fap1), FuncActualParameter(ref fap2)) => fap1 == fap2,
            (ProcActualParameter(ref pap1), ProcActualParameter(ref pap2)) => pap1 == pap2,
            (VarActualParameter(ref vap1), VarActualParameter(ref vap2)) => vap1 == vap2,
            (_, _) => false,
        }
    }
}

impl Eq for ActualParameter {}

impl fmt::Display for ActualParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ActualParameter::*;

        match *self {
            ConstActualParameter(ref ap) => write!(f, "ConstActualParameter({})", ap),
            VarActualParameter(ref ap) => write!(f, "VarActualParameter({})", ap),
            ProcActualParameter(ref ap) => write!(f, "ProcActualParameter({})", ap),
            FuncActualParameter(ref ap) => write!(f, "FuncActualParameter({})", ap),
        }
    }
}

impl Ast for ActualParameter {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        use ActualParameter::*;

        match *self {
            VarActualParameter(ref mut vap) => vap.accept(visitor, arg),
            ConstActualParameter(ref mut cap) => cap.accept(visitor, arg),
            ProcActualParameter(ref mut pap) => pap.accept(visitor, arg),
            FuncActualParameter(ref mut fap) => fap.accept(visitor, arg),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VarActualParameterState {
    pub vname: Box<Vname>,
    pub common_state: CommonState,
}

impl VarActualParameterState {
    pub fn new(vname: Vname) -> Self {
        VarActualParameterState {
            vname: Box::new(vname),
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(vname: Vname, position: SourcePosition) -> Self {
        let mut vap = VarActualParameterState::new(vname);
        vap.common_state.position = position;
        vap
    }
}

impl PartialEq for VarActualParameterState {
    fn eq(&self, other: &Self) -> bool {
        self.vname == other.vname
    }
}

impl Eq for VarActualParameterState {}

impl fmt::Display for VarActualParameterState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VarActualParameterState::new({})", self.vname)
    }
}

impl Ast for VarActualParameterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_var_actual_parameter(self, arg)
    }
}

#[derive(Debug, Clone)]
pub struct ConstActualParameterState {
    pub expr: Box<Expression>,
    pub common_state: CommonState,
}

impl ConstActualParameterState {
    pub fn new(expr: Expression) -> Self {
        ConstActualParameterState {
            expr: Box::new(expr),
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(expr: Expression, position: SourcePosition) -> Self {
        let mut cap = ConstActualParameterState::new(expr);
        cap.common_state.position = position;
        cap
    }
}

impl PartialEq for ConstActualParameterState {
    fn eq(&self, other: &Self) -> bool {
        self.expr == other.expr
    }
}

impl Eq for ConstActualParameterState {}

impl fmt::Display for ConstActualParameterState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ConstActualParameterState::new({})", self.expr)
    }
}

impl Ast for ConstActualParameterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_const_actual_parameter(self, arg)
    }
}

#[derive(Debug, Clone)]
pub struct ProcActualParameterState {
    pub id: Identifier,
    pub common_state: CommonState,
}

impl ProcActualParameterState {
    pub fn new(id: Identifier) -> Self {
        ProcActualParameterState {
            id: id,
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(id: Identifier, position: SourcePosition) -> Self {
        let mut pap = ProcActualParameterState::new(id);
        pap.common_state.position = position;
        pap
    }
}

impl PartialEq for ProcActualParameterState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ProcActualParameterState {}

impl fmt::Display for ProcActualParameterState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ProcActualParameterState::new({})", self.id)
    }
}

impl Ast for ProcActualParameterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_proc_actual_parameter(self, arg)
    }
}

#[derive(Debug, Clone)]
pub struct FuncActualParameterState {
    pub id: Identifier,
    pub common_state: CommonState,
}

impl FuncActualParameterState {
    pub fn new(id: Identifier) -> Self {
        FuncActualParameterState {
            id: id,
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(id: Identifier, position: SourcePosition) -> Self {
        let mut fap = FuncActualParameterState::new(id);
        fap.common_state.position = position;
        fap
    }
}

impl PartialEq for FuncActualParameterState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for FuncActualParameterState {}

impl fmt::Display for FuncActualParameterState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FuncActualParameterState::new({})", self.id)
    }
}

impl Ast for FuncActualParameterState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_func_actual_parameter(self, arg)
    }
}
