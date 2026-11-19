use crate::rules;
use crate::testing;

pub(crate) fn dunder_module_messages(src: &str) -> Vec<String> {
    testing::violation_messages(
        &rules::dunder_module_name::DunderModuleName,
        &[("stock/mod.rs", src)],
    )
}

#[cfg(test)]
mod tests {
    use super::dunder_module_messages;

    #[test]
    fn test_usage() {
        assert_eq!(
            dunder_module_messages("#[path = \"__basic__/mod.rs\"]\nmod basic;\n"),
            Vec::<String>::new()
        );
    }
}
