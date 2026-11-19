use crate::rules;

pub(crate) fn qualified_trait_impl_hit(
    line: usize,
    trait_segments: &[&str],
) -> rules::trait_impl_co_location::ImplHit {
    rules::trait_impl_co_location::ImplHit {
        line,
        trait_name: trait_segments.last().unwrap_or(&"").to_string(),
        trait_path: Some(
            trait_segments
                .iter()
                .map(|segment| segment.to_string())
                .collect(),
        ),
        type_name: Some("T".to_string()),
        type_path: Some(vec!["T".to_string()]),
        is_blanket: true,
    }
}

#[cfg(test)]
mod tests {
    use super::qualified_trait_impl_hit;

    #[test]
    fn test_usage() {
        assert_eq!(
            qualified_trait_impl_hit(3, &["crate", "a", "Foo"]).trait_name,
            "Foo"
        );
    }
}
