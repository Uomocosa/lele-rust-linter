use std::collections::HashSet;
use std::path::{Component, Path, PathBuf};

use lele_lint::ItemMarker;

use crate::scan;

pub fn test_only_files(project: &scan::BevyProject) -> HashSet<PathBuf> {
    let mut found = HashSet::new();
    for source in project.sources() {
        for item in &source.file.items {
            let syn::Item::Mod(module) = item else {
                continue;
            };
            if module.content.is_some()
                || !module
                    .attrs
                    .iter()
                    .any(|attr| ItemMarker(attr).is_cfg_test())
            {
                continue;
            }
            found.extend(module_files(source.relative_path, module));
        }
    }
    found
}

// needed helper: the file(s) an out-of-line `mod name;` loads, honouring `#[path = ".."]`
fn module_files(declaring: &Path, module: &syn::ItemMod) -> Vec<PathBuf> {
    let dir = declaring.parent().unwrap_or_else(|| Path::new(""));
    if let Some(path) = path_attr(&module.attrs) {
        return vec![normalize(&dir.join(path))];
    }
    let name = module.ident.to_string();
    let is_mod_rs = declaring
        .file_name()
        .is_some_and(|file| file == "mod.rs" || file == "lib.rs" || file == "main.rs");
    let base = if is_mod_rs {
        dir.to_path_buf()
    } else {
        dir.join(declaring.file_stem().unwrap_or_default())
    };
    vec![
        normalize(&base.join(format!("{name}.rs"))),
        normalize(&base.join(&name).join("mod.rs")),
    ]
}

// needed helper: the string value of a `#[path = "..."]` attribute
fn path_attr(attrs: &[syn::Attribute]) -> Option<String> {
    attrs.iter().find_map(|attr| {
        let syn::Meta::NameValue(pair) = &attr.meta else {
            return None;
        };
        if !pair.path.is_ident("path") {
            return None;
        }
        let syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(value),
            ..
        }) = &pair.value
        else {
            return None;
        };
        Some(value.value())
    })
}

// needed helper: lexically resolves `.`/`..` so paths match the project's relative keys
fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for part in path.components() {
        match part {
            Component::ParentDir => {
                out.pop();
            }
            Component::CurDir => {}
            other => out.push(other),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::test_only_files;
    use crate::scan;
    use lele_lint::Project;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("lib.rs"),
            syn::parse_str(
                "#[cfg(test)] #[path = \"scene/tests.rs\"] mod scene_tests;
                 #[cfg(test)] mod helpers;
                 #[path = \"real.rs\"] mod real;",
            )
            .unwrap(),
        );
        project.parsed_files.insert(
            PathBuf::from("ui/row.rs"),
            syn::parse_str("#[cfg(test)] mod tests;").unwrap(),
        );
        let found = test_only_files(&scan::BevyProject(&project));
        assert!(found.contains(&PathBuf::from("scene/tests.rs")));
        assert!(found.contains(&PathBuf::from("helpers.rs")));
        assert!(found.contains(&PathBuf::from("ui/row/tests.rs")));
        assert!(!found.contains(&PathBuf::from("real.rs")));
    }
}
