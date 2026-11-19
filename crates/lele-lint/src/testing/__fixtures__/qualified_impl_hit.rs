use crate::rules;

pub(crate) fn qualified_impl_hit(
    line: usize,
    trait_name: &str,
    segments: &[&str],
) -> rules::trait_impl_co_location::ImplHit {
    rules::trait_impl_co_location::ImplHit {
        line,
        trait_name: trait_name.to_string(),
        trait_path: Some(vec![trait_name.to_string()]),
        type_name: Some(segments.last().unwrap_or(&"").to_string()),
        type_path: Some(segments.iter().map(|segment| segment.to_string()).collect()),
        is_blanket: false,
    }
}

#[cfg(test)]
mod tests {
    use super::qualified_impl_hit;

    #[test]
    fn test_usage() {
        assert_eq!(
            qualified_impl_hit(3, "Default", &["crate", "a", "Config"])
                .type_path
                .as_deref(),
            Some(&["crate".to_string(), "a".to_string(), "Config".to_string()][..])
        );
    }
}
