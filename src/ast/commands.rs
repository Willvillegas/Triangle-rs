//! command asts

use super::declarations::Declaration;
use super::expressions::Expression;
use super::parameters::ActualParameterSequence;
use super::primitives::Identifier;
use super::vnames::Vname;
use super::CommonState;

#[derive(Debug)]
pub enum Command {
    EmptyCommand,
    AssignCommand(AssignCommandState),
    CallCommand(CallCommandState),
    BracketedCommand(BracketedCommandState),
    LetCommand(LetCommandState),
    IfCommand(IfCommandState),
    WhileCommand(WhileCommandState),
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Eq for Command {}

#[derive(Debug)]
pub struct AssignCommandState {
    pub vname: Box<Vname>,
    pub expr: Box<Expression>,
    pub common_state: CommonState,
}

impl AssignCommandState {
    pub fn new(vname: Vname, expr: Expression) -> Self {
        AssignCommandState {
            vname: Box::new(vname),
            expr: Box::new(expr),
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

#[derive(Debug)]
pub struct CallCommandState {
    pub id: Identifier,
    pub aps: Box<ActualParameterSequence>,
    pub common_state: CommonState,
}

impl CallCommandState {
    pub fn new(id: Identifier, aps: ActualParameterSequence) -> Self {
        CallCommandState {
            id: id,
            aps: Box::new(aps),
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

#[derive(Debug)]
pub struct BracketedCommandState {
    pub cmd: Box<Command>,
    common_state: CommonState,
}

impl BracketedCommandState {
    pub fn new(cmd: Command) -> Self {
        BracketedCommandState {
            cmd: Box::new(cmd),
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

#[derive(Debug)]
pub struct LetCommandState {
    decl: Box<Declaration>,
    cmd: Box<Command>,
    common_state: CommonState,
}

impl LetCommandState {
    pub fn new(decl: Declaration, cmd: Command) -> Self {
        LetCommandState {
            decl: Box::new(decl),
            cmd: Box::new(cmd),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for LetCommandState {
    fn eq(&self, other: &Self) -> bool {
        self.decl == other.decl && self.cmd == other.cmd
    }
}

impl Eq for LetCommandState {}

#[derive(Debug)]
pub struct IfCommandState {
    expr: Box<Expression>,
    cmd1: Box<Command>,
    cmd2: Box<Command>,
    common_state: CommonState,
}

impl IfCommandState {
    pub fn new(expr: Expression, cmd1: Command, cmd2: Command) -> Self {
        IfCommandState {
            expr: Box::new(expr),
            cmd1: Box::new(cmd1),
            cmd2: Box::new(cmd2),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for IfCommandState {
    fn eq(&self, other: &Self) -> bool {
        self.expr == other.expr && self.cmd1 == other.cmd1 && self.cmd2 == other.cmd2
    }
}

impl Eq for IfCommandState {}

#[derive(Debug)]
pub struct WhileCommandState {}

impl WhileCommandState {}

impl PartialEq for WhileCommandState {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for WhileCommandState {}
