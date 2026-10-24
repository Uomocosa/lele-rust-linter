pub fn named_message(kind: &str, name: &str, vis: &syn::Visibility) -> String {
    let vis_str = match vis {
        syn::Visibility::Public(_) => "pub ",
        syn::Visibility::Restricted(_) => "pub(crate) ",
        syn::Visibility::Inherited => "",
    };
    format!(
        "mod.rs may contain only `mod`/`pub mod` declarations and `pub use` re-exports — found `{vis_str}{kind} {name}`; move it to its own file"
    )
}

// no test_usage necessary
