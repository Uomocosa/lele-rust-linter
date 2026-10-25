use lele_snake_case::ToLeleSnakeCase;
use std::collections::BTreeSet;

const WRAPPERS: [&str; 4] = ["Result", "Option", "Vec", "Box"];

pub fn owner_hint(sig: &syn::Signature, local_types: &BTreeSet<String>) -> String {
    fn recursion(ty: &syn::Type) -> Option<String> {
        match ty {
            syn::Type::Reference(reference) => recursion(&reference.elem),
            syn::Type::Paren(paren) => recursion(&paren.elem),
            syn::Type::Slice(slice) => recursion(&slice.elem),
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
                    syn::GenericArgument::Type(inner) => recursion(inner),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    let first_arg = sig.inputs.first().and_then(|arg| match arg {
        syn::FnArg::Typed(typed) => recursion(&typed.ty),
        syn::FnArg::Receiver(_) => None,
    });
    let returned = match &sig.output {
        syn::ReturnType::Type(_, ty) => recursion(ty),
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
            "no candidate owner is a crate type, so wrap the first argument in a newtype (or bundle the arguments in a struct) and make it the owner of `{name}`"
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
        "free fn `{name}` has no owner type — {advice}; then run `lele-lint --sync-methods`. A true process/task entry point goes in [[lele.lint.entry_points]]"
    )
}

// no test_usage necessary
