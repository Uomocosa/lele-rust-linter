use crate::RawCode;
pub fn is_annotated_as_helper(code: &RawCode, fn_line: usize) -> bool {
    let lines: Vec<&str> = code.lines().collect();
    let mut idx = fn_line.saturating_sub(1);
    while idx > 0 {
        idx = idx.saturating_sub(1);
        let candidate = lines.get(idx).map(|line| line.trim()).unwrap_or("");
        if candidate.is_empty() || candidate.starts_with("#[") {
            continue;
        }
        return candidate.starts_with("// needed helper:");
    }
    false
}

#[cfg(test)]
mod tests {
    use super::is_annotated_as_helper;
    use crate::RawCode;

    #[test]
    fn test_usage() {
        let lines = ["// needed helper: why", "fn helper() {}"].join("\n");
        assert!(is_annotated_as_helper(&RawCode(&lines), 2));
    }

    #[test]
    fn test_usage_skips_attributes_and_blanks() {
        let lines = ["// needed helper: why", "", "#[test]", "fn helper() {}"].join("\n");
        assert!(is_annotated_as_helper(&RawCode(&lines), 4));
    }

    #[test]
    fn test_usage_without_annotation() {
        let lines = ["use std::path::Path;", "", "fn helper() {}"].join("\n");
        assert!(!is_annotated_as_helper(&RawCode(&lines), 3));
    }
}

// no test_usage necessary
