use std::path::Path;

use crate::rules;

pub fn offending_component(rel_path: &Path) -> Option<&str> {
    let mut components = rel_path.components();
    let first = components.next()?;
    if first.as_os_str() != "testing" {
        return None;
    }
    let second = components.next()?;
    let name = second.as_os_str().to_str()?;
    if name == "mod.rs" || rules::testing_layout::TESTING_ROLES.contains(&name) {
        return None;
    }
    Some(name)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::offending_component;
    use crate::rules;

    #[test]
    fn test_usage() {
        assert_eq!(offending_component(Path::new("testing/mod.rs")), None);
        assert_eq!(
            offending_component(Path::new("testing/__fixtures__/row.rs")),
            None
        );
        assert_eq!(
            offending_component(Path::new("testing/__app__/harness/mod.rs")),
            None
        );
        assert_eq!(
            offending_component(Path::new("testing/harness/lobby.rs")),
            Some("harness")
        );
        assert_eq!(
            offending_component(Path::new("testing/keys.rs")),
            Some("keys.rs")
        );
        assert_eq!(offending_component(Path::new("stock/mod.rs")), None);
        assert!(
            rules::testing_layout::TestingLayout::offending_component(Path::new(
                "testing/fakes/kit.rs"
            ))
            .is_some()
        );
    }
}
