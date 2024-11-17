use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::ExprNumberLiteral;
use ruff_text_size::Ranged;

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
pub struct UnderscoresInNumber;

impl Violation for UnderscoresInNumber {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Number is separated in ".to_string()
    }
}

/// WPS303
pub(crate) fn underscores_in_number(checker: &mut Checker, number: &ExprNumberLiteral) {
    let num_str = &checker.locator().contents()[number.range()];
    if !num_str.contains('_') {
        return;
    }
    let diagnostic = Diagnostic::new(
        UnderscoresInNumber,
        number.range(),
    );
    checker.diagnostics.push(diagnostic);
}
