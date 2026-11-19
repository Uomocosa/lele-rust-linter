use crate::rules;

pub(crate) fn unnamed_impl_hit(
    line: usize,
    trait_name: &str,
) -> rules::trait_impl_co_location::ImplHit {
    rules::trait_impl_co_location::ImplHit {
        line,
        trait_name: trait_name.to_string(),
        trait_path: Some(vec![trait_name.to_string()]),
        type_name: None,
        type_path: None,
        is_blanket: false,
    }
}

#[cfg(test)]
mod tests {
    use super::unnamed_impl_hit;

    #[test]
    fn test_usage() {
        assert!(unnamed_impl_hit(3, "Clock").type_name.is_none());
    }
}
