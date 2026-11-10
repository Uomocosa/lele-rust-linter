use crate::common;
use std::fmt::Write;

pub fn index_content(declared: &common::DeclaredType) -> String {
    let mut out = String::new();
    for method in &declared.methods {
        let _ = writeln!(out, "mod {method};");
    }
    for method in &declared.methods {
        let _ = writeln!(out, "pub use {method}::{method};");
    }
    out
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::index_content;
    use crate::common::DeclaredType;

    #[test]
    fn test_usage() {
        let methods: BTreeSet<String> = ["add".to_string(), "increment".to_string()].into();
        assert_eq!(
            index_content(&DeclaredType {
                methods,
                cfgs: Vec::new()
            }),
            "mod add;\nmod increment;\npub use add::add;\npub use increment::increment;\n"
        );
    }
}
