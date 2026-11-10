use crate::scan;

pub fn called_sibling<'b>(
    arg: &syn::Expr,
    module_fns: &scan::ModuleFns<'b>,
) -> Option<&'b syn::ItemFn> {
    let syn::Expr::Call(call) = arg else {
        return None;
    };
    let syn::Expr::Path(path) = &*call.func else {
        return None;
    };
    if path.path.segments.len() != 1 {
        return None;
    }
    let name = path.path.segments.first()?.ident.to_string();
    module_fns.get(&name).copied()
}

#[cfg(test)]
mod tests {
    use super::called_sibling;
    use crate::scan;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_str("fn helper() {}").unwrap();
        let module_fns: scan::ModuleFns<'_> = scan::ScannedFile::module_fns_of(&file.items);
        let arg: syn::Expr = syn::parse_str("helper()").unwrap();
        assert!(called_sibling(&arg, &module_fns).is_some());
        let _ = scan::ScannedFile::module_fns_of(&[]);
    }
}
