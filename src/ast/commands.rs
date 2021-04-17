//! command asts

use super::expressions::Expression;
use super::parameters::ActualParameterSequence;
use super::primitives::Identifier;
use super::vnames::Vname;
use super::CommonState;

pub enum Command {
    EmptyCommand,
    AssignCommand(Box<AssignCommandState>),
    CallCommand(Box<CallCommandState>),
    BracketedCommand(Box<BracketedCommandState>),
    LetCommand(Box<LetCommandState>),
    IfCommand(Box<IfCommandState>),
    WhileCommand(Box<WhileCommandState>),
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Eq for Command {}

pub struct AssignCommandState {
    pub vname: Vname,
    pub expr: Expression,
    pub common_state: CommonState,
}

impl AssignCommandState {
    pub fn new(vname: Vname, expr: Expression) -> Self {
        AssignCommandState {
            vname: vname,
            expr: expr,
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for AssignCommandState {
    fn eq(&self, other: &Self) -> bool {
        self.vname == other.vname && self.expr == other.expr
    }
}

impl Eq for AssignCommandState {}

pub struct CallCommandState {
    pub id: Identifier,
    pub aps: ActualParameterSequence,
    pub common_state: CommonState,
}

impl CallCommandState {
    pub fn new(id: Identifier, aps: ActualParameterSequence) -> Self {
        CallCommandState {
            id: id,
            aps: aps,
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for CallCommandState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.aps == other.aps
    }
}

impl Eq for CallCommandState {}

pub struct BracketedCommandState {
    pub cmd: Command,
    common_state: CommonState,
}

impl BracketedCommandState {
    pub fn new(cmd: Command) -> Self {
        BracketedCommandState {
            cmd: cmd,
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for BracketedCommandState {
    fn eq(&self, other: &BracketedCommandState) -> bool {
        self.cmd == other.cmd
    }
}

impl Eq for BracketedCommandState {}

pub struct LetCommandState {}

pub struct IfCommandState {}

pub struct WhileCommandState {}
