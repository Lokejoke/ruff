use regex::Regex;
use ruff_python_ast::{Expr, ExprNumberLiteral};
use ruff_text_size::{Ranged, TextSize};

use ruff_diagnostics::{Diagnostic, Edit, Fix, Violation};
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
pub struct ConsecutiveUndersocresInName;

impl Violation for ConsecutiveUndersocresInName {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Numeric literals with a string representation longer than ten characters are not permitted")
    }

    // fn fix_title(&self) -> String {
    //     "Replace with `...`".to_string()
    // }
}

/// WPS116
pub(crate) fn consecutive_undersocres_in_name(checker: &mut Checker, expr: Expr) {
    let name = checker.locator().slice(expr.range());

    if !name.contains("__") {
        return;
    }

    // let mut diagnostic = Diagnostic::new(ConsecutiveUndersocresInName, expr.range());
    // diagnostic.set_fix(Fix::safe_edit(Edit::range_replacement(
    //     number_str.replace('_', ""),
    //     expr.range(),
    // )));
    // checker.diagnostics.push(diagnostic);
}
