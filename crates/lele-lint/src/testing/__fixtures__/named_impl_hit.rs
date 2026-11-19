use crate::rules;

pub(crate) fn named_impl_hit(
    line: usize,
    trait_name: &str,
    type_name: &str,
) -> rules::trait_impl_co_location::ImplHit {
    rules::trait_impl_co_location::ImplHit {
        line,
        trait_name: trait_name.to_string(),
        trait_path: Some(vec![trait_name.to_string()]),
        type_name: Some(type_name.to_string()),
        type_path: Some(vec![type_name.to_string()]),
        is_blanket: false,
    }
}

#[cfg(test)]
mod tests {
    use super::named_impl_hit;

    #[test]
    fn test_usage() {
        assert_eq!(
            named_impl_hit(3, "Clock", "UnixTime").type_name.as_deref(),
            Some("UnixTime")
        );
    }
}
