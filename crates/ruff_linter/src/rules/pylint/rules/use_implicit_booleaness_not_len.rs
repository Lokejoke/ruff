use std::collections::HashSet;

use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::{Expr, ExprCall, ExprUnaryOp, UnaryOp};
use ruff_python_semantic::analyze::{
    self,
    type_inference::{PythonType, ResolvedPythonType},
};
use ruff_text_size::Ranged;

use crate::checkers::ast::{self, Checker};

/// ## What it does
///
/// ## Why is this bad?
/// Empty sequences are considered false in a boolean context.
/// You can either remove the call to 'len' (``if not x``)
/// or compare the length against a scalar (``if len(x) > 0``).
///
/// ## Example
/// ```python
/// fruits = ["orange", "apple"]
///
/// if len(fruits):  # [use-implicit-booleaness-not-len]
///     print(fruits)
/// ```
///
/// Use instead:
/// ```python
/// fruits = ["orange", "apple"]
///
/// if fruits:  # [use-implicit-booleaness-not-len]
///     print(fruits)
/// ```
#[violation]
pub struct UseImplicitBooleanessNotLen;

impl Violation for UseImplicitBooleanessNotLen {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Do not use `len(SEQUENCE)` without comparison to determine if a sequence is empty")
    }
}

/// PLC1802
pub(crate) fn use_implicit_booleaness_not_len(checker: &mut Checker, call: &ExprCall) {
    let semantic = checker.semantic();
    let ExprCall {
        range,
        func,
        arguments,
    } = call;

    if !semantic.in_boolean_test() {
        return;
    }

    if !semantic.match_builtin_expr(func, "len") {
        return;
    }

    let Some(argument) = arguments.find_positional(0) else {
        return;
    };

    if !is_collection_type(&ResolvedPythonType::from(argument)) {
        return;
    }

    // ResolvedPythonType::Atom(atom)

    // if matches!{argument, Expr::ListComp(_) | Expr::List(_)}

    // match semantic.current_expression_parent() {
    //     Some(parent) => {
    //         if !matches!(
    //             parent,
    //             Expr::BoolOp(_)
    //                 | Expr::UnaryOp(ExprUnaryOp {
    //                     op: UnaryOp::Not,
    //                     ..
    //                 })
    //         ) {
    //             return;
    //         }
    //     }
    //     None => {
    //         if !matches!(
    //             semantic.current_statement(),
    //             ruff_python_ast::Stmt::While(_)
    //                 | ruff_python_ast::Stmt::If(_)
    //                 | ruff_python_ast::Stmt::Assert(_)
    //         ) {
    //             return;
    //         }
    //     }
    // };

    checker
        .diagnostics
        .push(Diagnostic::new(UseImplicitBooleanessNotLen, range.range()));
}

fn is_collection_type(resolved_type: &ResolvedPythonType) -> bool {
    matches!(
        resolved_type,
        ResolvedPythonType::Atom(
            PythonType::String
                | PythonType::List
                | PythonType::Set
                | PythonType::Dict
                | PythonType::Tuple
                | PythonType::Generator
        )
    )
}
