use std::iter::zip;

use ruff_python_semantic::analyze::typing::find_binding_value;
use ruff_python_semantic::{BindingId, SemanticModel};

use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::ExprCompare;
use ruff_python_ast::{CmpOp, Expr};
use ruff_text_size::Ranged;

use crate::checkers::ast::Checker;

/// ## What it does
/// Checks for `in` or `not in` use on single item container
///
/// ## Why is this bad?
/// `in` comparison with container containing only one item
/// looks like an overhead and unneeded complexity.
///
/// Consider using `==` instead.
///
/// ## Example
/// ```python
/// a in {"yes"}
/// ```
///
/// Use instead:
/// ```python
/// a == "yes"
/// ```
#[violation]
pub struct InCompareWithSingleItemContainer;

impl Violation for InCompareWithSingleItemContainer {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Avoid comparing to a single item container with `in` operator".to_string()
    }
}

/// WPS525
pub(crate) fn in_compare_with_single_item_container(checker: &mut Checker, compare: &ExprCompare) {
    let ExprCompare {
        ops, comparators, ..
    } = compare;
    let semantic = checker.semantic();
    let diagnostics: Vec<Diagnostic> = zip(ops, comparators)
        .filter(|(op, comparator)| {
            // Check if the operator is CmpOp::In or CmpOp::NotIn
            matches!(op, CmpOp::In | CmpOp::NotIn)
                // Or check if the comparator is a single item container
                || is_single_item_container(semantic, comparator)
        })
        .map(|(_, comparator)| {
            // Create a diagnostic for each match
            Diagnostic::new(InCompareWithSingleItemContainer, comparator.range())
        })
        .collect();

    // Extend the checker diagnostics with the new diagnostics
    checker.diagnostics.extend(diagnostics);
}

fn is_single_item_container(semantic: &SemanticModel, expr: &Expr) -> bool {
    match expr {
        Expr::Dict(expr_dict) => expr_dict.len() == 1,
        Expr::Set(expr_set) => expr_set.len() == 1,
        Expr::Name(expr_name) => {
            let scope = semantic.current_scope();
            let bindings: Vec<BindingId> = scope.get_all(expr_name.id()).collect();
            let [binding_id] = bindings.as_slice() else {
                return false;
            };
            let binding = semantic.binding(*binding_id);
            find_binding_value(binding, semantic)
                .is_some_and(|value| is_single_item_container(semantic, value))
        }
        Expr::List(expr_list) => expr_list.len() == 1,
        Expr::Tuple(expr_tuple) => expr_tuple.len() == 1,
        _ => false,
    }
}
