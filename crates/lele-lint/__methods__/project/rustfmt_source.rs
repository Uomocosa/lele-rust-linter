use crate::RustfmtKey;

pub fn rustfmt_source(candidates: &[(RustfmtKey, String)]) -> String {
    let methods: Vec<String> = candidates
        .iter()
        .enumerate()
        .map(|(index, (_, text))| format!("    fn __lele_{index}() {{\n        {text}\n    }}\n"))
        .collect();
    format!("impl __Lele {{\n{}}}\n", methods.concat())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::RustfmtKey;

    #[test]
    fn test_usage() {
        let key: RustfmtKey = (PathBuf::from("a.rs"), 1, 0);
        let source =
            super::rustfmt_source(&[(key.clone(), "g()".to_string()), (key, "h()".to_string())]);
        assert!(source.contains("fn __lele_0() {\n        g()\n    }"));
        assert!(source.contains("fn __lele_1() {"));
    }
}
