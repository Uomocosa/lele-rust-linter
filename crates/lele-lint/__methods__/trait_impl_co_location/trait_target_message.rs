use std::path::Path;

pub fn trait_target_message(trait_name: &str, type_name: &str, target: &Path) -> String {
    format!("`impl {trait_name} for {type_name}` must live in `{}`, the file that defines `{trait_name}`", target.display())
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use std::path::Path;

    #[test]
    fn test_usage() {
        let message = rules::trait_impl_co_location::TraitImplCoLocation::trait_target_message(
            "Shape",
            "Circle",
            Path::new("src/shape/traits/shape.rs"),
        );
        assert!(message.contains("must live in `src/shape/traits/shape.rs`"));
        assert!(message.ends_with("the file that defines `Shape`"));
    }
}
