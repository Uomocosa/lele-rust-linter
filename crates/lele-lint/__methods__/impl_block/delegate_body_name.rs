use lele_snake_case::ToLeleSnakeCase;

use crate::ImplBlock;

pub fn delegate_body_name(impl_block: ImplBlock, method: &syn::Ident) -> String {
    fn recursion(ty: &syn::Type) -> Option<String> {
        match ty {
            syn::Type::Reference(reference) => recursion(&reference.elem),
            syn::Type::Paren(paren) => recursion(&paren.elem),
            syn::Type::Slice(slice) => recursion(&slice.elem).map(|elem| format!("{elem}_slice")),
            syn::Type::Path(path) => path
                .path
                .segments
                .last()
                .map(|segment| segment.ident.to_string()),
            _ => None,
        }
    }

    let trait_args = impl_block
        .trait_
        .as_ref()
        .and_then(|(_, path, _)| path.segments.last())
        .and_then(|segment| match &segment.arguments {
            syn::PathArguments::AngleBracketed(args) => Some(args),
            _ => None,
        });
    let suffix: String = trait_args
        .into_iter()
        .flat_map(|args| args.args.iter())
        .filter_map(|arg| match arg {
            syn::GenericArgument::Type(ty) => recursion(ty),
            _ => None,
        })
        .map(|name| format!("_{}", name.to_snake_case()))
        .collect::<Vec<_>>()
        .concat();
    format!("{method}{suffix}")
}

#[cfg(test)]
mod tests {
    use super::delegate_body_name;
    use crate::ImplBlock;

    #[test]
    fn test_usage() {
        let from: syn::ItemImpl = syn::parse_quote! { impl From<Celsius> for Temperature { fn from(c: Celsius) -> Self {} } };
        let ident: syn::Ident = syn::parse_quote!(from);
        assert_eq!(delegate_body_name(ImplBlock(&from), &ident), "from_celsius");
        let try_from: syn::ItemImpl =
            syn::parse_quote! { impl<'a> TryFrom<&'a str> for Temperature {} };
        let ident: syn::Ident = syn::parse_quote!(try_from);
        assert_eq!(
            delegate_body_name(ImplBlock(&try_from), &ident),
            "try_from_str"
        );
        let bytes: syn::ItemImpl = syn::parse_quote! { impl From<&[u8]> for Hash {} };
        let ident: syn::Ident = syn::parse_quote!(from);
        assert_eq!(
            delegate_body_name(ImplBlock(&bytes), &ident),
            "from_u8_slice"
        );
        let plain: syn::ItemImpl = syn::parse_quote! { impl Rule for Foo {} };
        let ident: syn::Ident = syn::parse_quote!(check);
        assert_eq!(delegate_body_name(ImplBlock(&plain), &ident), "check");
    }
}
