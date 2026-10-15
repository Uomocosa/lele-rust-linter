use crate::render_rule;
use crate::Checker;

pub fn explain(checkers: &[Box<dyn Checker>], code_or_name: &str) -> Option<String> {
    checkers
        .iter()
        .find(|c| c.code().eq_ignore_ascii_case(code_or_name) || c.name() == code_or_name)
        .map(|c| render_rule(c.as_ref()))
}

#[cfg(test)]
mod tests {
    use super::explain;
    use crate::checkers::build_checkers;

    #[test]
    fn test_usage() {
        let checkers = build_checkers();
        let by_name = explain(&checkers, "no_positional");
        let code = checkers
            .iter()
            .find(|checker| checker.name() == "no_positional")
            .map(|checker| checker.code().to_lowercase());
        let by_code = explain(&checkers, code.as_deref().unwrap_or(""));
        assert!(by_code.is_some());
        assert_eq!(by_code, by_name);
        assert!(explain(&checkers, "LELE999").is_none());
    }
}
