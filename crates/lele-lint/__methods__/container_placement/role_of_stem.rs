use crate::rules;

pub fn role_of_stem(stem: &str) -> Option<rules::container_placement::Role> {
    rules::container_placement::Role::ALL
        .into_iter()
        .find(|role| role.file().strip_suffix(".rs") == Some(stem))
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        assert!(matches!(
            rules::container_placement::ContainerPlacement::role_of_stem("structs"),
            Some(rules::container_placement::Role::Struct)
        ));
        assert!(rules::container_placement::ContainerPlacement::role_of_stem("widgets").is_none());
    }
}
