use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_semantic::Binding;

use crate::checkers::ast::Checker;

/// ## What it does
/// Checks for variables named as reserved arguments
/// (i.e., `self`, `cls` and `mcs`)
///
/// ## Why is this bad?
/// These names are special, they should only be used as first arguments inside methods.
///
/// ## Example
/// ```python
/// cls = 5
/// lambda self: self + 12
/// ```
#[violation]
pub struct ReservedArgumentAsVariable {
    name: String,
}

impl Violation for ReservedArgumentAsVariable {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Reserved argument name {} used as variable", self.name)
    }
}

/// WPS117
pub(crate) fn reserved_argument_as_variable(
    checker: &Checker,
    binding: &Binding,
) -> Option<Diagnostic> {
    let name = binding.name(checker.locator().contents());
    let semantic = checker.semantic();

    // Check if the name is a reserved argument
    if !is_reserved_argument(name) {
        return None;
    }

    // Check if it's not used as the first parameter in a method
    if binding.statement(semantic).is_some_and(|stmt| {
        stmt.as_function_def_stmt().is_some_and(|fun_def| {
            fun_def
                .parameters
                .args
                .first()
                .is_some_and(|par| par.default.is_none() && par.parameter.name.id == name)
        })
    }) {
        return None;
    }

    Some(Diagnostic::new(
        ReservedArgumentAsVariable {
            name: name.to_string(),
        },
        binding.range,
    ))
}

fn is_reserved_argument(name: &str) -> bool {
    matches!(name, "cls" | "self" | "mcs")
}
