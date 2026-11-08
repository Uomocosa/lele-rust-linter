use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use crate::Project;

pub fn file_in_boundary(
    project: &Project,
    crate_rel: &Path,
    prefix: &Path,
    folders: &[PathBuf],
    stem_map: &HashMap<String, BTreeMap<String, PathBuf>>,
) -> bool {
    if folders
        .iter()
        .any(|folder| prefix.join(crate_rel).starts_with(folder))
    {
        return true;
    }
    let Ok(rest) = crate_rel.strip_prefix("methods") else {
        return false;
    };
    let (methods_root, inner) = project.split_methods_rel(rest);
    let Some(type_dir) = inner
        .components()
        .next()
        .and_then(|c| c.as_os_str().to_str())
    else {
        return false;
    };
    let Some(candidates) = stem_map.get(type_dir) else {
        return false;
    };
    let src_rel = methods_root
        .as_ref()
        .and_then(|root| candidates.get(root))
        .or_else(|| candidates.values().next());
    let Some(src_crate_rel) = src_rel else {
        return false;
    };
    folders
        .iter()
        .any(|folder| prefix.join(src_crate_rel).starts_with(folder))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::collections::HashMap;
    use std::path::Path;
    use std::path::PathBuf;

    use super::file_in_boundary;
    use crate::Project;

    #[test]
    fn test_usage() {
        let project = Project::default();
        let mut stem_map: HashMap<String, BTreeMap<String, PathBuf>> = HashMap::new();
        stem_map.insert(
            "session".to_string(),
            BTreeMap::from([("src".to_string(), PathBuf::from("src/session/session.rs"))]),
        );
        assert!(file_in_boundary(
            &project,
            Path::new("methods/session/expiry.rs"),
            Path::new(""),
            &[PathBuf::from("src/session")],
            &stem_map,
        ));
        assert!(!file_in_boundary(
            &project,
            Path::new("methods/session/expiry.rs"),
            Path::new(""),
            &[PathBuf::from("src/other")],
            &stem_map,
        ));
        assert!(file_in_boundary(
            &project,
            Path::new("src/session/session.rs"),
            Path::new(""),
            &[PathBuf::from("src/session")],
            &stem_map,
        ));
    }
}
