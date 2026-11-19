use crate::testing;

pub(crate) fn no_allow_violation_count_without_whitelist(code: &str) -> usize {
    testing::no_allow_violation_count(code, Vec::new())
}

#[cfg(test)]
mod tests {
    use super::no_allow_violation_count_without_whitelist;

    #[test]
    fn test_usage() {
        assert_eq!(
            no_allow_violation_count_without_whitelist("struct Foo { bar: u32 }"),
            0
        );
    }
}
