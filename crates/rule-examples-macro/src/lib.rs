use std::path::Path;
use std::path::PathBuf;

use proc_macro::TokenStream;
use quote::quote;
use syn::LitStr;
use syn::parse_macro_input;

/// Expands `rule_examples!("atomic_file/bad")` into a `&[ExampleFile]` of that folder.
///
/// One entry per file under `<CARGO_MANIFEST_DIR>/rule_examples/atomic_file/bad/`, shallow
/// paths first (so `src/lib.rs` precedes `src/a/b.rs`), then in path order.
/// `ExampleFile` must be in scope at the call site.
#[proc_macro]
pub fn rule_examples(input: TokenStream) -> TokenStream {
    let folder = parse_macro_input!(input as LitStr);
    let root = std::env::var_os("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_default()
        .join("rule_examples")
        .join(folder.value());
    let mut files = Vec::new();
    if let Err(error) = collect_files(&root, &mut files) {
        let message = format!("cannot read `{}`: {error}", root.display());
        return syn::Error::new(folder.span(), message)
            .to_compile_error()
            .into();
    }
    if files.is_empty() {
        let message = format!("`{}` holds no example files", root.display());
        return syn::Error::new(folder.span(), message)
            .to_compile_error()
            .into();
    }
    let mut entries = files
        .iter()
        .filter_map(|file| {
            let relative = file.strip_prefix(&root).ok()?.to_str()?.replace('\\', "/");
            Some((relative, file.to_str()?.to_owned()))
        })
        .collect::<Vec<_>>();
    entries.sort_by_key(|(relative, _)| (relative.matches('/').count(), relative.clone()));
    let items = entries.iter().map(|(relative, absolute)| {
        quote! { ExampleFile { path: #relative, source: include_str!(#absolute) } }
    });
    quote! { &[#(#items),*] }.into()
}

fn collect_files(dir: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_files(&path, files)?;
        } else {
            files.push(path);
        }
    }
    Ok(())
}
