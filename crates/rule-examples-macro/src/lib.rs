use std::path::Path;
use std::path::PathBuf;

use proc_macro::TokenStream;
use quote::quote;
use syn::LitStr;

#[proc_macro]
pub fn rule_examples(input: TokenStream) -> TokenStream {
    <ExampleFolderScan as TryFrom<proc_macro2::TokenStream>>::try_from(input.into())
        .and_then(ExampleFolderScan::listing)
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}

pub(crate) struct ExampleFolderScan {
    scan_root: PathBuf,
    folder: LitStr,
    files: Vec<PathBuf>,
}

impl TryFrom<proc_macro2::TokenStream> for ExampleFolderScan {
    type Error = syn::Error;

    fn try_from(input_tokens: proc_macro2::TokenStream) -> Result<Self, Self::Error> {
        let folder = syn::parse2::<LitStr>(input_tokens)?;
        Ok(Self {
            scan_root: Self::scan_root(&folder),
            folder,
            files: Vec::new(),
        })
    }
}

impl ExampleFolderScan {
    pub(crate) fn scan_root(folder: &LitStr) -> PathBuf {
        let manifest = std::env::var_os("CARGO_MANIFEST_DIR")
            .map(PathBuf::from)
            .unwrap_or_default();
        manifest.join("rule_examples").join(folder.value())
    }

    pub(crate) fn listing(mut self) -> syn::Result<proc_macro2::TokenStream> {
        self.collect_files()?;
        self.empty_folder_violation()?;
        Ok(self.listing_tokens())
    }

    pub(crate) fn collect_files(&mut self) -> syn::Result<()> {
        fn recursion(dir: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
            for entry in std::fs::read_dir(dir)? {
                let path = entry?.path();
                if path.is_dir() {
                    recursion(&path, files)?;
                } else {
                    files.push(path);
                }
            }
            Ok(())
        }
        recursion(&self.scan_root, &mut self.files).map_err(|error| {
            syn::Error::new(
                self.folder.span(),
                format!("cannot read `{}`: {error}", self.scan_root.display()),
            )
        })
    }

    pub(crate) fn empty_folder_violation(&self) -> syn::Result<()> {
        if !self.files.is_empty() {
            return Ok(());
        }
        Err(syn::Error::new(
            self.folder.span(),
            format!("`{}` holds no example files", self.scan_root.display()),
        ))
    }

    pub(crate) fn listing_tokens(&self) -> proc_macro2::TokenStream {
        let entries = self.sorted_entries();
        let items = entries.iter().map(|(relative, absolute)| {
            quote! { ExampleFile { path: #relative, source: include_str!(#absolute) } }
        });
        quote! { &[#(#items),*] }
    }

    pub(crate) fn sorted_entries(&self) -> Vec<(String, String)> {
        let mut pairs = self.entry_pairs();
        pairs.sort_by_key(|(relative, _)| (relative.matches('/').count(), relative.clone()));
        pairs
    }

    pub(crate) fn entry_pairs(&self) -> Vec<(String, String)> {
        self.files
            .iter()
            .filter_map(|file| {
                let relative = file
                    .strip_prefix(&self.scan_root)
                    .ok()?
                    .to_str()?
                    .replace('\\', "/");
                Some((relative, file.to_str()?.to_owned()))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::ExampleFolderScan;
    use quote::quote;

    #[test]
    fn test_usage() {
        let tokens = <ExampleFolderScan as TryFrom<proc_macro2::TokenStream>>::try_from(quote!(
            "order_check"
        ))
        .expect("folder literal parses")
        .listing()
        .expect("fixture folder is scanned");
        let expanded = tokens.to_string();
        let first = expanded.find("path : \"first.rs\"").expect("first listed");
        let nested = expanded
            .find("path : \"nested/second.rs\"")
            .expect("nested listed");
        assert!(first < nested, "shallow paths are listed first");
        assert!(expanded.contains("include_str !"));
    }
}
