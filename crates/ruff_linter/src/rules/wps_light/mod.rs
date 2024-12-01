//! Rules from [wps-light](https://pypi.org/project/wps-light/).
pub(crate) mod rules;

#[cfg(test)]
mod tests {
    use std::convert::AsRef;
    use std::path::Path;

    use anyhow::Result;
    use test_case::test_case;

    use crate::registry::Rule;
    use crate::test::test_path;
    use crate::{assert_messages, settings};

    #[test_case(Rule::ListMultiplication, Path::new("WPS435.py"))]
    #[test_case(Rule::ReservedArgumentAsVariable, Path::new("WPS117.py"))]
    #[test_case(Rule::UnderscoresInNumber, Path::new("WPS303.py"))]
    #[test_case(Rule::AssignmentToSubscriptSlice, Path::new("WPS362.py"))]
    fn rules(rule_code: Rule, path: &Path) -> Result<()> {
        let snapshot = format!("{}_{}", rule_code.as_ref(), path.to_string_lossy());
        let diagnostics = test_path(
            Path::new("wps_light").join(path).as_path(),
            &settings::LinterSettings::for_rule(rule_code),
        )?;
        assert_messages!(snapshot, diagnostics);
        Ok(())
    }
}
