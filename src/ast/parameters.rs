//! parameter asts

use super::expressions::Expression;
use super::primitives::Identifier;
use super::typedenoters::TypeDenoter;
use super::vnames::Vname;
use super::CommonState;

#[derive(Debug)]
pub enum FormalParameterSequence {
    SingleFormalParameterSequence(Box<SingleFormalParameterSequenceState>),
    MultipleFormalParameterSequence(Box<MultipleFormalParameterSequenceState>),
}

impl PartialEq for FormalParameterSequence {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Eq for FormalParameterSequence {}

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
    pub fp: FormalParameter,
    pub fps: FormalParameterSequence,
    pub common_state: CommonState,
}

impl MultipleFormalParameterSequenceState {
    pub fn new(fp: FormalParameter, fps: FormalParameterSequence) -> Self {
        MultipleFormalParameterSequenceState {
            fp: fp,
            fps: fps,
            common_state: CommonState::default(),
        }
    }
}

#[derive(Debug)]
pub enum FormalParameter {
    VarFormalParameter(Box<VarFormalParameterState>),
    ConstFormalParameter(Box<ConstFormalParameterState>),
    ProcFormalParameter(Box<ProcFormalParameterState>),
    FuncFormalParameter(Box<FuncFormalParameterState>),
}

#[derive(Debug)]
pub struct VarFormalParameterState {
    pub id: Identifier,
    pub td: TypeDenoter,
    pub common_state: CommonState,
}

impl VarFormalParameterState {
    pub fn new(id: Identifier, td: TypeDenoter) -> Self {
        VarFormalParameterState {
            id: id,
            td: td,
            common_state: CommonState::default(),
        }
    }
}

#[derive(Debug)]
pub struct ConstFormalParameterState {
    pub id: Identifier,
    pub td: TypeDenoter,
    pub common_state: CommonState,
}

impl ConstFormalParameterState {
    pub fn new(id: Identifier, td: TypeDenoter) -> Self {
        ConstFormalParameterState {
            id: id,
            td: td,
            common_state: CommonState::default(),
        }
    }
}

#[derive(Debug)]
pub struct ProcFormalParameterState {
    pub id: Identifier,
    pub fps: FormalParameterSequence,
    pub common_state: CommonState,
}

impl ProcFormalParameterState {
    pub fn new(id: Identifier, fps: FormalParameterSequence) -> Self {
        ProcFormalParameterState {
            id: id,
            fps: fps,
            common_state: CommonState::default(),
        }
    }
}

#[derive(Debug)]
pub struct FuncFormalParameterState {
    pub id: Identifier,
    pub fps: FormalParameterSequence,
    pub td: TypeDenoter,
    pub common_state: CommonState,
}

impl FuncFormalParameterState {
    pub fn new(id: Identifier, fps: FormalParameterSequence, td: TypeDenoter) -> Self {
        FuncFormalParameterState {
            id: id,
            fps: fps,
            td: td,
            common_state: CommonState::default(),
        }
    }
}

#[derive(Debug)]
pub enum ActualParameterSequence {
    SingleActualParamterSequence(Box<SingleActualParamterSequenceState>),
    MultipleActualParameterSequence(Box<MultipleActualParameterSequenceState>),
}

impl PartialEq for ActualParameterSequence {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Eq for ActualParameterSequence {}

#[derive(Debug)]
pub struct SingleActualParamterSequenceState {
    pub ap: ActualParameter,
    pub common_state: CommonState,
}

impl SingleActualParamterSequenceState {
    pub fn new(ap: ActualParameter) -> Self {
        SingleActualParamterSequenceState {
            ap: ap,
            common_state: CommonState::default(),
        }
    }
}

#[derive(Debug)]
pub struct MultipleActualParameterSequenceState {
    pub ap: ActualParameter,
    pub aps: ActualParameterSequence,
    pub common_state: CommonState,
}

impl MultipleActualParameterSequenceState {
    pub fn new(ap: ActualParameter, aps: ActualParameterSequence) -> Self {
        MultipleActualParameterSequenceState {
            ap: ap,
            aps: aps,
            common_state: CommonState::default(),
        }
    }
}

#[derive(Debug)]
pub enum ActualParameter {
    VarActualParameter(Box<VarActualParameterState>),
    ConstActualParameter(Box<ConstActualParameterState>),
    ProcActualParameter(Box<ProcActualParameterState>),
    FuncActualParameter(Box<FuncActualParameterState>),
}

#[derive(Debug)]
pub struct VarActualParameterState {
    pub vname: Vname,
    pub common_state: CommonState,
}

impl VarActualParameterState {
    pub fn new(vname: Vname) -> Self {
        VarActualParameterState {
            vname: vname,
            common_state: CommonState::default(),
        }
    }
}

#[derive(Debug)]
pub struct ConstActualParameterState {
    pub expr: Expression,
    pub common_state: CommonState,
}

impl ConstActualParameterState {
    pub fn new(expr: Expression) -> Self {
        ConstActualParameterState {
            expr: expr,
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
