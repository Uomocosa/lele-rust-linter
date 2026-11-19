use std::collections::BTreeSet;

pub(crate) fn no_visuals() -> BTreeSet<String> {
    BTreeSet::new()
}

#[cfg(test)]
mod tests {
    use super::no_visuals;

    #[test]
    fn test_usage() {
        assert!(no_visuals().is_empty());
    }
}
