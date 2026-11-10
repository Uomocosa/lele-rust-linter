use syn::visit::Visit;

use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &rules::no_stuttered_path::NoStutteredPath,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    let root_stems = rules::no_stuttered_path::NoStutteredPath::root_module_stems(project);

    for source in project.content_sources() {
        let rel_path = source.relative_path;
        let file = source.file;
        let mut hits = Vec::new();
        {
            let mut visitor = rules::no_stuttered_path::StutterVisitor {
                hits: &mut hits,
                stems: &root_stems,
            };
            visitor.visit_file(file);
        }
        for hit in std::mem::take(&mut hits) {
            violations.push(RuleViolation::from_site(project.absolute_path(source.origin, rel_path), hit.line, 0, rules::no_stuttered_path::NoStutteredPath::CODE, format!(
                    "stuttered path `{}` adds no information — import `{}` once and use `{}` directly",
                    hit.path, hit.ty, hit.ty
                )));
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::path::PathBuf;
    use syn::visit::Visit;

    use super::check;
    use crate::rules;
    use crate::Project;

    #[test]
    fn test_usage() {
        let rule = rules::no_stuttered_path::NoStutteredPath;
        let project = Project::default();
        assert!(check(&rule, &project).is_empty());
    }

    struct Collector<'a> {
        hits: &'a mut Vec<rules::no_stuttered_path::StutterHit>,
        stems: &'a HashSet<String>,
    }

    impl<'ast> Visit<'ast> for Collector<'_> {
        fn visit_path(&mut self, path: &'ast syn::Path) {
            rules::no_stuttered_path::NoStutteredPath::collect_stutter(path, self.stems, self.hits);
            syn::visit::visit_path(self, path);
        }
    }

    fn stems(names: &[&str]) -> HashSet<String> {
        names.iter().map(|s| (*s).to_string()).collect()
    }

    fn hit_paths(src: &str, stems: &HashSet<String>) -> Vec<String> {
        let file = syn::parse_file(src).unwrap();
        let mut hits = Vec::new();
        {
            let mut visitor = Collector {
                hits: &mut hits,
                stems,
            };
            visitor.visit_file(&file);
        }
        std::mem::take(&mut hits)
            .into_iter()
            .map(|hit| hit.path)
            .collect()
    }

    #[test]
    fn test_usage_flags_diagnostic_stutter() {
        let stems = stems(&["diagnostic"]);
        let src = "use crate::diagnostic;\npub fn f(d: Vec<diagnostic::Diagnostic>) {}\n";
        assert_eq!(hit_paths(src, &stems), vec!["diagnostic::Diagnostic"]);
    }

    #[test]
    fn test_usage_flags_type_stutter() {
        let stems = stems(&["severity"]);
        let src = "fn f(s: severity::Severity) {}\n";
        assert_eq!(hit_paths(src, &stems), vec!["severity::Severity"]);
    }

    #[test]
    fn test_usage_flags_stutter_prefix_of_longer_path() {
        let stems = stems(&["entry_kind"]);
        let src = "fn f() { let _ = entry_kind::EntryKind::File; }\n";
        assert_eq!(hit_paths(src, &stems), vec!["entry_kind::EntryKind::File"]);
    }

    #[test]
    fn test_usage_allows_std_paths() {
        let stems = stems(&["error"]);
        let src = "fn f(e: &dyn std::error::Error) {}\n";
        assert_eq!(hit_paths(src, &stems), Vec::<String>::new());
    }

    #[test]
    fn test_usage_allows_distinct_names() {
        let stems = stems(&["diagnostic"]);
        let src = "use crate::boxes;\nfn f(id: boxes::PlayerId) {}\nfn g(m: module_info::ModuleInfoMap) {}\n";
        assert_eq!(hit_paths(src, &stems), Vec::<String>::new());
    }

    #[test]
    fn test_usage_allows_subfolder_domain_prefix() {
        let stems = stems(&["player_id"]);
        let src = "use crate::player;\nfn f() { let _ = player::Player::new(); }\n";
        assert_eq!(hit_paths(src, &stems), Vec::<String>::new());
    }

    #[test]
    fn test_usage_flags_root_module_stutter_only() {
        let without = HashSet::new();
        let src = "fn f() { let _ = player::Player::new(); }\n";
        assert_eq!(hit_paths(src, &without), Vec::<String>::new());
        let with = stems(&["player"]);
        assert_eq!(hit_paths(src, &with), vec!["player::Player::new"]);
    }

    #[test]
    fn test_usage_allows_function_dispatch() {
        let stems = stems(&["config_new"]);
        let src = "fn f() { config_new::new(); }\n";
        assert_eq!(hit_paths(src, &stems), Vec::<String>::new());
    }

    #[test]
    fn test_usage_allows_external_paths() {
        let stems = stems(&["toml", "syn"]);
        let src = "fn f(v: toml::Value) {}\nfn g(item: syn::ItemMod) {}\n";
        assert_eq!(hit_paths(src, &stems), Vec::<String>::new());
    }

    #[test]
    fn test_usage_allows_use_items_and_crate_paths() {
        let stems = stems(&["diagnostic"]);
        let src = "use crate::diagnostic::Diagnostic;\nfn f(d: crate::diagnostic::Diagnostic) {}\n";
        assert_eq!(hit_paths(src, &stems), Vec::<String>::new());
    }

    #[test]
    fn test_usage_stem_name_from_rel_path() {
        assert_eq!(
            rules::no_stuttered_path::NoStutteredPath::stem_name(&PathBuf::from("diagnostic.rs")),
            Some("diagnostic".to_string())
        );
        assert_eq!(
            rules::no_stuttered_path::NoStutteredPath::stem_name(&PathBuf::from(
                "rules/no_crate_paths.rs"
            )),
            Some("no_crate_paths".to_string())
        );
    }
}
