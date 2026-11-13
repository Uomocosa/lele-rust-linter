pub fn generic_suggestion() -> (String, String) {
    (
        "use crate::<domain>;".to_string(),
        "<domain>::…".to_string(),
    )
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let (import, path) = rules::no_super_imports::NoSuperImports::generic_suggestion();
        assert_eq!(import, "use crate::<domain>;");
        assert_eq!(path, "<domain>::…");
    }
}
