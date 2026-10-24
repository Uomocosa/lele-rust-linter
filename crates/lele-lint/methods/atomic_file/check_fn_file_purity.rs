use crate::rules;
use crate::Project;
use crate::RuleViolation;
use crate::Visibility;
use lele_snake_case::ToLeleSnakeCase;
use std::path::Path;

pub fn check_fn_file_purity(
    file: &syn::File,
    file_stem: &str,
    rel_path: &Path,
    project: &Project,
) -> Vec<RuleViolation> {
    let has_pub_fn = file
        .items
        .iter()
        .any(|item| matches!(item, syn::Item::Fn(f) if Visibility(&f.vis).is_exposed()));
    if !has_pub_fn {
        return Vec::new();
    }
    let mut violations = Vec::new();
    for item in &file.items {
        let (name, kind) = match item {
            syn::Item::Struct(s) if Visibility(&s.vis).is_exposed() => {
                (s.ident.to_string(), "struct")
            }
            syn::Item::Enum(e) if Visibility(&e.vis).is_exposed() => (e.ident.to_string(), "enum"),
            syn::Item::Const(c) if Visibility(&c.vis).is_exposed() => {
                (c.ident.to_string(), "const")
            }
            syn::Item::Static(s) if Visibility(&s.vis).is_exposed() => {
                (s.ident.to_string(), "static")
            }
            syn::Item::Type(t) if Visibility(&t.vis).is_exposed() => (t.ident.to_string(), "type"),
            syn::Item::Trait(t) if Visibility(&t.vis).is_exposed() => {
                (t.ident.to_string(), "trait")
            }
            syn::Item::Union(u) if Visibility(&u.vis).is_exposed() => {
                (u.ident.to_string(), "union")
            }
            _ => continue,
        };
        let home = if kind == "const" || kind == "static" {
            "nearest `constants.rs`"
        } else {
            &format!("`{}.rs`", name.to_snake_case())
        };
        violations.push(RuleViolation::from_file(project.src_dir.join(rel_path), rules::atomic_file::AtomicFile::CODE, format!(
                "SHAPE-F fn-file `{file_stem}.rs` must hold only the fn — move exposed `{kind} {name}` to {home} (O2-extraction)"
            )));
    }
    violations
}

// no test_usage necessary
