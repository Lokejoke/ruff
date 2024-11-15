use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, violation};

use crate::checkers::ast::Checker;

/// ## What it does
///
/// ## Why is this bad?
///
/// ## Example
/// ```python
/// ```
///
/// Use instead:
/// ```python
/// ```
#[violation]
pub struct Dummy;

impl Violation for Dummy {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Empty rule".to_string()
    }
}

/// WPS000
#[allow(dead_code)]
pub(crate) fn dummy(_checker: &mut Checker) {}
