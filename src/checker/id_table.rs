use crate::ast::declarations::Declaration;
use std::collections::HashMap;

type Level = u8;

/// The Identification Table contains metadats about all the
/// applied occurrences in a program - it maps a given applied occurrence
/// to its declaration in the AST, if present.
pub struct IdentificationTable {
    table: HashMap<Level, HashMap<String, Declaration>>,
    curr_level: Level,
}

impl IdentificationTable {
    pub fn new() -> Self {
        IdentificationTable {
            table: HashMap::new(),
            curr_level: 0,
        }
    }

    pub fn enter(&mut self, id: &str) {}

    pub fn retrieve(&self, id: &str) -> Option<Declaration> {
        None
    }

    pub fn open_scope(&mut self) {}

    pub fn close_scope(&mut self) {}
}
