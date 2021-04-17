//! declaration asts

use super::CommonState;

#[derive(Debug)]
pub enum Declaration {}

impl PartialEq for Declaration {
    fn eq(&self, other: &Self) -> bool {
        true // todo
    }
}

impl Eq for Declaration {}
