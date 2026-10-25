use crate::rules;
use crate::ImplBlock;
use crate::ItemMarker;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &rules::constructor_no_skip::ConstructorNoSkip,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for (rel_path, file) in &project.parsed_files {
        for item in &file.items {
            if let syn::Item::Impl(impl_block) = item {
                if impl_block.trait_.is_some() && !ImplBlock(impl_block).is_default_impl() {
                    continue;
                }

                if !impl_block
                    .attrs
                    .iter()
                    .any(|attr| ItemMarker(attr).is_rustfmt_skip())
                {
                    continue;
                }

                if rules::constructor_no_skip::ConstructorNoSkip::is_atomic_delegates(impl_block) {
                    continue;
                }

                let type_name = rules::constructor_no_skip::ConstructorNoSkip::type_name_string(
                    &impl_block.self_ty,
                );
                let blurb = if ImplBlock(impl_block).is_default_impl() {
                    format!("impl Default for {type_name}")
                } else if rules::constructor_no_skip::ConstructorNoSkip::has_any_real_constructor(
                    impl_block,
                ) {
                    format!("constructor impl for {type_name}")
                } else {
                    continue;
                };

                violations.push(RuleViolation::from_file(
                    project.src_dir.join(rel_path),
                    rules::constructor_no_skip::ConstructorNoSkip::CODE,
                    format!("{blurb} must not have `#[rustfmt::skip]` — move it to an impl block without the attribute",),
                ));
            }
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::check;
    use crate::rules;
    use crate::Project;
    use syn::ItemImpl;

    #[test]
    fn test_usage() {
        let rule = rules::constructor_no_skip::ConstructorNoSkip;
        let project = Project::default();
        assert!(check(&rule, &project).is_empty());
    }

    #[test]
    fn test_usage_detects_default_skip() {
        let code =
            "#[rustfmt::skip] impl Default for Bar { fn default() -> Self { Bar { x: 1 } } }";
        let parsed: ItemImpl = syn::parse_str(code).unwrap();
        assert!(!rules::constructor_no_skip::ConstructorNoSkip::is_atomic_delegates(&parsed));
    }

    #[test]
    fn test_usage_constructor_detection() {
        let parsed: ItemImpl = syn::parse_str(
            "impl Foo { fn production() -> Self { let a = 1; let b = 2; let c = 3; Foo { x: a + b + c } } }",
        )
        .unwrap();
        assert!(rules::constructor_no_skip::ConstructorNoSkip::has_any_real_constructor(&parsed));
        assert!(!rules::constructor_no_skip::ConstructorNoSkip::is_atomic_delegates(&parsed));
    }

    #[test]
    fn test_usage_atomic_delegate_has_no_real_constructor() {
        let parsed: ItemImpl =
            syn::parse_str("impl Foo { pub fn new() -> Self { config_new::new() } }").unwrap();
        assert!(!rules::constructor_no_skip::ConstructorNoSkip::has_any_real_constructor(&parsed));
        assert!(rules::constructor_no_skip::ConstructorNoSkip::is_atomic_delegates(&parsed));
    }

    #[test]
    fn test_usage_struct_literal_is_real_constructor() {
        let parsed: ItemImpl =
            syn::parse_str("impl Foo { pub fn new(x: i32) -> Self { Self { x } } }").unwrap();
        assert!(rules::constructor_no_skip::ConstructorNoSkip::has_any_real_constructor(&parsed));
        assert!(!rules::constructor_no_skip::ConstructorNoSkip::is_atomic_delegates(&parsed));
    }

    #[test]
    fn test_usage_self_constructor_call_is_real_constructor() {
        let parsed: ItemImpl =
            syn::parse_str("impl Foo { pub fn coop() -> Self { Self::new() } }").unwrap();
        assert!(rules::constructor_no_skip::ConstructorNoSkip::has_any_real_constructor(&parsed));
        assert!(!rules::constructor_no_skip::ConstructorNoSkip::is_atomic_delegates(&parsed));
    }
}
