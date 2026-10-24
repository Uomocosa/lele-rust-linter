use std::collections::HashMap;

use quote::ToTokens;
use syn::spanned::Spanned;

use crate::rules;
use crate::ImplBlock;
use crate::ImplItemGroup;
use crate::ItemMarker;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &rules::impl_item_order::ImplItemOrder,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for (rel_path, file) in &project.parsed_files {
        let mut furthest_by_type: HashMap<String, (ImplItemGroup, String)> = HashMap::new();
        let mut place = |type_name: &str, group: ImplItemGroup, label: String, line: usize| {
            match furthest_by_type.get(type_name) {
                Some((furthest_group, furthest_label)) if group < *furthest_group => {
                    violations.push(RuleViolation::from_site(
                        project.src_dir.join(rel_path),
                        line,
                        0,
                        rules::impl_item_order::ImplItemOrder::CODE,
                        format!(
                            "`{label}` ({group}) comes after `{furthest_label}` ({furthest_group}) — order a type's impls as: `impl Default`, constructors, trait impls, `pub` fns, `pub(crate)` fns"
                        ),
                    ));
                }
                _ => {
                    furthest_by_type.insert(type_name.to_string(), (group, label));
                }
            }
        };
        for item in &file.items {
            let syn::Item::Impl(block) = item else {
                continue;
            };
            if block
                .attrs
                .iter()
                .any(|attr| ItemMarker(attr).is_cfg_test())
            {
                continue;
            }
            let Some(type_name) = ImplBlock(block).self_type_last() else {
                continue;
            };
            if let Some((_, path, _)) = &block.trait_ {
                let trait_name = path
                    .segments
                    .last()
                    .map(|segment| segment.ident.to_string())
                    .unwrap_or_default();
                let group = match trait_name.as_str() {
                    "Default" => ImplItemGroup::DefaultImpl,
                    "From" | "TryFrom" | "FromStr" => ImplItemGroup::Constructor,
                    _ => ImplItemGroup::TraitImpl,
                };
                place(
                    &type_name,
                    group,
                    format!("impl {trait_name}"),
                    block.impl_token.span.start().line,
                );
                continue;
            }
            for impl_item in &block.items {
                let syn::ImplItem::Fn(method) = impl_item else {
                    continue;
                };
                let returns_self = method
                    .sig
                    .output
                    .to_token_stream()
                    .to_string()
                    .split_whitespace()
                    .any(|token| token == "Self");
                let group = if method.sig.receiver().is_none() && returns_self {
                    ImplItemGroup::Constructor
                } else if matches!(method.vis, syn::Visibility::Public(_)) {
                    ImplItemGroup::PubFn
                } else {
                    ImplItemGroup::CrateFn
                };
                place(
                    &type_name,
                    group,
                    format!("fn {}", method.sig.ident),
                    method.sig.fn_token.span().start().line,
                );
            }
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::check;
    use crate::rules::impl_item_order::ImplItemOrder;
    use crate::Project;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("round.rs"),
            syn::parse_str(
                "impl Default for Round {} impl Round { pub fn new() -> Self {} } impl Display for Round {} impl Round { pub fn a(&self) {} pub(crate) fn b() {} pub fn c(&self) {} } impl From<u8> for Round {} impl Other { pub(crate) fn d() {} pub fn e() {} }",
            )
            .unwrap(),
        );
        let messages: Vec<String> = check(&ImplItemOrder, &project)
            .into_iter()
            .map(|v| v.message)
            .collect();
        assert_eq!(messages.len(), 3);
        assert!(messages
            .iter()
            .any(|m| m.starts_with("`fn c` (`pub` fn) comes after `fn b`")));
        assert!(messages
            .iter()
            .any(|m| m.starts_with("`impl From` (constructor) comes after `fn b`")));
        assert!(messages
            .iter()
            .any(|m| m.starts_with("`fn e` (`pub` fn) comes after `fn d`")));
    }
}
