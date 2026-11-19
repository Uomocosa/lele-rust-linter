use std::path::Path;

use crate::methods;
use crate::testing;
use crate::Project;

pub(crate) fn dunder_reexport_suggestions(rel_path: &str, source: &str) -> Vec<String> {
    let project: Project =
        testing::parsed_project(testing::dunder_project(), &[(rel_path, source)]);
    let file = project.get_parsed(Path::new(rel_path)).unwrap();
    methods::dunder_module_name::dunder_reexport_hits(Path::new(rel_path), file, &project)
        .into_iter()
        .map(|hit| hit.suggestion)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::dunder_reexport_suggestions;

    #[test]
    fn test_usage() {
        assert_eq!(
            dunder_reexport_suggestions("greeting/mod.rs", "pub use traits::Greetable;\n"),
            vec!["greeting::traits::Greetable".to_string()]
        );
    }
}
