pub fn is_excluded(derives: &[String]) -> bool {
    derives.iter().any(|derive| {
        matches!(
            derive.as_str(),
            "Error" | "Parser" | "Args" | "Subcommand" | "ValueEnum" | "NetworkBehaviour"
        )
    })
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        assert!(rules::container_placement::ContainerPlacement::is_excluded(
            &["Debug".to_string(), "Error".to_string(),]
        ));
        assert!(
            !rules::container_placement::ContainerPlacement::is_excluded(&["Debug".to_string()])
        );
    }
}
