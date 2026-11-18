use std::collections::HashSet;

use crate::scan;

pub fn preview_names(items: &[syn::Item]) -> HashSet<String> {
    let file = syn::File {
        shebang: None,
        attrs: Vec::new(),
        items: items.to_vec(),
    };
    scan::ScannedFile(&file)
        .collect_previews()
        .into_iter()
        .map(|preview| preview.name)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::preview_names;

    #[test]
    fn test_usage() {
        let file: syn::File =
            syn::parse_str("#[test] fn wheel_ui_png_preview() {} fn helper() {}").unwrap();
        let names = preview_names(&file.items);
        assert!(names.contains("wheel_ui_png_preview"));
        assert!(!names.contains("helper"));
    }
}
