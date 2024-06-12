use ruff_python_ast::Identifier;

use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_text_size::Ranged;

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
pub struct FunctionNamedA(pub String);

impl Violation for FunctionNamedA {
    #[derive_message_formats]
    fn message(&self) -> String {
        let FunctionNamedA(name) = self;
        format!("Single lettered function named: `{name}`")
    }
}

/// BRT001
pub(crate) fn function_named_a(name: &Identifier) -> Option<Diagnostic> {
    if name == "a" || name == "A" {
        Some(Diagnostic::new(
            FunctionNamedA(name.to_string()),
            name.range(),
        ))
    } else {
        None
    }
}
