use std::path::PathBuf;

use crate::methods;
use crate::rules;
use crate::testing;
use crate::AllowWhitelistEntry;
use crate::Project;

pub(crate) fn no_allow_violation_count(
    code: &str,
    clippy_allow_whitelist: Vec<AllowWhitelistEntry>,
) -> usize {
    let base = Project {
        src_dir: PathBuf::from("src"),
        clippy_allow_whitelist,
        ..Project::default()
    };
    let project = testing::parsed_project(base, &[("x.rs", code)]);
    methods::no_allow_attributes::rule_check(
        &rules::no_allow_attributes::NoAllowAttributes,
        &project,
    )
    .len()
}

#[cfg(test)]
mod tests {
    use super::no_allow_violation_count;

    #[test]
    fn test_usage() {
        assert_eq!(
            no_allow_violation_count("#[allow(dead_code)] struct Foo;", Vec::new()),
            1
        );
        assert_eq!(
            no_allow_violation_count("struct Foo { bar: u32 }", Vec::new()),
            0
        );
    }
}
