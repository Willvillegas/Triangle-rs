//! command asts

use super::declarations::Declaration;
use super::expressions::Expression;
use super::parameters::ActualParameterSequence;
use super::primitives::Identifier;
use super::vnames::Vname;
use super::{Ast, AstObject, AstVisitor, CommonState};

#[derive(Debug)]
pub enum Command {
    AssignCommand(AssignCommandState),
    CallCommand(CallCommandState),
    EmptyCommand,
    IfCommand(IfCommandState),
    LetCommand(LetCommandState),
    SequentialCommand(SequentialCommandState),
    WhileCommand(WhileCommandState),
}

impl Ast for Command {
    fn accept(&mut self, visitor: &dyn AstVisitor) -> AstObject {
        visitor.visit_command(self)
    }
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        use Command::*;
        match (self, other) {
            (EmptyCommand, EmptyCommand) => true,
            (AssignCommand(ref asscmd1), AssignCommand(ref asscmd2)) => asscmd1 == asscmd2,
            (CallCommand(ref callcmd1), CallCommand(ref callcmd2)) => callcmd1 == callcmd2,
            (IfCommand(ref ifcmd1), IfCommand(ref ifcmd2)) => ifcmd1 == ifcmd2,
            (LetCommand(ref letcmd1), LetCommand(ref letcmd2)) => letcmd1 == letcmd2,
            (SequentialCommand(ref seqcmd1), SequentialCommand(ref seqcmd2)) => seqcmd1 == seqcmd2,
            (WhileCommand(ref whilecmd1), WhileCommand(ref whilecmd2)) => whilecmd1 == whilecmd2,
            (_, _) => false,
        }
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
pub struct WhileCommandState {
    expr: Box<Expression>,
    cmd: Box<Command>,
    common_state: CommonState,
}

impl WhileCommandState {
    pub fn new(expr: Expression, cmd: Command) -> Self {
        WhileCommandState {
            expr: Box::new(expr),
            cmd: Box::new(cmd),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for WhileCommandState {
    fn eq(&self, other: &Self) -> bool {
        self.expr == other.expr && self.cmd == other.cmd
    }
}

impl Eq for WhileCommandState {}

#[derive(Debug)]
pub struct SequentialCommandState {
    cmd1: Box<Command>,
    cmd2: Box<Command>,
    common_state: CommonState,
}

impl SequentialCommandState {
    pub fn new(cmd1: Command, cmd2: Command) -> Self {
        SequentialCommandState {
            cmd1: Box::new(cmd1),
            cmd2: Box::new(cmd2),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for SequentialCommandState {
    fn eq(&self, other: &Self) -> bool {
        self.cmd1 == other.cmd1 && self.cmd1 == other.cmd2
    }
}

impl Eq for SequentialCommandState {}
