use ruff_diagnostics::{AlwaysFixableViolation, Diagnostic, Edit, Fix};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::ExprNumberLiteral;
use ruff_text_size::Ranged;

use crate::checkers::ast::Checker;

/// ## What it does
/// Checks for float literals that
/// start or end with a dot, such as `.5`, `23.`, or `.05`.
/// 
/// ## Why is this bad?
/// While Python allows shorthand notation for floats (e.g., `.5` instead of `0.5`),
/// it is recommended to avoid partial float literals to maintain consistency and clarity.
/// 
/// ## Example
/// ```python
/// half = .5
/// ten_float = 10.
/// ```
/// 
/// Use instead:
/// ```python
/// half = 0.5
/// ten_float = 10.0
/// ```
#[violation]
pub struct PartialFloatLiteral {
    number_str: String,
}

impl AlwaysFixableViolation for PartialFloatLiteral {
    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { number_str, .. } = self;
        format!("Found partial float literal `{number_str}`")
    }

    fn fix_title(&self) -> String {
        "Insert beginning or trailing zero".to_string()
    }
}

/// WPS304
pub(crate) fn partial_float_literal(checker: &mut Checker, number_literal: &ExprNumberLiteral) {
    if !number_literal.value.is_float() {
        return;
    }

    let number_str = &checker.locator().contents()[number_literal.range()];

    let mut fix: Option<Fix> = None;
    if number_str.starts_with('.') {
        fix = Some(Fix::safe_edit(Edit::range_replacement(
            format!("0{number_str}"),
            number_literal.range(),
        )));
    } else if number_str.ends_with('.') {
        fix = Some(Fix::safe_edit(Edit::range_replacement(
            format!("{number_str}0"),
            number_literal.range(),
        )));
    }
    if let Some(fix) = fix {
        checker.diagnostics.push(
            Diagnostic::new(
                PartialFloatLiteral {
                    number_str: number_str.to_string(),
                },
                number_literal.range(),
            )
            .with_fix(fix),
        );
    }
}
