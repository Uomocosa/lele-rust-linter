use crate::Linter;

pub fn explain(linter: &Linter, code_or_name: &str) -> Option<String> {
    linter
        .rules
        .iter()
        .find(|c| c.code().eq_ignore_ascii_case(code_or_name) || c.name() == code_or_name)
        .map(|c| Linter::render_rule(c.as_ref()))
}

#[cfg(test)]
mod tests {
    use super::explain;
    use crate::Linter;

    #[test]
    fn test_usage() {
        let linter = Linter::default();
        assert!(explain(&linter, "no_positional").is_some());
        assert!(explain(&linter, "LELE999").is_none());
    }
}
