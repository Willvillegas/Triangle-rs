//! parameter asts

use super::expressions::Expression;
use super::primitives::Identifier;
use super::typedenoters::TypeDenoter;
use super::vnames::Vname;
use super::{Ast, AstObject, AstVisitor, CommonState};

#[derive(Debug)]
pub enum FormalParameterSequence {
    EmptyFormalParameterSequence(EmptyFormalParameterSequenceState),
    SingleFormalParameterSequence(SingleFormalParameterSequenceState),
    MultipleFormalParameterSequence(MultipleFormalParameterSequenceState),
}

impl PartialEq for FormalParameterSequence {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for FormalParameterSequence {}

#[derive(Debug, PartialEq, Eq)]
pub struct EmptyFormalParameterSequenceState;

#[derive(Debug)]
pub struct SingleFormalParameterSequenceState {
    pub fp: FormalParameter,
    pub common_state: CommonState,
}

impl SingleFormalParameterSequenceState {
    pub fn new(fp: FormalParameter) -> Self {
        SingleFormalParameterSequenceState {
            fp: fp,
            common_state: CommonState::default(),
        }
    }
}

#[derive(Debug)]
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
}

#[derive(Debug)]
pub enum FormalParameter {
    VarFormalParameter(VarFormalParameterState),
    ConstFormalParameter(ConstFormalParameterState),
    ProcFormalParameter(ProcFormalParameterState),
    FuncFormalParameter(FuncFormalParameterState),
}

#[derive(Debug)]
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
}

#[derive(Debug)]
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
}

#[derive(Debug)]
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
}

#[derive(Debug)]
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
}

#[derive(Debug)]
pub enum ActualParameterSequence {
    EmptyActualParameterSequence(EmptyActualParameterSequenceState),
    SingleActualParamterSequence(SingleActualParameterSequenceState),
    MultipleActualParameterSequence(MultipleActualParameterSequenceState),
}

impl PartialEq for ActualParameterSequence {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for ActualParameterSequence {}

impl Ast for ActualParameterSequence {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct EmptyActualParameterSequenceState;

#[derive(Debug)]
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
}

#[derive(Debug)]
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
}

#[derive(Debug)]
pub enum ActualParameter {
    VarActualParameter(VarActualParameterState),
    ConstActualParameter(ConstActualParameterState),
    ProcActualParameter(ProcActualParameterState),
    FuncActualParameter(FuncActualParameterState),
}

#[derive(Debug)]
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
}

#[derive(Debug)]
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
}

#[derive(Debug)]
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
}

#[derive(Debug)]
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
}
