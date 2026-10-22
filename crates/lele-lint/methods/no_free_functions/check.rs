use lele_snake_case::ToLeleSnakeCase;
use std::collections::BTreeSet;
use std::path::Path;
use syn::spanned::Spanned;

use crate::checkers;
use crate::ItemMarker;
use crate::Project;
use crate::RuleViolation;

const CODE: &str = checkers::no_free_functions::NoFreeFunctions::CODE;

const WRAPPERS: [&str; 4] = ["Result", "Option", "Vec", "Box"];

const EXEMPT_DIRS: [&str; 3] = ["bevy_systems", "bin", "tests"];

const EXEMPT_FILES: [&str; 5] = ["mod.rs", "lib.rs", "main.rs", "build.rs", "constants.rs"];

pub fn check(
    _self: &checkers::no_free_functions::NoFreeFunctions,
    project: &Project,
) -> Vec<RuleViolation> {
    let local_types = local_type_names(project);
    let mut violations = Vec::new();
    let mut matched_entry_points = BTreeSet::new();

    for (rel_path, file) in &project.parsed_files {
        if is_exempt_file(rel_path, project) {
            continue;
        }
        let absolute = project.src_dir.join(rel_path);
        for item in &file.items {
            let syn::Item::Fn(func) = item else {
                continue;
            };
            if matches!(func.vis, syn::Visibility::Inherited)
                || func.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test())
                || func.attrs.iter().any(|attr| ItemMarker(attr).is_test())
            {
                continue;
            }
            let name = func.sig.ident.to_string();
            let entry_point = project.entry_points.iter().position(|entry| {
                entry.function == name && project.root.join(&entry.file) == absolute
            });
            if let Some(index) = entry_point {
                matched_entry_points.insert(index);
                continue;
            }
            violations.push(RuleViolation::site(
                absolute.clone(),
                func.sig.fn_token.span().start().line,
                0,
                CODE,
                owner_hint(&func.sig, &local_types),
            ));
        }
    }

    for (index, entry) in project.entry_points.iter().enumerate() {
        let path = project.root.join(&entry.file);
        if !matched_entry_points.contains(&index) && path.is_file() {
            violations.push(RuleViolation::file(
                path,
                CODE,
                format!(
                    "stale [[lele.lint.entry_points]] entry: no free `pub fn {}` in `{}` — delete the entry",
                    entry.function,
                    entry.file.display()
                ),
            ));
        }
    }

    violations
}

// needed helper: the exemption list is specific to this rule
fn is_exempt_file(rel_path: &Path, project: &Project) -> bool {
    let file_name = rel_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    EXEMPT_FILES.contains(&file_name)
        || project.is_in_dunder_dir(rel_path)
        || rel_path.components().any(|component| {
            component
                .as_os_str()
                .to_str()
                .is_some_and(|dir| EXEMPT_DIRS.contains(&dir))
        })
}

// needed helper: every struct/enum/trait the crate defines is a candidate owner
fn local_type_names(project: &Project) -> BTreeSet<String> {
    project
        .parsed_files
        .values()
        .flat_map(|file| file.items.iter())
        .filter_map(|item| match item {
            syn::Item::Struct(item) => Some(item.ident.to_string()),
            syn::Item::Enum(item) => Some(item.ident.to_string()),
            syn::Item::Trait(item) => Some(item.ident.to_string()),
            _ => None,
        })
        .collect()
}

// needed helper: names the owner candidates so the fix is a choice between at most two types
fn owner_hint(sig: &syn::Signature, local_types: &BTreeSet<String>) -> String {
    fn type_name(ty: &syn::Type) -> Option<String> {
        match ty {
            syn::Type::Reference(reference) => type_name(&reference.elem),
            syn::Type::Paren(paren) => type_name(&paren.elem),
            syn::Type::Slice(slice) => type_name(&slice.elem),
            syn::Type::TraitObject(object) => object.bounds.iter().find_map(|bound| match bound {
                syn::TypeParamBound::Trait(bound) => bound
                    .path
                    .segments
                    .last()
                    .map(|segment| segment.ident.to_string()),
                _ => None,
            }),
            syn::Type::Path(path) => {
                let segment = path.path.segments.last()?;
                let name = segment.ident.to_string();
                if !WRAPPERS.contains(&name.as_str()) {
                    return Some(name);
                }
                let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
                    return None;
                };
                match args.args.first()? {
                    syn::GenericArgument::Type(inner) => type_name(inner),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    let first_arg = sig.inputs.first().and_then(|arg| match arg {
        syn::FnArg::Typed(typed) => type_name(&typed.ty),
        syn::FnArg::Receiver(_) => None,
    });
    let returned = match &sig.output {
        syn::ReturnType::Type(_, ty) => type_name(ty),
        syn::ReturnType::Default => None,
    };
    let local: Vec<(String, &str)> = [(first_arg, "first argument"), (returned, "return type")]
        .into_iter()
        .filter_map(|(name, source)| name.map(|name| (name, source)))
        .filter(|(name, _)| local_types.contains(name))
        .fold(Vec::new(), |mut acc, (name, source)| {
            if !acc.iter().any(|(seen, _)| *seen == name) {
                acc.push((name, source));
            }
            acc
        });

    let name = &sig.ident;
    let advice = match local.as_slice() {
        [] => format!(
            "no candidate owner is a crate type — wrap the first argument in a newtype (or bundle the arguments in a struct) and make it the owner of `{name}`"
        ),
        [(owner, source)] => format!(
            "move it to `#[atomic_delegates] impl {owner}` ({source}; an associated fn when it takes or returns a collection of `{owner}`), body in `methods/{}/{name}.rs`",
            owner.to_snake_case()
        ),
        candidates => format!(
            "pick its owner: {} — first argument for an operation, return type for a constructor",
            candidates
                .iter()
                .map(|(owner, source)| format!("`{owner}` ({source})"))
                .collect::<Vec<_>>()
                .join(" or ")
        ),
    };
    format!(
        "free fn `{name}` has no owner type; {advice}; then run `lele_lint --sync-methods`. A true process/task entry point goes in [[lele.lint.entry_points]]"
    )
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::check;
    use crate::checkers::no_free_functions::NoFreeFunctions;
    use crate::EntryPoint;
    use crate::Project;

    fn project_with(files: &[(&str, &str)]) -> Project {
        let mut project = Project {
            root: PathBuf::from("/crate"),
            src_dir: PathBuf::from("/crate/src"),
            ..Project::default()
        };
        for (path, source) in files {
            project
                .parsed_files
                .insert(PathBuf::from(path), syn::parse_str(source).unwrap());
        }
        project
    }

    #[test]
    fn test_usage() {
        let project = project_with(&[
            ("player/player.rs", "pub struct Player;"),
            (
                "player/heal.rs",
                "pub fn heal(player: &mut Player, amount: u32) {}",
            ),
        ]);
        let violations = check(&NoFreeFunctions, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("impl Player"));
        assert!(violations[0].message.contains("methods/player/heal.rs"));
    }

    #[test]
    fn test_return_type_and_two_candidates() {
        let project = project_with(&[
            ("a.rs", "pub struct Client; pub struct Config;"),
            (
                "connect.rs",
                "pub fn connect(host: &str) -> Result<Client, E> {}",
            ),
            (
                "make.rs",
                "pub fn build(config: &Config) -> Option<Client> {}",
            ),
            ("split.rs", "pub fn split(bytes: &[u8]) -> Vec<u8> {}"),
        ]);
        let messages: Vec<String> = check(&NoFreeFunctions, &project)
            .into_iter()
            .map(|v| v.message)
            .collect();
        assert!(messages
            .iter()
            .any(|m| m.contains("impl Client` (return type")));
        assert!(messages
            .iter()
            .any(|m| m.contains("`Config` (first argument) or `Client` (return type)")));
        assert!(messages.iter().any(|m| m.contains("newtype")));
    }

    #[test]
    fn test_exemptions_and_entry_points() {
        let mut project = project_with(&[
            ("game/bevy_systems/tick.rs", "pub fn tick() {}"),
            ("bin/tool.rs", "pub fn helper() {}"),
            ("main.rs", "pub fn run() {}"),
            (
                "run.rs",
                "pub fn run() {} fn private() {} #[cfg(test)] pub fn t() {}",
            ),
        ]);
        project.entry_points.push(EntryPoint {
            file: PathBuf::from("src/run.rs"),
            function: "run".to_string(),
            reason: "task".to_string(),
        });
        assert!(check(&NoFreeFunctions, &project).is_empty());
    }
}
