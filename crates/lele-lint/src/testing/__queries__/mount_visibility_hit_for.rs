use crate::methods;
use crate::DunderPath;

pub(crate) fn mount_visibility_hit_for(
    methods_mount_name: &str,
    ident: &str,
    vis: &syn::Visibility,
) -> Option<String> {
    methods::dunder_module_name::mount_visibility_hit(
        &DunderPath::defaults(),
        methods_mount_name,
        ident,
        vis,
    )
}

#[cfg(test)]
mod tests {
    use super::mount_visibility_hit_for;
    use crate::methods;
    use crate::DunderPath;

    #[test]
    fn test_usage() {
        let public: syn::Visibility = syn::parse_quote! { pub };
        assert!(mount_visibility_hit_for("methods", "basic", &public).is_some());
        assert_eq!(
            methods::dunder_module_name::mount_visibility_hit(
                &DunderPath::defaults(),
                "methods",
                "basic",
                &syn::Visibility::Inherited,
            ),
            None
        );
    }
}
