use std::path::Path;
use std::path::PathBuf;

use crate::rules;
use crate::ItemMarker;

pub fn out_of_line_test_files(rel_path: &Path, file: &syn::File) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    let Some(dir) = rel_path.parent() else {
        return candidates;
    };
    for item in &file.items {
        let syn::Item::Mod(module) = item else {
            continue;
        };
        let is_out_of_line_test = module.content.is_none()
            && module
                .attrs
                .iter()
                .any(|attr| ItemMarker(attr).is_cfg_test());
        let is_testing_fixture_root =
            rules::test_module_items::TestModuleItems::is_testing_fixture_root(item);
        if !is_out_of_line_test || is_testing_fixture_root {
            continue;
        }
        let name = &module.ident;
        candidates.push(dir.join(format!("{name}.rs")));
        candidates.push(dir.join(name.to_string()).join("mod.rs"));
    }
    candidates
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    use super::out_of_line_test_files;
    use crate::rules;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_quote! {
            #[cfg(test)]
            mod tests;
            #[cfg(test)]
            mod inline {
                #[test]
                fn test_usage() {}
            }
            pub struct Guest;
        };
        let candidates = out_of_line_test_files(Path::new("run/mod.rs"), &file);
        assert_eq!(
            candidates,
            vec![
                PathBuf::from("run/tests.rs"),
                PathBuf::from("run/tests/mod.rs")
            ]
        );
        assert_eq!(
            rules::test_module_items::TestModuleItems::out_of_line_test_files(
                Path::new("lib.rs"),
                &file
            ),
            vec![PathBuf::from("tests.rs"), PathBuf::from("tests/mod.rs")]
        );
    }

    #[test]
    fn test_usage_skips_the_testing_fixture_root() {
        let file: syn::File = syn::parse_quote! {
            #[cfg(test)]
            pub(crate) mod testing;
            #[cfg(test)]
            mod tests;
        };
        assert_eq!(
            out_of_line_test_files(Path::new("lib.rs"), &file),
            vec![PathBuf::from("tests.rs"), PathBuf::from("tests/mod.rs")]
        );
    }
}
