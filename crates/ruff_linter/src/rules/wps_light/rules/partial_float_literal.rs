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
pub struct PartialFloatLiteral;

impl Violation for PartialFloatLiteral {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("TODO: write message: {}", todo!("implement message"))
    }
}

/// WPS304
pub(crate) fn partial_float_literal(checker: &mut Checker) {}
