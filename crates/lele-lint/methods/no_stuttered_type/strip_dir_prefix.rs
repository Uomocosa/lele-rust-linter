pub fn strip_dir_prefix(snake: &str, dir: &str) -> Option<String> {
    let rest = snake.strip_prefix(dir)?;
    let suffix = rest.strip_prefix('_')?;
    if suffix.is_empty() || suffix.len() < 3 {
        return None;
    }
    Some(suffix.to_string())
}

// no test_usage necessary
