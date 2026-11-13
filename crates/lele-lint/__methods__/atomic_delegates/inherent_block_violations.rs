use std::path::Path;

use crate::rules;
use crate::FnBody;
use crate::ImplMethod;
use crate::ItemMarker;
use crate::PlacementKind;
use crate::Project;
use crate::RuleViolation;

pub fn inherent_block_violations(
    project: &Project,
    rel_path: &Path,
    impl_block: &syn::ItemImpl,
) -> Vec<RuleViolation> {
    let has_skip = impl_block
        .attrs
        .iter()
        .any(|attr| ItemMarker(attr).is_rustfmt_skip());
    let file = project.absolute_src_path(rel_path);
    let code = rules::atomic_delegates::AtomicDelegates::CODE;
    let root = project.methods_root_name();
    let listed = rules::atomic_delegates::AtomicDelegates::is_no_delegates_crate(project);
    let mut violations = Vec::new();
    let mut plain_one_liners: Vec<String> = Vec::new();
    let mut plain_ctors: Vec<String> = Vec::new();
    let mut multi_line: Vec<String> = Vec::new();
    for item in &impl_block.items {
        let syn::ImplItem::Fn(method) = item else {
            continue;
        };
        if !has_skip && method.block.stmts.is_empty() {
            continue;
        }
        let name = method.sig.ident.to_string();
        let block = FnBody(&method.block);
        let rustfmt_one_line = method
            .block
            .stmts
            .first()
            .is_some_and(|stmt| project.rustfmt_one_liner(rel_path, stmt));
        let is_ctor = ImplMethod(method).is_real_constructor();
        let kind = block.placement_kind(rustfmt_one_line);
        if !has_skip && is_ctor && kind != PlacementKind::OneLiner {
            continue;
        }
        match kind {
            PlacementKind::OneLiner if has_skip => {
                if block.is_delegate_call()
                    && !rules::atomic_delegates::AtomicDelegates::is_one_line_body(method)
                {
                    violations.push(RuleViolation::from_file(file.clone(), code, format!(
                        "one-liner method `{name}` spans several lines — put the body on one line, e.g. `{{ module::func(self) }}`"
                    )));
                }
            }
            PlacementKind::OneLiner if is_ctor => {
                plain_ctors.push(name);
            }
            PlacementKind::OneLiner => plain_one_liners.push(name),
            PlacementKind::SimpleBlock | PlacementKind::FewStatements if has_skip => {
                violations.push(RuleViolation::from_file(file.clone(), code, format!(
                    "method `{name}` in a `#[rustfmt::skip]` block is not a one-liner — move it to a plain `impl` block so rustfmt formats it"
                )));
            }
            PlacementKind::SimpleBlock | PlacementKind::FewStatements => multi_line.push(name),
            PlacementKind::Delegate if listed && has_skip => {
                violations.push(RuleViolation::from_file(file.clone(), code, format!(
                    "method `{name}` in a `#[rustfmt::skip]` block is not a one-liner — move it to a plain `impl` block so rustfmt formats it"
                )));
            }
            PlacementKind::Delegate if listed => multi_line.push(name),
            PlacementKind::Delegate => {
                let message = if block.is_control_statement() {
                    format!(
                        "method `{name}` is a `match`/`if`/loop whose branches do more than one thing — give each branch its own method, or move the body to `{root}/<type>/<method>.rs` behind an empty shell in the type's `#[atomic_delegates]` block"
                    )
                } else {
                    format!(
                        "method `{name}` is one statement that rustfmt prints on several lines — move its body to `{root}/<type>/<method>.rs` behind an empty shell in the type's `#[atomic_delegates]` block"
                    )
                };
                violations.push(RuleViolation::from_file(file.clone(), code, message));
            }
        }
    }
    if has_skip || plain_one_liners.is_empty() {
        return violations;
    }
    let names = plain_one_liners.join(", ");
    let message = if !multi_line.is_empty() {
        format!(
            "one-liner method(s) `{names}` share a plain block with multi-line methods — move them to a `#[rustfmt::skip]` impl block"
        )
    } else if !plain_ctors.is_empty() {
        format!(
            "one-liner method(s) `{names}` share a block with constructor(s) `{}` — move the constructor(s) to their own skip-free block, then add `#[rustfmt::skip]` here",
            plain_ctors.join(", ")
        )
    } else {
        format!(
            "one-liner method(s) `{names}` in a block without `#[rustfmt::skip]` — add it to the impl block"
        )
    };
    violations.push(RuleViolation::from_file(file, code, message));
    violations
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    use crate::rules;
    use crate::Project;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let source = "pub struct Foo;\nimpl Foo { fn run(&self) -> u8 { let a = 1; a } }\nimpl Foo { fn items(&self) -> Vec<u32> { self.items.iter().map(|item| item.value * 2 + item.offset).filter(|v| *v > 10).map(|v| v * 3).collect::<Vec<u32>>() } }\nimpl Foo { fn empty(&self) {} }\n";
        std::fs::write(dir.path().join("foo.rs"), source).unwrap();
        let mut project = Project {
            root: PathBuf::from("."),
            src_dir: dir.path().to_path_buf(),
            ..Project::default()
        };
        let file: syn::File = syn::parse_str(source).unwrap();
        project
            .parsed_files
            .insert(PathBuf::from("foo.rs"), file.clone());
        project.fill_rustfmt_answers().unwrap();
        let syn::Item::Impl(two_statements) = &file.items[1] else {
            panic!("expected an impl block");
        };
        let syn::Item::Impl(long_chain) = &file.items[2] else {
            panic!("expected an impl block");
        };
        let syn::Item::Impl(empty) = &file.items[3] else {
            panic!("expected an impl block");
        };
        let check = |block| {
            rules::atomic_delegates::AtomicDelegates::inherent_block_violations(
                &project,
                Path::new("foo.rs"),
                block,
            )
        };
        assert!(check(two_statements).is_empty());
        assert_eq!(check(long_chain).len(), 1);
        assert!(check(empty).is_empty());
    }

    #[test]
    fn test_usage_listed_crate_accepts_multi_line_statement_in_plain_block() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("crates/listed")).unwrap();
        std::fs::write(
            dir.path().join("crates/listed/Cargo.toml"),
            "[package]\nname = \"listed\"\nedition = \"2021\"\n",
        )
        .unwrap();
        let source = "pub struct Foo;\nimpl Foo { fn items(&self) -> Vec<u32> { self.items.iter().map(|item| item.value * 2 + item.offset).filter(|v| *v > 10).map(|v| v * 3).collect::<Vec<u32>>() } }\nimpl Foo { fn items(&self) -> Vec<u32> { self.items.iter().map(|item| item.value * 2 + item.offset).filter(|v| *v > 10).map(|v| v * 3).collect::<Vec<u32>>() } }\n#[rustfmt::skip]\nimpl Foo { fn items(&self) -> Vec<u32> { self.items.iter().map(|item| item.value * 2 + item.offset).filter(|v| *v > 10).map(|v| v * 3).collect::<Vec<u32>>() } }\n";
        std::fs::write(dir.path().join("crates/listed/foo.rs"), source).unwrap();
        let file: syn::File = syn::parse_str(source).unwrap();
        let mut project = Project {
            root: dir.path().join("crates/listed"),
            config_dir: dir.path().to_path_buf(),
            src_dir: dir.path().join("crates/listed"),
            no_delegates: vec![crate::NoDelegatesEntry {
                crate_path: PathBuf::from("crates/listed"),
                reason: "cannot depend on the macro crate".to_string(),
            }],
            ..Project::default()
        };
        project
            .parsed_files
            .insert(PathBuf::from("foo.rs"), file.clone());
        project.fill_rustfmt_answers().unwrap();
        let syn::Item::Impl(plain) = &file.items[1] else {
            panic!("expected an impl block");
        };
        let syn::Item::Impl(skipped) = &file.items[3] else {
            panic!("expected an impl block");
        };
        let check = |block| {
            rules::atomic_delegates::AtomicDelegates::inherent_block_violations(
                &project,
                Path::new("foo.rs"),
                block,
            )
        };
        assert!(check(plain).is_empty());
        assert_eq!(check(skipped).len(), 1);
    }
}
