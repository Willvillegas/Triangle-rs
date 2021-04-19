//! command asts

use super::declarations::Declaration;
use super::expressions::Expression;
use super::parameters::ActualParameterSequence;
use super::primitives::Identifier;
use super::vnames::Vname;
use super::{Ast, AstObject, AstVisitor, CommonState};
use crate::scanner::SourcePosition;

#[derive(Debug)]
pub enum Command {
    AssignCommand(AssignCommandState),
    CallCommand(CallCommandState),
    EmptyCommand(EmptyCommandState),
    IfCommand(IfCommandState),
    LetCommand(LetCommandState),
    SequentialCommand(SequentialCommandState),
    WhileCommand(WhileCommandState),
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        use Command::*;

        match (self, other) {
            (EmptyCommand(_), EmptyCommand(_)) => true,
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

impl Ast for Command {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        use Command::*;

        match *self {
            AssignCommand(ref mut asscmd) => asscmd.accept(visitor, arg),
            CallCommand(ref mut callcmd) => callcmd.accept(visitor, arg),
            EmptyCommand(ref mut emptycmd) => emptycmd.accept(visitor, arg),
            IfCommand(ref mut ifcmd) => ifcmd.accept(visitor, arg),
            LetCommand(ref mut letcmd) => letcmd.accept(visitor, arg),
            SequentialCommand(ref mut seqcmd) => seqcmd.accept(visitor, arg),
            WhileCommand(ref mut whilecmd) => whilecmd.accept(visitor, arg),
        }
    }
}

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

    pub fn new_with_position(vname: Vname, expr: Expression, position: SourcePosition) -> Self {
        let mut cmd = AssignCommandState::new(vname, expr);
        cmd.common_state.position = position;
        cmd
    }
}

impl PartialEq for AssignCommandState {
    fn eq(&self, other: &Self) -> bool {
        self.vname == other.vname && self.expr == other.expr
    }
}

impl Eq for AssignCommandState {}

impl Ast for AssignCommandState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_assign_command(self, arg)
    }
}

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

    pub fn new_with_position(
        id: Identifier,
        aps: ActualParameterSequence,
        position: SourcePosition,
    ) -> Self {
        let mut cmd = CallCommandState::new(id, aps);
        cmd.common_state.position = position;
        cmd
    }
}

impl PartialEq for CallCommandState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.aps == other.aps
    }
}

impl Eq for CallCommandState {}

impl Ast for CallCommandState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_call_command(self, arg)
    }
}

#[derive(Debug)]
pub struct EmptyCommandState {
    pub common_state: CommonState,
}

impl EmptyCommandState {
    pub fn new() -> Self {
        EmptyCommandState {
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(position: SourcePosition) -> Self {
        let mut cmd = EmptyCommandState::new();
        cmd.common_state.position = position;
        cmd
    }
}

impl PartialEq for EmptyCommandState {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Eq for EmptyCommandState {}

impl Ast for EmptyCommandState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_empty_command(self, arg)
    }
}
#[derive(Debug)]
pub struct LetCommandState {
    pub decl: Box<Declaration>,
    pub cmd: Box<Command>,
    pub common_state: CommonState,
}

impl LetCommandState {
    pub fn new(decl: Declaration, cmd: Command) -> Self {
        LetCommandState {
            decl: Box::new(decl),
            cmd: Box::new(cmd),
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(decl: Declaration, cmd: Command, position: SourcePosition) -> Self {
        let mut cmd = LetCommandState::new(decl, cmd);
        cmd.common_state.position = position;
        cmd
    }
}

impl PartialEq for LetCommandState {
    fn eq(&self, other: &Self) -> bool {
        self.decl == other.decl && self.cmd == other.cmd
    }
}

impl Eq for LetCommandState {}

impl Ast for LetCommandState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_let_command(self, arg)
    }
}
#[derive(Debug)]
pub struct IfCommandState {
    pub expr: Box<Expression>,
    pub cmd1: Box<Command>,
    pub cmd2: Box<Command>,
    pub common_state: CommonState,
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

    pub fn new_with_position(
        expr: Expression,
        cmd1: Command,
        cmd2: Command,
        position: SourcePosition,
    ) -> Self {
        let mut cmd = IfCommandState::new(expr, cmd1, cmd2);
        cmd.common_state.position = position;
        cmd
    }
}

impl PartialEq for IfCommandState {
    fn eq(&self, other: &Self) -> bool {
        self.expr == other.expr && self.cmd1 == other.cmd1 && self.cmd2 == other.cmd2
    }
}

impl Eq for IfCommandState {}

impl Ast for IfCommandState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_if_command(self, arg)
    }
}

#[derive(Debug)]
pub struct WhileCommandState {
    pub expr: Box<Expression>,
    pub cmd: Box<Command>,
    pub common_state: CommonState,
}

impl WhileCommandState {
    pub fn new(expr: Expression, cmd: Command) -> Self {
        WhileCommandState {
            expr: Box::new(expr),
            cmd: Box::new(cmd),
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(expr: Expression, cmd: Command, position: SourcePosition) -> Self {
        let mut cmd = WhileCommandState::new(expr, cmd);
        cmd.common_state.position = position;
        cmd
    }
}

impl PartialEq for WhileCommandState {
    fn eq(&self, other: &Self) -> bool {
        self.expr == other.expr && self.cmd == other.cmd
    }
}

impl Eq for WhileCommandState {}

impl Ast for WhileCommandState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_while_command(self, arg)
    }
}

#[derive(Debug)]
pub struct SequentialCommandState {
    pub cmd1: Box<Command>,
    pub cmd2: Box<Command>,
    pub common_state: CommonState,
}

impl SequentialCommandState {
    pub fn new(cmd1: Command, cmd2: Command) -> Self {
        SequentialCommandState {
            cmd1: Box::new(cmd1),
            cmd2: Box::new(cmd2),
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(cmd1: Command, cmd2: Command, position: SourcePosition) -> Self {
        let mut seqcmd = SequentialCommandState::new(cmd1, cmd2);
        seqcmd.common_state.position = position;
        seqcmd
    }
}

impl PartialEq for SequentialCommandState {
    fn eq(&self, other: &Self) -> bool {
        self.cmd1 == other.cmd1 && self.cmd1 == other.cmd2
    }
}

impl Eq for SequentialCommandState {}

impl Ast for SequentialCommandState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_sequential_command(self, arg)
    }
}
