use crate::scan;
pub fn declared_components(file: &scan::ScannedFile) -> Vec<String> {
    let mut names: Vec<String> = file
        .items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Struct(data) if scan::ScannedFile::derives_component(&data.attrs) => {
                Some(data.ident.to_string())
            }
            _ => None,
        })
        .collect();
    names.sort();
    names.dedup();
    names
}

#[cfg(test)]
mod tests {
    use super::declared_components;
    use crate::scan;

    #[test]
    fn test_usage() {
        let file = syn::parse_str(
            "#[derive(Component)] pub struct Root;
             #[derive(Component, Debug)] pub struct Button;
             pub struct Plain;
             #[derive(Debug)] pub struct NotAComponent;",
        )
        .unwrap();
        let found = declared_components(&scan::ScannedFile(&file));
        assert_eq!(found, vec!["Button".to_string(), "Root".to_string()]);
    }
}
