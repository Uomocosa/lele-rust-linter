use crate::RawCode;

pub fn ends_with_test_usage_opt_out(code: &RawCode) -> bool {
    let last_line = code.lines().rev().find(|line| !line.trim().is_empty());
    last_line.is_some_and(|line| line.trim().starts_with("// no test_usage necessary"))
}

#[cfg(test)]
mod tests {
    use super::ends_with_test_usage_opt_out;
    use crate::RawCode;

    #[test]
    fn test_usage() {
        assert!(ends_with_test_usage_opt_out(&RawCode(
            "fn a() {}\n// no test_usage necessary\n\n"
        )));
        assert!(!ends_with_test_usage_opt_out(&RawCode(
            "// no test_usage necessary\nfn a() {}\n"
        )));
    }
}
