use crate::scan;

pub fn module_fns<'b>(file: &'b scan::ScannedFile<'_>) -> scan::ModuleFns<'b> {
    let declared = file.items.iter().filter_map(|item| match item {
        syn::Item::Fn(func) => Some((func.sig.ident.to_string(), func)),
        _ => None,
    });
    declared.collect()
}

#[cfg(test)]
mod tests {
    use super::module_fns;
    use crate::scan;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_str("fn build(app: &mut App) {}").unwrap();
        let scanned = scan::ScannedFile(&file);
        let found = module_fns(&scanned);
        assert!(found.contains_key("build"));
    }
}
