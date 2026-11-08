use std::collections::BTreeSet;
use std::path::Path;

use crate::Project;

pub fn wrong_root_message(
    project: &Project,
    rel: &Path,
    actual_root: Option<&String>,
    type_snake: &str,
    expected: Option<&BTreeSet<String>>,
) -> String {
    let label = project
        .methods_dir
        .as_ref()
        .and_then(|dir| dir.file_name())
        .and_then(|name| name.to_str())
        .unwrap_or("methods");
    let mut names: Vec<&String> = expected
        .map(|roots| roots.iter().collect())
        .unwrap_or_default();
    names.sort();
    let targets = names
        .iter()
        .map(|root| format!("`{label}/{root}/{type_snake}/`"))
        .collect::<Vec<_>>()
        .join(", ");
    let expected_blurb = match names.as_slice() {
        [single] => format!("is declared in root `{single}` — move it to {targets}"),
        _ => format!(
            "is declared in roots {} — move it to one of {targets}",
            names
                .iter()
                .map(|name| format!("`{name}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    };
    match actual_root {
        Some(actual) => format!(
            "method file `methods/{}` is under root `{actual}` but type `{type_snake}` {expected_blurb}",
            rel.display()
        ),
        None => format!(
            "method file `methods/{}` is not under any root subfolder but type `{type_snake}` {expected_blurb}",
            rel.display()
        ),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::Path;
    use std::path::PathBuf;

    use super::wrong_root_message;
    use crate::Project;

    #[test]
    fn test_usage() {
        let project = Project {
            methods_dir: Some(PathBuf::from("__methods__")),
            ..Project::default()
        };
        let expected: BTreeSet<String> = BTreeSet::from(["src".to_string()]);
        let message = wrong_root_message(
            &project,
            Path::new("ui/widget/spin.rs"),
            Some("ui".to_string()).as_ref(),
            "widget",
            Some(&expected),
        );
        assert!(message.contains("under root `ui`"));
        assert!(message.contains("declared in root `src`"));
        assert!(message.contains("`__methods__/src/widget/`"));
        let bare = wrong_root_message(
            &project,
            Path::new("widget/spin.rs"),
            None,
            "widget",
            Some(&expected),
        );
        assert!(bare.contains("not under any root subfolder"));
    }
}
