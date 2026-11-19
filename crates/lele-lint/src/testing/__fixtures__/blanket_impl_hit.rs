use crate::rules;

pub(crate) fn blanket_impl_hit(
    line: usize,
    trait_name: &str,
) -> rules::trait_impl_co_location::ImplHit {
    rules::trait_impl_co_location::ImplHit {
        line,
        trait_name: trait_name.to_string(),
        trait_path: Some(vec![trait_name.to_string()]),
        type_name: Some("T".to_string()),
        type_path: Some(vec!["T".to_string()]),
        is_blanket: true,
    }
}

#[cfg(test)]
mod tests {
    use super::blanket_impl_hit;

    #[test]
    fn test_usage() {
        assert!(blanket_impl_hit(3, "Clock").is_blanket);
    }
}
