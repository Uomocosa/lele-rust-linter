const OPT_OUT: &str = "// no test_usage necessary";

pub fn opt_out_at_end(content: &str) -> bool {
    content
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .is_some_and(|line| line.trim().starts_with(OPT_OUT))
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        assert!(rules::test_usage::TestUsage::opt_out_at_end(
            "fn f() {}\n\n// no test_usage necessary\n"
        ));
        assert!(!rules::test_usage::TestUsage::opt_out_at_end("fn f() {}\n"));
    }
}
