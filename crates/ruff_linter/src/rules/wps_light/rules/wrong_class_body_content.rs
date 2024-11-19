use itertools::enumerate;
use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::{helpers::is_docstring_stmt, Stmt, StmtClassDef};
use ruff_text_size::Ranged;

use crate::checkers::ast::Checker;

/// ## What it does
/// Checks for disallowed statements in the body of a class.
///
/// ## Why is this bad?
/// Python allows us to have conditions, context managers,
/// and even infinite loops inside class definitions.
/// On the other hand, only methods, attributes, and docstrings make sense.
/// So, we discourage using anything except these nodes in class bodies.
///
/// ## Example
/// ```python
/// class Test:
///     for _ in range(10):
///         print("What?!")
/// ```
#[violation]
pub struct WrongClassBodyContent;

impl Violation for WrongClassBodyContent {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Wrong statement inside class definition".to_string()
    }
}

/// WPS604
pub(crate) fn wrong_class_body_content(checker: &mut Checker, class: &StmtClassDef) {
    let StmtClassDef { body, .. } = class;
    for (index, stmt) in enumerate(body) {
        if index == 0 && is_docstring_stmt(stmt) {
            continue;
        }
        if !is_allowed_statement(stmt) {
            checker
                .diagnostics
                .push(Diagnostic::new(WrongClassBodyContent, stmt.range()));
        }
    }
}

fn is_allowed_statement(stmt: &Stmt) -> bool {
    matches!(
        stmt,
        Stmt::FunctionDef(_)
        | Stmt::ClassDef(_)
        | Stmt::Assign(_)
        | Stmt::AnnAssign(_)
    )
}
