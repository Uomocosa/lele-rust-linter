use std::fmt::Write;

pub fn glue_index_content(root_names: &[String]) -> String {
    let mut out = String::new();
    for root in root_names {
        let _ = writeln!(out, "#[path = \"{root}/mod.rs\"]");
        let _ = writeln!(out, "mod {root}_root;");
        let _ = writeln!(out, "pub use {root}_root::*;");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::glue_index_content;

    #[test]
    fn test_usage() {
        assert_eq!(
            glue_index_content(&["src".to_string(), "ui".to_string()]),
            "#[path = \"src/mod.rs\"]\nmod src_root;\npub use src_root::*;\n#[path = \"ui/mod.rs\"]\nmod ui_root;\npub use ui_root::*;\n"
        );
    }

    #[test]
    fn test_usage_single_root_has_one_block() {
        assert_eq!(
            glue_index_content(&["src".to_string()]),
            "#[path = \"src/mod.rs\"]\nmod src_root;\npub use src_root::*;\n"
        );
    }
}
