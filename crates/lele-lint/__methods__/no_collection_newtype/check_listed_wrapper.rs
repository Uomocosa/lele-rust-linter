use crate::rules;
use crate::RuleViolation;

pub fn check_listed_wrapper(
    struct_def: &syn::ItemStruct,
    file_path: &std::path::Path,
    items: &[syn::Item],
) -> Vec<RuleViolation> {
    let wrapper =
        rules::no_collection_newtype::NoCollectionNewtype::check_struct(struct_def, file_path);
    if wrapper.is_empty() {
        return Vec::new();
    }
    let name = struct_def.ident.to_string();
    let prefix = format!("`{name}` is a listed collection wrapper but");
    let problem = if rules::no_collection_newtype::NoCollectionNewtype::has_public_field(struct_def)
    {
        format!("{prefix} its field is public — make it private (`pub struct {name}(...)`) so the invariant cannot be bypassed")
    } else if rules::no_collection_newtype::NoCollectionNewtype::derives_deref_mut(struct_def) {
        format!("{prefix} derives `DerefMut` — remove it; mutate through the type's own methods")
    } else if !rules::no_collection_newtype::NoCollectionNewtype::has_invariant_impl(items, &name) {
        format!("{prefix} has no impl that enforces its invariant — add an inherent impl or a `TryFrom` that checks it")
    } else {
        return Vec::new();
    };
    vec![RuleViolation::from_file(
        file_path.to_path_buf(),
        rules::no_collection_newtype::NoCollectionNewtype::CODE,
        problem,
    )]
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use std::path::Path;

    #[test]
    fn test_usage() {
        let file =
            syn::parse_str::<syn::File>("impl Deck { pub fn size(&self) -> usize { 0 } }").unwrap();
        let strukt: syn::ItemStruct = syn::parse_quote!(
            pub struct Deck(Vec<Card>);
        );
        assert!(
            rules::no_collection_newtype::NoCollectionNewtype::check_listed_wrapper(
                &strukt,
                Path::new("x.rs"),
                &file.items,
            )
            .is_empty()
        );
    }
}
