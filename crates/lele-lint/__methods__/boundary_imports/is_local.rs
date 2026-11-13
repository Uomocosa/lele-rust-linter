pub fn is_local(path: &str) -> bool {
    ["crate", "self", "super"]
        .iter()
        .any(|root| path == *root || path.starts_with(&format!("{root}::")))
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        assert!(rules::boundary_imports::BoundaryImports::is_local(
            "crate::stock"
        ));
        assert!(rules::boundary_imports::BoundaryImports::is_local("super"));
        assert!(!rules::boundary_imports::BoundaryImports::is_local(
            "cratex::stock"
        ));
    }
}
