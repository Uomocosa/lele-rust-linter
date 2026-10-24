use crate::common;
use crate::rules;
use crate::FnBody;
use crate::ImplBlock;
use crate::ImplMethod;
use crate::ItemMarker;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &rules::atomic_delegates::AtomicDelegates,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for (rel_path, file) in &project.parsed_files {
        let Some(file_stem) = rel_path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let Some(primary) = common::SourceFile(file).primary_type_name(file_stem) else {
            continue;
        };

        for item in &file.items {
            let syn::Item::Impl(impl_block) = item else {
                continue;
            };

            if ImplBlock(impl_block).self_type_last().as_deref() != Some(primary.as_str()) {
                continue;
            }

            if ImplBlock(impl_block).is_default_impl() {
                continue;
            }

            if !rules::atomic_delegates::AtomicDelegates::has_any_method(impl_block) {
                continue;
            }

            if impl_block
                .attrs
                .iter()
                .any(|attr| ItemMarker(attr).is_cfg_test())
            {
                continue;
            }

            let is_trait_impl = impl_block.trait_.is_some();

            let methods_over_three: Vec<&syn::Ident> = impl_block
                .items
                .iter()
                .filter_map(|item| {
                    if let syn::ImplItem::Fn(method) = item {
                        if FnBody(&method.block).is_short() {
                            None
                        } else {
                            Some(&method.sig.ident)
                        }
                    } else {
                        None
                    }
                })
                .collect();

            if !methods_over_three.is_empty() && !is_trait_impl {
                let names =
                    rules::atomic_delegates::AtomicDelegates::names_str(&methods_over_three);
                violations.push(RuleViolation::from_file(project.src_dir.join(rel_path), rules::atomic_delegates::AtomicDelegates::CODE, format!(
                        "method(s) `{names}` have >3 statements — extract each into `<type>_<method>.rs`"
                    )));
                continue;
            }

            let one_liners: Vec<&syn::ImplItemFn> = impl_block
                .items
                .iter()
                .filter_map(|item| {
                    if let syn::ImplItem::Fn(method) = item {
                        if method.block.stmts.len() == 1 {
                            Some(method)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

            if !one_liners.is_empty() {
                if impl_block
                    .attrs
                    .iter()
                    .any(|attr| ItemMarker(attr).is_rustfmt_skip())
                    || impl_block
                        .attrs
                        .iter()
                        .any(|attr| ItemMarker(attr).is_atomic_delegates())
                {
                    for method in &one_liners {
                        if FnBody(&method.block).is_delegate_call()
                            && !rules::atomic_delegates::AtomicDelegates::is_one_line_body(method)
                        {
                            violations.push(RuleViolation::from_file(project.src_dir.join(rel_path), rules::atomic_delegates::AtomicDelegates::CODE, format!(
                                    "one-liner method `{}` body must be on one line, e.g. `{{ module::func(self) }}`",
                                    method.sig.ident
                                )));
                        }
                    }
                } else if !ImplBlock(impl_block).is_default_impl() {
                    let plain: Vec<&&syn::ImplItemFn> = one_liners
                        .iter()
                        .filter(|m| !ImplMethod(m).is_real_constructor())
                        .collect();
                    if plain.is_empty() {
                        continue;
                    }
                    let names = plain
                        .iter()
                        .map(|m| m.sig.ident.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    let ctors: Vec<String> = one_liners
                        .iter()
                        .filter(|m| ImplMethod(m).is_real_constructor())
                        .map(|m| m.sig.ident.to_string())
                        .collect();
                    let message = if ctors.is_empty() || is_trait_impl {
                        format!(
                            "one-liner method(s) `{names}` require `#[rustfmt::skip]` on the impl block"
                        )
                    } else {
                        format!(
                            "one-liner method(s) `{names}` share a block with constructor(s) `{}`; move the constructor(s) to their own skip-free block, then add `#[rustfmt::skip]` here",
                            ctors.join(", ")
                        )
                    };
                    violations.push(RuleViolation::from_file(
                        project.src_dir.join(rel_path),
                        rules::atomic_delegates::AtomicDelegates::CODE,
                        message,
                    ));
                }
            }
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::check;
    use crate::rules;
    use crate::Project;

    #[test]
    fn test_usage() {
        let rule = rules::atomic_delegates::AtomicDelegates;
        let project = Project::default();
        assert!(check(&rule, &project).is_empty());
    }

    #[test]
    fn test_usage_real_one_liner_requires_skip() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str(
                "pub struct Foo;\nimpl Foo { fn name(&self) -> &'static str { Self::NAME } }\n",
            )
            .unwrap(),
        );
        let violations = check(&rules::atomic_delegates::AtomicDelegates, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("#[rustfmt::skip]"));
    }

    #[test]
    fn test_usage_ctor_one_liner_is_exempt() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str("pub struct Foo;\nimpl Foo { pub fn new() -> Self { Self } }\n")
                .unwrap(),
        );
        assert!(check(&rules::atomic_delegates::AtomicDelegates, &project).is_empty());
    }

    #[test]
    fn test_usage_ctor_mixed_demands_move_out() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str(
                "pub struct Foo;\nimpl Foo { pub fn new() -> Self { Self } fn name(&self) -> &'static str { Self::NAME } }\n",
            )
            .unwrap(),
        );
        let violations = check(&rules::atomic_delegates::AtomicDelegates, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("move the constructor"));
    }

    #[test]
    fn test_usage_fn_shell_block_requires_skip() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str(
                "pub struct Foo;\nimpl Foo { #[atomic_delegate(Foo)] pub fn run(&self) {} fn name(&self) -> &'static str { Self::NAME } }\n",
            )
            .unwrap(),
        );
        let violations = check(&rules::atomic_delegates::AtomicDelegates, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("#[rustfmt::skip]"));
    }

    #[test]
    fn test_usage_multiline_delegate_call_demands_one_line() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str(
                "pub struct Foo;\n#[rustfmt::skip]\nimpl Foo {\n    fn run(&self) {\n        foo_run::run(self)\n    }\n}\n",
            )
            .unwrap(),
        );
        let violations = check(&rules::atomic_delegates::AtomicDelegates, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("must be on one line"));
    }

    #[test]
    fn test_usage_single_line_delegate_call_is_clean() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str(
                "pub struct Foo;\n#[rustfmt::skip]\nimpl Foo {\n    fn run(&self) { foo_run::run(self) }\n}\n",
            )
            .unwrap(),
        );
        assert!(check(&rules::atomic_delegates::AtomicDelegates, &project).is_empty());
    }
}
