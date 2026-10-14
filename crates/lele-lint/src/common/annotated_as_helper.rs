pub(crate) fn annotated_as_helper(lines: &[&str], fn_line: usize) -> bool {
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
    use super::annotated_as_helper;

    #[test]
    fn test_usage() {
        let lines = vec!["// needed helper: why", "fn helper() {}"];
        assert!(annotated_as_helper(&lines, 2));
    }

    #[test]
    fn test_usage_skips_attributes_and_blanks() {
        let lines = vec!["// needed helper: why", "", "#[test]", "fn helper() {}"];
        assert!(annotated_as_helper(&lines, 4));
    }

    #[test]
    fn test_usage_without_annotation() {
        let lines = vec!["use std::path::Path;", "", "fn helper() {}"];
        assert!(!annotated_as_helper(&lines, 3));
    }
}

// no test_usage necessary
