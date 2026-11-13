use crate::Visibility;
use lele_snake_case::traits::ToLeleSnakeCase;

pub fn primary_exposed_type(file: &syn::File, stem: &str) -> Option<String> {
    file.items.iter().find_map(|item| {
        let (ident, vis) = match item {
            syn::Item::Struct(s) => (&s.ident, &s.vis),
            syn::Item::Enum(e) => (&e.ident, &e.vis),
            _ => return None,
        };
        if !Visibility(vis).is_exposed() {
            return None;
        }
        let name = ident.to_string();
        if name.to_snake_case() == stem {
            Some(name)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_quote! { pub struct Client; };
        assert_eq!(
            rules::no_stuttered_type::NoStutteredType::primary_exposed_type(&file, "client"),
            Some("Client".to_string())
        );
        assert!(
            rules::no_stuttered_type::NoStutteredType::primary_exposed_type(&file, "other")
                .is_none()
        );
    }
}
