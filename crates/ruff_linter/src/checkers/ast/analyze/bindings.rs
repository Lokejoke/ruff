use ruff_diagnostics::{Diagnostic, Fix};
use ruff_text_size::Ranged;

use crate::checkers::ast::Checker;
use crate::codes::Rule;
use crate::rules::{
    flake8_import_conventions, flake8_pyi, flake8_type_checking, pyflakes, pylint, ruff, wps_light,
};

/// Run lint rules over the [`Binding`]s.
pub(crate) fn bindings(checker: &mut Checker) {
    if !checker.any_enabled(&[
        Rule::AssignmentInAssert,
        Rule::InvalidAllFormat,
        Rule::InvalidAllObject,
        Rule::NonAsciiName,
        Rule::ReservedArgumentAsVariable,
        Rule::UnaliasedCollectionsAbcSetImport,
        Rule::UnconventionalImportAlias,
        Rule::UnsortedDunderSlots,
        Rule::UnusedVariable,
        Rule::UnquotedTypeAlias,
        Rule::UsedDummyVariable,
        Rule::ConsecutiveUnderscoresInName,
    ]) {
        return;
    }

    for binding in &*checker.semantic.bindings {
        if checker.enabled(Rule::UnusedVariable) {
            if binding.kind.is_bound_exception()
                && binding.is_unused()
                && !checker
                    .settings
                    .dummy_variable_rgx
                    .is_match(binding.name(checker.source()))
            {
                let mut diagnostic = Diagnostic::new(
                    pyflakes::rules::UnusedVariable {
                        name: binding.name(checker.source()).to_string(),
                    },
                    binding.range(),
                );
                diagnostic.try_set_fix(|| {
                    pyflakes::fixes::remove_exception_handler_assignment(binding, checker.locator)
                        .map(Fix::safe_edit)
                });
                checker.diagnostics.push(diagnostic);
            }
        }
        if checker.enabled(Rule::InvalidAllFormat) {
            if let Some(diagnostic) = pylint::rules::invalid_all_format(binding) {
                checker.diagnostics.push(diagnostic);
            }
        }
        if checker.enabled(Rule::InvalidAllObject) {
            if let Some(diagnostic) = pylint::rules::invalid_all_object(binding) {
                checker.diagnostics.push(diagnostic);
            }
        }
        if checker.enabled(Rule::NonAsciiName) {
            if let Some(diagnostic) = pylint::rules::non_ascii_name(binding, checker.locator) {
                checker.diagnostics.push(diagnostic);
            }
        }
        if checker.enabled(Rule::UnconventionalImportAlias) {
            if let Some(diagnostic) = flake8_import_conventions::rules::unconventional_import_alias(
                checker,
                binding,
                &checker.settings.flake8_import_conventions.aliases,
            ) {
                checker.diagnostics.push(diagnostic);
            }
        }
        if checker.enabled(Rule::UnaliasedCollectionsAbcSetImport) {
            if let Some(diagnostic) =
                flake8_pyi::rules::unaliased_collections_abc_set_import(checker, binding)
            {
                checker.diagnostics.push(diagnostic);
            }
        }
        if checker.enabled(Rule::UnquotedTypeAlias) {
            if let Some(diagnostics) =
                flake8_type_checking::rules::unquoted_type_alias(checker, binding)
            {
                checker.diagnostics.extend(diagnostics);
            }
        }
        if checker.enabled(Rule::UnsortedDunderSlots) {
            if let Some(diagnostic) = ruff::rules::sort_dunder_slots(checker, binding) {
                checker.diagnostics.push(diagnostic);
            }
        }
        if checker.enabled(Rule::UsedDummyVariable) {
            if let Some(diagnostic) = ruff::rules::used_dummy_variable(checker, binding) {
                checker.diagnostics.push(diagnostic);
            }
        }
        if checker.enabled(Rule::AssignmentInAssert) {
            if let Some(diagnostic) = ruff::rules::assignment_in_assert(checker, binding) {
                checker.diagnostics.push(diagnostic);
            }
        }
        if checker.enabled(Rule::ConsecutiveUnderscoresInName) {
            if let Some(diagnostic) =
                wps_light::rules::consecutive_underscores_in_name(checker.locator(), binding)
            {
                checker.diagnostics.push(diagnostic);
            }
        }
        if checker.enabled(Rule::ReservedArgumentAsVariable) {
            if let Some(dignostic) =
                wps_light::rules::reserved_argument_as_variable(checker, binding)
            {
                checker.diagnostics.push(dignostic);
            }
        }
    }
}
