use crate::ast::declarations::Declaration;
use crate::error;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

type Level = u8;

/// The Identification Table contains metadats about all the
/// applied occurrences in a program - it maps a given applied occurrence
/// to its declaration in the AST, if present.
pub struct IdentificationTable {
    table: HashMap<Level, HashMap<String, Arc<Mutex<Declaration>>>>,
    curr_level: Level,
}

impl IdentificationTable {
    pub fn new() -> Self {
        IdentificationTable {
            table: HashMap::new(),
            curr_level: 0,
        }
    }

    pub fn enter(&mut self, id: String, val: Arc<Mutex<Declaration>>) {
        if !self.table.contains_key(&self.curr_level) {
            self.table.insert(self.curr_level, HashMap::new());
        }

        if self.table.get(&self.curr_level).unwrap().contains_key(&id) {
            error::report_error_and_exit(error::GenError::from(error::CheckerError::new(
                &format!(
                    "identifier {} is already defined at level {}",
                    id, self.curr_level
                ),
            )));
        }

        self.table
            .get_mut(&self.curr_level)
            .unwrap()
            .insert(id, val);
    }

    pub fn retrieve(&self, id: &String) -> Option<&Arc<Mutex<Declaration>>> {
        let mut level = self.curr_level;

        loop {
            if let Some(mapping) = self.table.get(&level) {
                if mapping.contains_key(id) {
                    return self.table.get(&level).unwrap().get(id);
                }
            }
            level -= 1;

            if level == 0 {
                return None;
            }
        }
    }

    pub fn open_scope(&mut self) {
        self.curr_level += 1;
    }

    pub fn close_scope(&mut self) {
        self.table.remove_entry(&self.curr_level);
        self.curr_level -= 1;
    }
}

impl fmt::Display for IdentificationTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (level, entries) in &self.table {
            let _ = writeln!(f, "Level {}", level);

            for (key, val) in entries {
                let _ = writeln!(f, "{} => {}", key, *val.lock().unwrap());
            }
        }
        writeln!(f, "{}", "")
    }
}
