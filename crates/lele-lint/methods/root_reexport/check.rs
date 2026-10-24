use std::path::PathBuf;

use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check(_self: &rules::root_reexport::RootReexport, project: &Project) -> Vec<RuleViolation> {
    let lib = match project.parsed_files.get(&PathBuf::from("lib.rs")) {
        Some(lib) => lib,
        None => return Vec::new(),
    };
    let pub_mods = rules::root_reexport::RootReexport::public_modules(lib);
    let reexported = rules::root_reexport::RootReexport::root_reexports(lib);
    let mut violations = Vec::new();

    for (rel_path, file) in &project.parsed_files {
        if rel_path.components().count() != 1 {
            continue;
        }
        let stem = rel_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        if matches!(stem, "lib" | "main" | "mod") {
            continue;
        }
        if !pub_mods.contains(stem) {
            continue;
        }
        for missing in
            rules::root_reexport::RootReexport::missing_reexports(stem, file, &reexported)
        {
            violations.push(RuleViolation::from_site(project.src_dir.join(rel_path), missing.line, 0, rules::root_reexport::RootReexport::CODE, format!(
                    "public type `{}` in root module `{stem}` is not re-exported at the crate root — add `pub use {stem}::{};` to lib.rs",
                    missing.ty, missing.ty
                )));
        }
        for missing in rules::root_reexport::RootReexport::missing_fn_flatten(stem, file, lib) {
            violations.push(RuleViolation::from_file(
                project.src_dir.join("lib.rs"),
                rules::root_reexport::RootReexport::CODE,
                missing,
            ));
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::check;
    use crate::rules;
    use crate::Project;

    #[test]
    fn test_usage() {
        let rule = rules::root_reexport::RootReexport;
        let project = Project::default();
        assert!(check(&rule, &project).is_empty());
    }

    fn parse(src: &str) -> syn::File {
        syn::parse_str(src).unwrap()
    }

    #[test]
    fn test_usage_flags_missing_reexport() {
        let lib = parse("pub mod diagnostic;\n");
        let file = parse("pub struct Diagnostic { pub x: u32 }\n");
        let missing = rules::root_reexport::RootReexport::missing_reexports(
            "diagnostic",
            &file,
            &rules::root_reexport::RootReexport::root_reexports(&lib),
        );
        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0].ty, "Diagnostic");
        assert!(rules::root_reexport::RootReexport::public_modules(&lib).contains("diagnostic"));
    }

    #[test]
    fn test_usage_allows_reexported_type() {
        let lib = parse("pub mod diagnostic;\npub use diagnostic::Diagnostic;\n");
        let file = parse("pub struct Diagnostic { pub x: u32 }\n");
        let missing = rules::root_reexport::RootReexport::missing_reexports(
            "diagnostic",
            &file,
            &rules::root_reexport::RootReexport::root_reexports(&lib),
        );
        assert!(missing.is_empty());
    }

    #[test]
    fn test_usage_allows_function_modules() {
        let lib = parse("pub mod discover;\n");
        let file = parse("pub fn discover() {}\n");
        let missing = rules::root_reexport::RootReexport::missing_reexports(
            "discover",
            &file,
            &rules::root_reexport::RootReexport::root_reexports(&lib),
        );
        assert!(missing.is_empty());
    }

    #[test]
    fn test_usage_allows_non_stutter_types() {
        let lib = parse("pub mod module_info;\n");
        let file = parse("pub type ModuleInfoMap = u32;\n");
        let missing = rules::root_reexport::RootReexport::missing_reexports(
            "module_info",
            &file,
            &rules::root_reexport::RootReexport::root_reexports(&lib),
        );
        assert!(missing.is_empty());
    }

    #[test]
    fn test_usage_ignores_private_modules() {
        let lib = parse("mod config_load;\n");
        assert!(!rules::root_reexport::RootReexport::public_modules(&lib).contains("config_load"));
    }

    #[test]
    fn test_usage_flags_missing_fn_flatten() {
        let lib = parse("pub mod discover;\n");
        let file = parse("pub fn discover() {}\n");
        let missing =
            rules::root_reexport::RootReexport::missing_fn_flatten("discover", &file, &lib);
        assert_eq!(missing.len(), 2);
        assert!(missing[0].contains("private `mod discover;`"));
        assert!(missing[1].contains("pub use discover::discover;"));
    }

    #[test]
    fn test_usage_allows_flat_fn() {
        let lib = parse("mod discover;\npub use discover::discover;\n");
        let file = parse("pub fn discover() {}\n");
        assert_eq!(
            rules::root_reexport::RootReexport::missing_fn_flatten("discover", &file, &lib).len(),
            0
        );
    }

    #[test]
    fn test_usage_ignores_non_stutter_fn() {
        let lib = parse("pub mod discover;\n");
        let file = parse("pub fn run() {}\n");
        assert_eq!(
            rules::root_reexport::RootReexport::missing_fn_flatten("discover", &file, &lib).len(),
            0
        );
    }
}
