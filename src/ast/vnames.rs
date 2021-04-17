//! vname asts

use super::CommonState;

#[derive(Debug)]
pub enum Vname {
    SimpleVname(Box<SimpleVnameState>),
    SubscriptVname(Box<SubscriptVnameState>),
    DotVname(Box<DotVnameState>),
}

impl PartialEq for Vname {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for Vname {}

#[derive(Debug)]
pub struct SimpleVnameState {}

#[derive(Debug)]
pub struct SubscriptVnameState {}

#[derive(Debug)]
pub struct DotVnameState {}
