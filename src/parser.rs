//! The Parser module.
//!
//! This module consumes the stream of tokens produced by the Scanner, and constructs an AST for
//! Triangle, which is then used by all subsequent phases of the compiler.

use crate::error::report_error_and_exit;

pub struct Parser {}