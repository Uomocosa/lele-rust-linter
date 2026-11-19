use crate::ItemMarker;

pub fn test_usage_fns(file: &syn::File) -> Vec<&syn::ItemFn> {
    let mut found = Vec::new();
    for item in &file.items {
        match item {
            syn::Item::Fn(function) if function.sig.ident == "test_usage" => found.push(function),
            syn::Item::Mod(module) => {
                let Some((_, items)) = &module.content else {
                    continue;
                };
                if !module
                    .attrs
                    .iter()
                    .any(|attr| ItemMarker(attr).is_cfg_test())
                {
                    continue;
                }
                for inner in items {
                    if let syn::Item::Fn(function) = inner {
                        if function.sig.ident == "test_usage" {
                            found.push(function);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::test_usage_fns;
    use crate::rules;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_quote! {
            pub struct Guest;

            #[cfg(test)]
            mod tests {
                #[test]
                fn test_usage() {}
            }

            #[cfg(test)]
            mod more {
                #[test]
                fn test_usage() {}
            }
        };
        assert_eq!(test_usage_fns(&file).len(), 2);
        assert_eq!(
            rules::test_usage_shape::TestUsageShape::test_usage_fns(&file).len(),
            2
        );
    }

    #[test]
    fn test_usage_finds_the_top_level_fn_of_an_out_of_line_test_file() {
        let file: syn::File = syn::parse_quote! {
            #[test]
            fn test_usage() {}
        };
        assert_eq!(test_usage_fns(&file).len(), 1);
    }

    #[test]
    fn test_usage_ignores_a_fn_outside_a_test_module() {
        let file: syn::File = syn::parse_quote! {
            mod helper {
                fn test_usage() {}
            }
        };
        assert_eq!(test_usage_fns(&file).len(), 0);
    }
}
