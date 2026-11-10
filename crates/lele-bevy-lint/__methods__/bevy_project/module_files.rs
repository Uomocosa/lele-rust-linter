use std::path::Path;
use std::path::PathBuf;

use crate::scan;

pub fn module_files(declaring: &Path, module: &syn::ItemMod) -> Vec<PathBuf> {
    let dir = declaring.parent().unwrap_or_else(|| Path::new(""));
    if let Some(path) = scan::BevyProject::path_attr(&module.attrs) {
        return vec![scan::BevyProject::normalize(&dir.join(path))];
    }
    let name = module.ident.to_string();
    let is_mod_rs = declaring
        .file_name()
        .is_some_and(|file| file == "mod.rs" || file == "lib.rs" || file == "main.rs");
    let base = if is_mod_rs {
        dir.to_path_buf()
    } else {
        dir.join(declaring.file_stem().unwrap_or_default())
    };
    vec![
        scan::BevyProject::normalize(&base.join(format!("{name}.rs"))),
        scan::BevyProject::normalize(&base.join(&name).join("mod.rs")),
    ]
}

#[cfg(test)]
mod tests {
    use super::module_files;
    use std::path::Path;

    #[test]
    fn test_usage() {
        let module: syn::ItemMod = syn::parse_quote! { mod helpers; };
        let found = module_files(Path::new("lib.rs"), &module);
        assert!(
            found
                .iter()
                .any(|p| p.to_string_lossy().contains("helpers"))
        );
    }
}
