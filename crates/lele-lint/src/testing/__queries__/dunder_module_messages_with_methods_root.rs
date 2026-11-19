use std::path::PathBuf;

use crate::methods;
use crate::rules;
use crate::testing;

pub(crate) fn dunder_module_messages_with_methods_root(
    methods_root: &str,
    src: &str,
) -> Vec<String> {
    let mut project = testing::parsed_project(testing::dunder_project(), &[("stock/mod.rs", src)]);
    project.methods_dir = Some(PathBuf::from(methods_root));
    methods::dunder_module_name::rule_check(&rules::dunder_module_name::DunderModuleName, &project)
        .into_iter()
        .map(|violation| violation.message)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::dunder_module_messages_with_methods_root;

    #[test]
    fn test_usage() {
        assert_eq!(
            dunder_module_messages_with_methods_root(
                "bodies",
                "#[path = \"../bodies/mod.rs\"]\npub mod bodies;\n\npub mod methods;\n",
            )
            .len(),
            1
        );
    }
}
