use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::{
    helpers::{map_callable, ReturnStatementVisitor},
    visitor::Visitor,
    Expr, Stmt, StmtExpr, StmtFunctionDef, StmtRaise,
};
use ruff_python_semantic::SemanticModel;
use ruff_text_size::Ranged;

use crate::checkers::ast::Checker;

/// ## What it does
/// Checks for return value in functions and method prefixed with 'get_'
///
/// ## Why is this bad?
/// 'get_' prefixed are considered as getter function, with capacity
/// of returning any value.
///
/// ## Example
/// ```python
/// def get_function(cond_val):
///     if cond_val:
///         return None
///     return
/// ```
///
/// Use instead:
/// ```python
/// def get_function(cond_val):
///     if cond_val:
///         return None
///     pass
/// ```
#[violation]
pub struct GetterWithoutReturnValue;

impl Violation for GetterWithoutReturnValue {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Getter function returns no value.".to_string()
    }
}

/// WPS463
pub(crate) fn getter_without_return_value(checker: &mut Checker, function_def: &StmtFunctionDef) {
    let StmtFunctionDef { body, name, .. } = function_def;

    // Matching regex (get_.\w+)
    if name.len() < 5 || !name.starts_with("get_") {
        return;
    }

    if is_not_implemented_function(function_def, checker.semantic()) {
        return;
    }

    let mut visitor = ReturnStatementVisitor::default();
    visitor.visit_body(body);

    // No yield
    if visitor.is_generator {
        return;
    }

    // No return value
    if visitor.returns.is_empty() || visitor.returns.iter().any(|stmt| stmt.value.is_some()) {
        return;
    }

    checker.diagnostics.push(Diagnostic::new(
        GetterWithoutReturnValue,
        function_def.range(),
    ));
}

fn is_not_implemented_function(function_def: &StmtFunctionDef, semantic: &SemanticModel) -> bool {
    let body = &function_def.body;

    // Checks if last and only statement is doc string
    if body.len() == 1 {
        if let Stmt::Expr(StmtExpr { value, range: _ }) = &body[0] {
            if value.is_string_literal_expr() {
                return true;
            }
        }
    }

    // Checks if is not implemented
    body.iter().any(|stmt| match stmt {
        Stmt::Pass(_) => true,
        Stmt::Expr(StmtExpr { value, .. }) => {
            matches!(value.as_ref(), Expr::EllipsisLiteral(_))
        }
        Stmt::Raise(StmtRaise { exc: exception, .. }) => exception.as_ref().is_some_and(|exc| {
            semantic
                .resolve_builtin_symbol(map_callable(exc))
                .is_some_and(|name| matches!(name, "NotImplementedError" | "NotImplemented"))
        }),
        _ => false,
    })
}
