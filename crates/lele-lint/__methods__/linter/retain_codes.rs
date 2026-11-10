use crate::Linter;
use crate::Rule;

pub fn retain_codes(linter: &mut Linter, codes: &[String]) -> Vec<String> {
    let matches =
        |rule: &dyn Rule, code: &str| rule.code().eq_ignore_ascii_case(code) || rule.name() == code;
    let unknown = codes
        .iter()
        .filter(|code| !linter.rules.iter().any(|rule| matches(rule.as_ref(), code)))
        .cloned()
        .collect();
    linter
        .rules
        .retain(|rule| codes.iter().any(|code| matches(rule.as_ref(), code)));
    unknown
}

#[cfg(test)]
mod tests {
    use crate::Linter;

    #[test]
    fn test_usage() {
        let mut linter = Linter::default();
        let unknown = linter.retain_codes(&[
            "lele001".to_string(),
            "no_comments".to_string(),
            "LELE999".to_string(),
        ]);
        assert_eq!(unknown, vec!["LELE999".to_string()]);
        let mut codes: Vec<&str> = linter.rules.iter().map(|rule| rule.code()).collect();
        codes.sort_unstable();
        assert_eq!(codes, vec!["LELE001", "LELE031"]);
    }
}
