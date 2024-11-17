use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::{Expr, StmtAssign};
use ruff_text_size::Ranged;

use crate::checkers::ast::Checker;

/// ## What it does
/// This rule checks for assignments to a subscript slice.
///
/// ## Why is this bad?
/// Assignment to a slice can implicitly change the size of a list,
/// leading to potential bugs that are difficult to spot.
///
/// ## Example
/// ```python
/// a[1:3] = [1, 2]
/// a[slice(1)] = [1, 3]
/// ```
/// 
/// Use instead:
/// Instead of using slice assignment, modify individual elements to avoid implicit size changes and potential confusion:
/// ```python
/// a[5] = 1
/// ```
/// 
/// ## Notes
/// - A common example of this, which violates the rule, is in-place list replacement using `[:]`.
/// This approach can replace the entire content of the list while maintaining the same object reference.
/// - Slice assignment is only in-place replacement of multiple array elements.
/// 
/// ## References
/// - [Python documentation: Assign](https://docs.python.org/3/library/ast.html#ast.Assign)
#[violation]
pub struct AssignmentToSubscriptSlice;

impl Violation for AssignmentToSubscriptSlice {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Assignment to a subscript slice found".to_string()
    }
}

/// WPS362
pub(crate) fn assignment_to_subscript_slice(checker: &mut Checker, stmt: &StmtAssign) {
    let StmtAssign { targets, .. } = stmt;
    targets
        .iter()
        .for_each(|target| check_subscript_slice(checker, target));
}

fn check_subscript_slice(checker: &mut Checker, expr: &Expr) {
    match expr {
        Expr::Subscript(expr_subscript) => {
            if !expr_subscript.slice.is_slice_expr() {
                return;
            }
            checker
                .diagnostics
                .push(Diagnostic::new(AssignmentToSubscriptSlice, expr.range()));
        }
        Expr::List(expr_list) => expr_list
            .elts
            .iter()
            .for_each(|item| check_subscript_slice(checker, item)),
        Expr::Tuple(expr_tuple) => expr_tuple
            .elts
            .iter()
            .for_each(|item| check_subscript_slice(checker, item)),
        _ => {}
    }
}
