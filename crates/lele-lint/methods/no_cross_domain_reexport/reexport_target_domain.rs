pub fn reexport_target_domain(segments: &[String]) -> Option<String> {
    if segments.len() >= 2 && segments.first().is_some_and(|s| s == "crate") {
        return segments.get(1).cloned();
    }
    None
}

// no test_usage necessary
