use ruff_python_ast::ExprNumberLiteral;
use ruff_text_size::{Ranged, TextSize};

use ruff_diagnostics::{AlwaysFixableViolation, Diagnostic, Edit, Fix};
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
pub struct UnderscoresInNumbers;

impl AlwaysFixableViolation for UnderscoresInNumbers {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Numeric literals with a string representation longer than ten characters are not permitted")
    }

    fn fix_title(&self) -> String {
        "Replace with `...`".to_string()
    }
}

/// WPS303
pub(crate) fn underscores_in_numbers(checker: &mut Checker, expr: &ExprNumberLiteral) {
    let number_str = checker.locator().slice(expr.range);

    if !number_str.contains('_') {
        return;
    }

    let mut diagnostic = Diagnostic::new(UnderscoresInNumbers, expr.range());
    diagnostic.set_fix(Fix::safe_edit(Edit::range_replacement(
        number_str.replace('_', ""),
        expr.range(),
    )));
    checker.diagnostics.push(diagnostic);
}
