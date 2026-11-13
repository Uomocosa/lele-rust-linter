use std::path::Path;

pub fn container_target_message(
    trait_name: &str,
    type_name: &str,
    target: &Path,
    graduate: &Path,
) -> String {
    format!("`impl {trait_name} for {type_name}` lives outside `{}`, which defines `{type_name}`: a type with an impl keeps its own file — move `{type_name}` to `{}` and put the impl there", target.display(), graduate.display())
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use std::path::Path;

    #[test]
    fn test_usage() {
        let message = rules::trait_impl_co_location::TraitImplCoLocation::container_target_message(
            "Shape",
            "Circle",
            Path::new("src/basic/structs.rs"),
            Path::new("src/circle.rs"),
        );
        assert!(message.contains("lives outside `src/basic/structs.rs`"));
        assert!(message.contains("move `Circle` to `src/circle.rs`"));
    }
}
