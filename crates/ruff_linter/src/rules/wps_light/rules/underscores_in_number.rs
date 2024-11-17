use ruff_diagnostics::{AlwaysFixableViolation, Diagnostic, Edit, Fix};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::ExprNumberLiteral;
use ruff_text_size::Ranged;

use crate::checkers::ast::Checker;

/// ## What it does
/// Forbids underscores (`_`) in number literals.
///
/// ## Why is this bad?
/// Numbers like `1000` can be written in multiple ways using underscores:
/// `1_000`, `10_00`, and `100_0`. While all of these are valid and represent
/// the same number, they rely on the author's cultural habits, leading to
/// inconsistencies and potential confusion. Enforcing a single, clear way to
/// write numbers improves readability and maintainability.
///
/// ## Example
/// ```python
/// phone = 8_83_134_43
/// million = 100_00_00
/// ```
///
/// Use instead:
/// ```python
/// phone = 88313443
/// million = 1000000
/// ```
#[violation]
pub struct UnderscoresInNumber {
    number: String,
}

impl AlwaysFixableViolation for UnderscoresInNumber {
    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { number, .. } = self;
        format!("Found underscores in number literal `{number}`")
    }

    fn fix_title(&self) -> String {
        "Remove underscores from number literal".to_string()
    }
}

/// WPS303
pub(crate) fn underscores_in_number(checker: &mut Checker, number: &ExprNumberLiteral) {
    let num_str = &checker.locator().contents()[number.range()];
    if !num_str.contains('_') {
        return;
    }
    checker.diagnostics.push(
        Diagnostic::new(
            UnderscoresInNumber {
                number: num_str.to_string(),
            },
            number.range(),
        )
        .with_fix(Fix::safe_edit(Edit::range_replacement(
            num_str.replace('_', ""),
            number.range(),
        ))),
    );
}
