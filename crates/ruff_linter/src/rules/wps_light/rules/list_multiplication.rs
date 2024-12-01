use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_ast::{Expr, ExprBinOp, ExprCall, ExprName, Operator};
use ruff_python_semantic::analyze::typing::find_binding_value;
use ruff_python_semantic::{
    analyze::{
        type_inference::{NumberLike, PythonType, ResolvedPythonType},
        typing,
    },
    BindingId, SemanticModel,
};

use crate::checkers::ast::Checker;

/// ## What it does
/// Checks for multiplication of list of mutable objects
///
/// ## Why is this bad?
/// Mutable objects in a list can lead to unexpected behavior when the list is multiplied.
/// Each element in the resulting list will refer to the same mutable object. Any modification
/// to one element affects all instances in the multiplied list. This can lead to bugs that are difficult to trace.
///
/// ## Example
/// ```python
/// row = [""] * 3
/// tic_tac_toe = [row] * 3
///
/// tic_tac_toe[0][0] = "X"
/// tic_tac_toe  # [["X", "", ""], ["X", "", ""], ["", "", ""]]
/// ```
///
/// Use instead:
///
/// ```python
/// row = [[""] * 3 for _ in range(3)]
///
/// tic_tac_toe[0][0] = "X"
/// tic_tac_toe  # [["X", "", ""], ["", "", ""], ["", "", ""]]
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct ListMultiplication;

impl Violation for ListMultiplication {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Multiplication of list of mutable objects".to_string()
    }
}

/// WPS435
pub(crate) fn list_multiplication(checker: &mut Checker, expr: &ExprBinOp) {
    let ExprBinOp {
        range, left, right, ..
    } = expr;
    let semantic = checker.semantic();

    let operands = [left, right];
    if operands
        .into_iter()
        .filter(|expr| is_integer(expr, semantic))
        .count()
        != 1
    {
        return;
    }
    if operands
        .into_iter()
        .filter(|expr| is_nested_list_like(expr, semantic))
        .count()
        != 1
    {
        return;
    }

    checker
        .diagnostics
        .push(Diagnostic::new(ListMultiplication, *range));
}

fn is_integer(expr: &Expr, semantic: &SemanticModel) -> bool {
    // Check if the expression directly resolves to an integer type
    if matches!(
        ResolvedPythonType::from(expr),
        ResolvedPythonType::Atom(PythonType::Number(NumberLike::Integer))
    ) {
        return true;
    }

    // Check if the expression is a name and resolve its binding
    let Some(expr_name) = expr.as_name_expr() else {
        return false;
    };
    let Some(id) = semantic.only_binding(expr_name) else {
        return false;
    };

    // Verify if the binding resolves to an integer
    typing::is_int(semantic.binding(id), semantic)
}

fn is_nested_list_like(expr: &Expr, semantic: &SemanticModel) -> bool {
    match expr {
        Expr::ListComp(expr_list_comp) => is_terminal_list_like(&expr_list_comp.elt, semantic),
        Expr::Name(expr_name) => get_name_value(expr_name, semantic)
            .is_some_and(|value| is_nested_list_like(value, semantic)),
        Expr::List(expr_list) => expr_list
            .into_iter()
            .any(|item| is_terminal_list_like(item, semantic)),
        _ => false,
    }
}

fn is_terminal_list_like(expr: &Expr, semantic: &SemanticModel) -> bool {
    match expr {
        Expr::BinOp(ExprBinOp {
            op: Operator::Mult,
            left,
            right,
            ..
        }) => {
            let operands = [left, right];
            if operands
                .into_iter()
                .filter(|expr| is_integer(expr, semantic))
                .count()
                != 1
            {
                return false;
            }
            if operands
                .into_iter()
                .filter(|expr| is_terminal_list_like(expr, semantic))
                .count()
                != 1
            {
                return false;
            }
            true
        }
        Expr::Call(ExprCall { func, .. }) => semantic.match_builtin_expr(func, "range"),
        Expr::Name(expr_name) => get_name_value(expr_name, semantic)
            .is_some_and(|value| is_terminal_list_like(value, semantic)),
        _ => typing::is_mutable_expr(expr, semantic),
    }
}

fn get_name_value<'a>(name: &ExprName, semantic: &'a SemanticModel) -> Option<&'a Expr> {
    let scope = semantic.current_scope();
    let bindings: Vec<BindingId> = scope.get_all(name.id()).collect();
    let [binding_id] = bindings.as_slice() else {
        return None;
    };
    let binding = semantic.binding(*binding_id);
    find_binding_value(binding, semantic)
}
