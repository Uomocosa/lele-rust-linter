use std::collections::HashSet;

use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &rules::no_trivial_accessors::NoTrivialAccessors,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for source in project.content_sources() {
        let rel_path = source.relative_path;
        let file = source.file;
        if file
            .items
            .iter()
            .any(|item| matches!(item, syn::Item::Trait(_)))
        {
            continue;
        }

        let vis_fields = rules::no_trivial_accessors::NoTrivialAccessors::collect_vis_fields(file);
        let pub_fields: HashSet<String> = vis_fields.keys().cloned().collect();

        for item in &file.items {
            if let syn::Item::Impl(impl_block) = item {
                if impl_block.trait_.is_some() {
                    continue;
                }
                for impl_item in &impl_block.items {
                    if let syn::ImplItem::Fn(method) = impl_item {
                        if rules::no_trivial_accessors::NoTrivialAccessors::is_builder(method) {
                            continue;
                        }
                        if let Some(field) =
                            rules::no_trivial_accessors::NoTrivialAccessors::is_trivial_accessor(
                                method,
                                &pub_fields,
                            )
                        {
                            let vis = vis_fields.get(&field).cloned().unwrap_or_default();
                            violations.push(RuleViolation::from_file(project.absolute_path(source.origin, rel_path), rules::no_trivial_accessors::NoTrivialAccessors::CODE, format!(
                                    "trivial accessor `{}` reads {vis} field `{field}`, make field public/pub(crate) and access directly",
                                    method.sig.ident
                                )));
                        } else if let Some(field) =
                            rules::no_trivial_accessors::NoTrivialAccessors::is_trivial_setter(
                                method,
                                &pub_fields,
                            )
                        {
                            let vis = vis_fields.get(&field).cloned().unwrap_or_default();
                            violations.push(RuleViolation::from_file(project.absolute_path(source.origin, rel_path), rules::no_trivial_accessors::NoTrivialAccessors::CODE, format!(
                                    "trivial setter `{}` assigns {vis} field `{field}`, make field public/pub(crate) and assign directly",
                                    method.sig.ident
                                )));
                        }
                    }
                }
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
    use std::collections::HashSet;

    #[test]
    fn test_usage() {
        let rule = rules::no_trivial_accessors::NoTrivialAccessors;
        let project = Project::default();
        assert!(check(&rule, &project).is_empty());
    }

    #[test]
    fn test_usage_flags_trivial_getter() {
        let method: syn::ImplItemFn =
            syn::parse_str("fn value(&self) -> u32 { self.value }").unwrap();
        let mut fields = HashSet::new();
        fields.insert("value".into());
        assert!(
            rules::no_trivial_accessors::NoTrivialAccessors::is_trivial_accessor(&method, &fields)
                .is_some()
        );
    }

    #[test]
    fn test_usage_flags_wrapped_getter() {
        let mut fields = HashSet::new();
        fields.insert("items".into());
        for source in [
            "fn items(&self) -> Result<Vec<u32>, E> { Ok(self.items.clone()) }",
            "fn items(&self) -> Option<&[u32]> { Some(self.items.as_slice()) }",
            "fn items(&self) -> Vec<u32> { self.items.to_vec() }",
        ] {
            let method: syn::ImplItemFn = syn::parse_str(source).unwrap();
            assert!(
                rules::no_trivial_accessors::NoTrivialAccessors::is_trivial_accessor(
                    &method, &fields
                )
                .is_some(),
                "{source}"
            );
        }
        let computed: syn::ImplItemFn =
            syn::parse_str("fn count(&self) -> Result<usize, E> { Ok(self.items.len()) }").unwrap();
        assert!(
            rules::no_trivial_accessors::NoTrivialAccessors::is_trivial_accessor(
                &computed, &fields
            )
            .is_none()
        );
    }

    #[test]
    fn test_usage_allows_computation() {
        let method: syn::ImplItemFn =
            syn::parse_str("fn doubled(&self) -> u32 { self.value * 2 }").unwrap();
        let mut fields = HashSet::new();
        fields.insert("value".into());
        assert!(
            rules::no_trivial_accessors::NoTrivialAccessors::is_trivial_accessor(&method, &fields)
                .is_none()
        );
    }

    #[test]
    fn test_usage_allows_non_pub() {
        let method: syn::ImplItemFn =
            syn::parse_str("fn inner(&self) -> u32 { self.secret }").unwrap();
        let fields: HashSet<String> = HashSet::new();
        assert!(
            rules::no_trivial_accessors::NoTrivialAccessors::is_trivial_accessor(&method, &fields)
                .is_none()
        );
    }

    #[test]
    fn test_usage_flags_getter_with_log() {
        let method: syn::ImplItemFn =
            syn::parse_str("fn value(&self) -> u32 { tracing::info!(\"get\"); self.value }")
                .unwrap();
        let mut fields = HashSet::new();
        fields.insert("value".into());
        assert!(
            rules::no_trivial_accessors::NoTrivialAccessors::is_trivial_accessor(&method, &fields)
                .is_some()
        );
    }

    #[test]
    fn test_usage_flags_setter() {
        let method: syn::ImplItemFn =
            syn::parse_str("fn set_value(&mut self, v: u32) { self.value = v; }").unwrap();
        let mut fields = HashSet::new();
        fields.insert("value".into());
        assert!(
            rules::no_trivial_accessors::NoTrivialAccessors::is_trivial_setter(&method, &fields)
                .is_some()
        );
    }

    #[test]
    fn test_usage_flags_setter_with_log() {
        let method: syn::ImplItemFn = syn::parse_str(
            "fn set_value(&mut self, v: u32) { self.value = v; println!(\"set\"); }",
        )
        .unwrap();
        let mut fields = HashSet::new();
        fields.insert("value".into());
        assert!(
            rules::no_trivial_accessors::NoTrivialAccessors::is_trivial_setter(&method, &fields)
                .is_some()
        );
    }

    #[test]
    fn test_usage_allows_builder() {
        let method: syn::ImplItemFn =
            syn::parse_str("fn with_tag(self, tag: u64) -> Self { Self { tag, ..self } }").unwrap();
        assert!(rules::no_trivial_accessors::NoTrivialAccessors::is_builder(
            &method
        ));
    }
}

// no test_usage necessary
