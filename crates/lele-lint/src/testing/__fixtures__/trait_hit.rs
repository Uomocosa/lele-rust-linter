use crate::rules;

pub(crate) fn trait_hit(line: usize, name: &str) -> rules::trait_placement::TraitHit {
    rules::trait_placement::TraitHit {
        line,
        name: name.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::trait_hit;

    #[test]
    fn test_usage() {
        assert_eq!(trait_hit(3, "Sessions").name, "Sessions");
    }
}
