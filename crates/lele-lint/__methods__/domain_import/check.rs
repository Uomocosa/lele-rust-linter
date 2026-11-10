use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check(_self: &rules::domain_import::DomainImport, project: &Project) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for source in project.content_sources() {
        let rel_path = source.relative_path;
        let file = source.file;
        for item in &file.items {
            if let syn::Item::Use(item_use) = item {
                if let Some(msg) = rules::domain_import::DomainImport::check_import(
                    item_use,
                    &project.dunder_paths,
                ) {
                    violations.push(RuleViolation::from_site(
                        project.absolute_path(source.origin, rel_path),
                        rules::domain_import::DomainImport::find_use_line(item_use),
                        0,
                        rules::domain_import::DomainImport::CODE,
                        msg,
                    ));
                }
            }
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use crate::DunderPath;
    use std::path::PathBuf;

    use super::check;
    use crate::rules;
    use crate::rules::domain_import::DomainImport;
    use crate::Entry;
    use crate::EntryKind;
    use crate::Location;
    use crate::Project;

    #[test]
    fn test_usage() {
        let rule = rules::domain_import::DomainImport;
        let project = Project::default();
        assert!(check(&rule, &project).is_empty());
    }
    use syn::parse_quote;

    #[test]
    fn test_usage_flags_direct_type_import() {
        let u: syn::ItemUse = parse_quote! { use crate::player::PlayerId; };
        assert!(
            rules::domain_import::DomainImport::check_import(&u, &DunderPath::defaults()).is_some()
        );
    }

    #[test]
    fn test_usage_allows_domain_import() {
        let u: syn::ItemUse = parse_quote! { use crate::player; };
        assert!(
            rules::domain_import::DomainImport::check_import(&u, &DunderPath::defaults()).is_none()
        );
    }

    #[test]
    fn test_usage_flags_subfolder_import() {
        let u: syn::ItemUse = parse_quote! { use crate::clicker::plugin::ClickerPlugin; };
        assert!(
            rules::domain_import::DomainImport::check_import(&u, &DunderPath::defaults()).is_some()
        );
    }

    #[test]
    fn test_usage_allows_super_import() {
        let u: syn::ItemUse = parse_quote! { use super::player_new; };
        assert!(
            rules::domain_import::DomainImport::check_import(&u, &DunderPath::defaults()).is_none()
        );
    }

    #[test]
    fn test_usage_exempts_pub_use() {
        let u: syn::ItemUse = parse_quote! { pub use player::Player; };
        assert!(rules::domain_import::DomainImport::is_pub_use(&u));
    }

    #[test]
    fn test_usage_allows_direct_stutter_import() {
        let u: syn::ItemUse = parse_quote! { use crate::diagnostic::Diagnostic; };
        assert!(
            rules::domain_import::DomainImport::check_import(&u, &DunderPath::defaults()).is_none()
        );
    }

    #[test]
    fn test_usage_still_flags_direct_non_stutter_import() {
        let u: syn::ItemUse = parse_quote! { use crate::module_info::ModuleInfoMap; };
        assert!(
            rules::domain_import::DomainImport::check_import(&u, &DunderPath::defaults()).is_some()
        );
    }

    #[test]
    fn test_usage_reports_offending_line() {
        let dir = tempfile::tempdir().unwrap();
        let source = "use crate::player;\nuse crate::player::PlayerId;\n";
        let absolute_path = dir.path().join("probe.rs");
        std::fs::write(&absolute_path, source).unwrap();
        let mut project = Project {
            src_dir: dir.path().to_path_buf(),
            ..Project::default()
        };
        project.entries.push(Entry {
            relative_path: PathBuf::from("probe.rs"),
            absolute_path,
            kind: EntryKind::File,
        });
        project
            .parsed_files
            .insert(PathBuf::from("probe.rs"), syn::parse_str(source).unwrap());
        let violations = check(&DomainImport, &project);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].location, Location::Site { line: 2, col: 0 });
    }
}

// no test_usage necessary
