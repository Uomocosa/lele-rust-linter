use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::PathBuf;

use crate::rules;
use crate::Project;
use syn::visit::Visit;

pub fn type_definitions(project: &Project) -> rules::trait_impl_co_location::DefinitionMap {
    let mut map: BTreeMap<String, BTreeSet<PathBuf>> = BTreeMap::new();
    for (rel_path, file) in &project.parsed_files {
        let mut names = Vec::new();
        let mut visitor = rules::trait_impl_co_location::TypeVisitor(&mut names);
        visitor.visit_file(file);
        for name in names {
            map.entry(name).or_default().insert(rel_path.clone());
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use super::type_definitions;
    use crate::Project;

    fn definitions(files: &[(&str, &str)]) -> BTreeMap<String, Vec<PathBuf>> {
        let mut project = Project::default();
        for (path, source) in files {
            let file: syn::File = syn::parse_str(source).unwrap();
            project.parsed_files.insert(PathBuf::from(path), file);
        }
        type_definitions(&project)
            .into_iter()
            .map(|(name, paths)| (name, paths.into_iter().collect()))
            .collect()
    }

    #[test]
    fn test_usage() {
        let defined = definitions(&[
            (
                "stock/unix_time.rs",
                "pub struct UnixTime { pub seconds: u64 }\n",
            ),
            ("stock/shape.rs", "pub enum Shape { Circle }\n"),
            ("stock/pair.rs", "pub union Pair { raw: u64 }\n"),
            ("stock/meters.rs", "pub type Meters = u64;\n"),
        ]);
        assert_eq!(
            defined.get("UnixTime").map(Vec::as_slice),
            Some(&[PathBuf::from("stock/unix_time.rs")][..])
        );
        assert_eq!(
            defined.get("Shape").map(Vec::as_slice),
            Some(&[PathBuf::from("stock/shape.rs")][..])
        );
        assert_eq!(
            defined.get("Pair").map(Vec::as_slice),
            Some(&[PathBuf::from("stock/pair.rs")][..])
        );
        assert_eq!(
            defined.get("Meters").map(Vec::as_slice),
            Some(&[PathBuf::from("stock/meters.rs")][..])
        );
    }

    #[test]
    fn test_usage_skips_test_modules_and_test_items() {
        let defined = definitions(&[
            (
                "stock/unix_time.rs",
                "#[cfg(test)] mod tests { pub struct TestOnly; }\n#[cfg(test)] pub struct TestStruct;\npub struct UnixTime;\n",
            ),
        ]);
        assert_eq!(defined.len(), 1);
        assert!(defined.contains_key("UnixTime"));
    }

    #[test]
    fn test_usage_traits_are_not_types() {
        let defined = definitions(&[("stock/__traits__/clock.rs", "pub trait Clock {}\n")]);
        assert!(defined.is_empty());
    }
}

// no test_usage necessary
