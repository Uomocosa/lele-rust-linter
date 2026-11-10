use crate::rules;
use crate::DunderKind;
use crate::DunderPath;
use crate::Visibility;

pub fn mount_visibility_hit(
    dunders: &[DunderPath],
    methods_mount_name: &str,
    ident: &str,
    vis: &syn::Visibility,
) -> Option<String> {
    let rendered = rules::dunder_module_name::DunderModuleName::render_visibility(vis);

    if ident == methods_mount_name {
        if rendered.is_empty() {
            return None;
        }
        return Some(format!(
            "`{rendered} mod {ident};` must be private: write `mod {ident};` — delegate bodies are reached through their type's methods"
        ));
    }

    let dunder = dunders
        .iter()
        .find(|dunder| dunder.is_dunder_dir() && dunder.module_name() == ident)?;

    match dunder.kind() {
        DunderKind::Container => {
            if rendered.is_empty() {
                return None;
            }
            Some(format!(
                "`{rendered} mod {ident};` must be private: write `mod {ident};` and re-export each item flat (`pub use {ident}::enums::HelloKind;`)"
            ))
        }
        DunderKind::Code => {
            if Visibility(vis).is_exposed() {
                return None;
            }
            let shown = if rendered.is_empty() {
                format!("mod {ident};")
            } else {
                format!("{rendered} mod {ident};")
            };
            Some(format!(
                "`{shown}` hides the mount — declare `pub mod {ident};` or `pub(crate) mod {ident};`; callers write `domain::{ident}::<name>`"
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DunderPath;

    use super::mount_visibility_hit;

    fn hits_for(methods_mount_name: &str, ident: &str, vis: &syn::Visibility) -> Option<String> {
        mount_visibility_hit(&DunderPath::defaults(), methods_mount_name, ident, vis)
    }

    #[test]
    fn test_usage() {
        let public: syn::Visibility = syn::parse_quote! { pub };
        let found = hits_for("methods", "basic", &public);
        assert_eq!(
            found.as_deref(),
            Some("`pub mod basic;` must be private: write `mod basic;` and re-export each item flat (`pub use basic::enums::HelloKind;`)")
        );
    }

    #[test]
    fn test_usage_private_basic_mount_is_clean() {
        let inherited = syn::Visibility::Inherited;
        assert_eq!(hits_for("methods", "basic", &inherited), None);
    }

    #[test]
    fn test_usage_pub_crate_basic_mount_is_flagged() {
        let crate_vis: syn::Visibility = syn::parse_quote! { pub(crate) };
        let found = hits_for("methods", "basic", &crate_vis);
        assert!(found
            .as_deref()
            .is_some_and(|message| message.starts_with("`pub(crate) mod basic;` must be private")));
    }

    #[test]
    fn test_usage_methods_mount_must_be_private() {
        let public: syn::Visibility = syn::parse_quote! { pub };
        let found = hits_for("methods", "methods", &public);
        assert!(found
            .as_deref()
            .is_some_and(|message| message.starts_with("`pub mod methods;` must be private")));
        assert_eq!(
            hits_for("methods", "methods", &syn::Visibility::Inherited),
            None
        );
    }

    #[test]
    fn test_usage_methods_mount_follows_the_configured_root() {
        let public: syn::Visibility = syn::parse_quote! { pub };
        let inherited = syn::Visibility::Inherited;
        let found = hits_for("bodies", "bodies", &public);
        assert!(found
            .as_deref()
            .is_some_and(|message| message.starts_with("`pub mod bodies;` must be private")));
        assert_eq!(hits_for("bodies", "bodies", &inherited), None);
        assert_eq!(hits_for("bodies", "methods", &public), None);
        assert_eq!(hits_for("bodies", "stock", &public), None);
    }

    #[test]
    fn test_usage_code_mounts_must_be_exposed() {
        let inherited = syn::Visibility::Inherited;
        let traits = hits_for("methods", "traits", &inherited);
        assert!(traits
            .as_deref()
            .is_some_and(|message| message.contains("declare `pub mod traits;`")));
        let systems = hits_for("methods", "bevy_systems", &inherited);
        assert!(systems
            .as_deref()
            .is_some_and(|message| message.contains("domain::bevy_systems::<name>")));
    }

    #[test]
    fn test_usage_exposed_code_mounts_are_clean() {
        let public: syn::Visibility = syn::parse_quote! { pub };
        let crate_vis: syn::Visibility = syn::parse_quote! { pub(crate) };
        assert_eq!(hits_for("methods", "traits", &public), None);
        assert_eq!(hits_for("methods", "traits", &crate_vis), None);
        assert_eq!(hits_for("methods", "bevy_systems", &public), None);
        assert_eq!(hits_for("methods", "bevy_systems", &crate_vis), None);
    }

    #[test]
    fn test_usage_unknown_modules_are_ignored() {
        let public: syn::Visibility = syn::parse_quote! { pub };
        assert_eq!(hits_for("methods", "stock", &public), None);
        assert_eq!(hits_for("methods", "prelude", &public), None);
    }
}
