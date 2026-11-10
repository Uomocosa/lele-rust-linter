use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::PathBuf;

use crate::rules;
use crate::Project;
use syn::visit::Visit;

pub fn trait_definitions(project: &Project) -> rules::trait_impl_co_location::DefinitionMap {
    let mut map: BTreeMap<String, BTreeSet<PathBuf>> = BTreeMap::new();
    for (rel_path, file) in &project.parsed_files {
        let mut hits = Vec::new();
        let mut visitor = rules::trait_placement::TraitVisitor(&mut hits);
        visitor.visit_file(file);
        for hit in hits {
            map.entry(hit.name).or_default().insert(rel_path.clone());
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::trait_definitions;
    use crate::Project;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("stock/__traits__/clock.rs"),
            syn::parse_str("pub trait Clock { fn now(&self) -> u64; }\n").unwrap(),
        );
        let defined = trait_definitions(&project);
        assert_eq!(defined.get("Clock").map(|paths| paths.len()), Some(1),);
        assert!(defined
            .get("Clock")
            .is_some_and(|paths| paths.contains(&PathBuf::from("stock/__traits__/clock.rs"))));
    }

    #[test]
    fn test_usage_skips_test_modules() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("stock/__traits__/clock.rs"),
            syn::parse_str(
                "#[cfg(test)] mod tests { pub trait TestOnly {} }\npub trait Clock {}\n",
            )
            .unwrap(),
        );
        let defined = trait_definitions(&project);
        assert_eq!(defined.len(), 1);
        assert!(defined.contains_key("Clock"));
    }
}

// no test_usage necessary
