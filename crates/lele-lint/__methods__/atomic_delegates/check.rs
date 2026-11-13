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
    let mut violations = rules::atomic_delegates::AtomicDelegates::no_delegates_violations(project);
    let listed = rules::atomic_delegates::AtomicDelegates::is_no_delegates_crate(project);

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
            let is_atomic_block = impl_block
                .attrs
                .iter()
                .any(|attr| ItemMarker(attr).is_atomic_delegates());

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

            if !methods_over_three.is_empty() && !is_trait_impl && !listed {
                let names =
                    rules::atomic_delegates::AtomicDelegates::names_str(&methods_over_three);
                violations.push(RuleViolation::from_file(project.absolute_src_path(rel_path), rules::atomic_delegates::AtomicDelegates::CODE, format!(
                        "method(s) `{names}` have >3 statements — move each body to `{}/<type>/<method>.rs` behind an empty shell in the type's `#[atomic_delegates]` block",
                        project.methods_root_name()
                    )));
                continue;
            }

            if !is_trait_impl && !is_atomic_block {
                violations.extend(
                    rules::atomic_delegates::AtomicDelegates::inherent_block_violations(
                        project, rel_path, impl_block,
                    ),
                );
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
                let has_skip = impl_block
                    .attrs
                    .iter()
                    .any(|attr| ItemMarker(attr).is_rustfmt_skip());
                if has_skip
                    || impl_block
                        .attrs
                        .iter()
                        .any(|attr| ItemMarker(attr).is_atomic_delegates())
                {
                    for method in &one_liners {
                        let is_multi_line =
                            !rules::atomic_delegates::AtomicDelegates::is_one_line_body(method);
                        if FnBody(&method.block).is_delegate_call() && is_multi_line {
                            violations.push(RuleViolation::from_file(project.absolute_src_path(rel_path), rules::atomic_delegates::AtomicDelegates::CODE, format!(
                                    "one-liner method `{}` spans several lines — put the body on one line, e.g. `{{ module::func(self) }}`",
                                    method.sig.ident
                                )));
                        } else if has_skip && !is_trait_impl && is_multi_line {
                            violations.push(RuleViolation::from_file(project.absolute_src_path(rel_path), rules::atomic_delegates::AtomicDelegates::CODE, format!(
                                    "method `{}` is one statement spanning several lines — move its body to `{}/<type>/<method>.rs` behind an empty shell in the type's `#[atomic_delegates]` block",
                                    method.sig.ident,
                                    project.methods_root_name()
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
                            "one-liner method(s) `{names}` in a block without `#[rustfmt::skip]` — add it to the impl block"
                        )
                    } else {
                        format!(
                            "one-liner method(s) `{names}` share a block with constructor(s) `{}` — move the constructor(s) to their own skip-free block, then add `#[rustfmt::skip]` here",
                            ctors.join(", ")
                        )
                    };
                    violations.push(RuleViolation::from_file(
                        project.absolute_src_path(rel_path),
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
    use crate::RuleViolation;

    fn run_check(src: &str) -> Vec<RuleViolation> {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("foo.rs"), src).unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            src_dir: dir.path().to_path_buf(),
            ..Project::default()
        };
        project
            .parsed_files
            .insert(PathBuf::from("foo.rs"), syn::parse_str(src).unwrap());
        project.fill_rustfmt_answers().unwrap();
        check(&rules::atomic_delegates::AtomicDelegates, &project)
    }

    #[test]
    fn test_usage() {
        let project = Project::default();
        assert!(check(&rules::atomic_delegates::AtomicDelegates, &project).is_empty());
    }

    #[test]
    fn test_usage_real_one_liner_requires_skip() {
        let violations = run_check(
            "pub struct Foo;\nimpl Foo { fn name(&self) -> &'static str { Self::NAME } }\n",
        );
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("#[rustfmt::skip]"));
    }

    #[test]
    fn test_usage_ctor_one_liner_is_exempt() {
        assert!(
            run_check("pub struct Foo;\nimpl Foo { pub fn new() -> Self { Self } }\n").is_empty()
        );
    }

    #[test]
    fn test_usage_ctor_mixed_demands_move_out() {
        let violations = run_check(
            "pub struct Foo;\nimpl Foo { pub fn new() -> Self { Self } fn name(&self) -> &'static str { Self::NAME } }\n",
        );
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("move the constructor"));
    }

    #[test]
    fn test_usage_fn_shell_block_requires_skip() {
        let violations = run_check(
            "pub struct Foo;\nimpl Foo { #[atomic_delegate(Foo)] pub fn run(&self) {} fn name(&self) -> &'static str { Self::NAME } }\n",
        );
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("#[rustfmt::skip]"));
    }

    #[test]
    fn test_usage_multiline_delegate_call_demands_one_line() {
        let violations = run_check(
            "pub struct Foo;\n#[rustfmt::skip]\nimpl Foo {\n    fn run(&self) {\n        foo_run::run(self)\n    }\n}\n",
        );
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("put the body on one line"));
    }

    #[test]
    fn test_usage_one_liner_braces_on_own_lines_in_skip_block_is_clean() {
        let violations = run_check(
            "pub struct Foo;\n#[rustfmt::skip]\nimpl Foo {\n    fn run(&self) -> u8 {\n        self.0\n    }\n    fn name(&self) -> u8 { self.0 }\n}\n",
        );
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_long_chain_on_one_line_in_skip_block_is_m1() {
        let violations = run_check(
            "pub struct Foo;\n#[rustfmt::skip]\nimpl Foo {\n    fn run(&self) -> Vec<u32> { self.items.iter().map(|item| item.value * 2 + item.offset).filter(|v| *v > 10).map(|v| v * 3).collect::<Vec<u32>>() }\n}\n",
        );
        assert_eq!(violations.len(), 1);
        assert!(violations[0]
            .message
            .contains("rustfmt prints on several lines"));
    }

    #[test]
    fn test_usage_simple_match_in_plain_block_is_clean() {
        let violations = run_check(
            "pub struct Foo;\nimpl Foo {\n    fn run(&self) -> u8 {\n        match self.0 { 0 => 1, _ => 2 }\n    }\n}\n",
        );
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_simple_match_in_skip_block_is_m2() {
        let violations = run_check(
            "pub struct Foo;\n#[rustfmt::skip]\nimpl Foo {\n    fn run(&self) -> u8 {\n        match self.0 {\n            0 => 1,\n            _ => 2,\n        }\n    }\n}\n",
        );
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("is not a one-liner"));
    }

    #[test]
    fn test_usage_match_arm_with_two_statements_is_m1() {
        let violations = run_check(
            "pub struct Foo;\nimpl Foo {\n    fn run(&self) -> u8 {\n        match self.0 { 0 => { let a = 1; a } _ => 2 }\n    }\n}\n",
        );
        assert_eq!(violations.len(), 1);
        assert!(violations[0]
            .message
            .contains("branches do more than one thing"));
    }

    #[test]
    fn test_usage_match_arm_with_nested_if_is_m1() {
        let violations = run_check(
            "pub struct Foo;\nimpl Foo {\n    fn run(&self) -> u8 {\n        match self.0 { 0 => if self.1 { 1 } else { 2 }, _ => 3 }\n    }\n}\n",
        );
        assert_eq!(violations.len(), 1);
        assert!(violations[0]
            .message
            .contains("branches do more than one thing"));
    }

    #[test]
    fn test_usage_simple_for_loop_in_plain_block_is_clean() {
        let violations = run_check(
            "pub struct Foo;\nimpl Foo {\n    fn run(&self) {\n        for item in self.items.iter() { println!(\"{}\", item) }\n    }\n}\n",
        );
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_for_loop_with_if_in_body_is_m1() {
        let violations = run_check(
            "pub struct Foo;\nimpl Foo {\n    fn run(&self) {\n        for item in self.items.iter() { if item.ok { println!(\"{}\", item) } }\n    }\n}\n",
        );
        assert_eq!(violations.len(), 1);
        assert!(violations[0]
            .message
            .contains("branches do more than one thing"));
    }

    #[test]
    fn test_usage_three_statements_in_skip_block_is_m2() {
        let violations = run_check(
            "pub struct Foo;\n#[rustfmt::skip]\nimpl Foo {\n    fn run(&self) -> u8 {\n        let a = 1;\n        let b = 2;\n        a + b + self.0\n    }\n}\n",
        );
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("is not a one-liner"));
    }

    #[test]
    fn test_usage_three_statements_in_plain_block_is_clean() {
        let violations = run_check(
            "pub struct Foo;\nimpl Foo {\n    fn run(&self) -> u8 {\n        let a = 1;\n        let b = 2;\n        a + b + self.0\n    }\n}\n",
        );
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_one_liner_with_multi_line_method_in_plain_block_is_m3() {
        let violations = run_check(
            "pub struct Foo;\nimpl Foo {\n    fn name(&self) -> u8 { self.0 }\n    fn sum(&self) -> u8 {\n        let a = 1;\n        a + self.0\n    }\n}\n",
        );
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("share a plain block"));
    }

    #[test]
    fn test_usage_trait_impl_simple_match_in_skip_block_is_clean() {
        let violations = run_check(
            "pub struct Foo;\n#[rustfmt::skip]\nimpl Rule for Foo {\n    fn run(&self) -> u8 {\n        match self.0 {\n            0 => 1,\n            _ => 2,\n        }\n    }\n}\n",
        );
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_single_line_delegate_call_is_clean() {
        assert!(run_check(
            "pub struct Foo;\n#[rustfmt::skip]\nimpl Foo {\n    fn run(&self) { foo_run::run(self) }\n}\n",
        )
        .is_empty());
    }

    #[test]
    fn test_usage_multi_line_tracing_call_in_skip_block_is_m1() {
        let violations = run_check(
            "pub struct Foo;\n#[rustfmt::skip]\nimpl Foo {\n    fn log(&self) {\n        tracing::info!(\n            target: \"x\",\n            value,\n            \"hello\"\n        );\n    }\n}\n",
        );
        assert_eq!(violations.len(), 1);
        assert!(violations[0]
            .message
            .contains("rustfmt prints on several lines"));
    }

    #[test]
    fn test_usage_over_width_one_line_macro_in_skip_block_is_m1() {
        let violations = run_check(
            "pub struct Foo;\n#[rustfmt::skip]\nimpl Foo {\n    fn log(&self) { tracing::info!(target: \"discovery\", missed_sends, \"a long message that keeps going for quite a while so the whole statement exceeds the width\"); }\n}\n",
        );
        assert_eq!(violations.len(), 1);
        assert!(violations[0]
            .message
            .contains("rustfmt prints on several lines"));
    }
}
