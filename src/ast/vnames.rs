//! vname asts

use super::CommonState;

pub enum Vname {
    SimpleVname(Box<SimpleVnameState>),
    SubscriptVname(Box<SubscriptVnameState>),
    DotVname(Box<DotVnameState>),
}

impl PartialEq for Vname {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Eq for Vname {}

pub struct SimpleVnameState {}

pub struct SubscriptVnameState {}

pub struct DotVnameState {}
