use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

pub fn file_in_boundary(
    crate_rel: &Path,
    prefix: &Path,
    folders: &[PathBuf],
    stem_map: &HashMap<String, PathBuf>,
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
    let Some(type_dir) = rest
        .components()
        .next()
        .and_then(|c| c.as_os_str().to_str())
    else {
        return false;
    };
    let Some(src_rel) = stem_map.get(type_dir) else {
        return false;
    };
    let src_crate_rel = Path::new("src").join(src_rel);
    folders
        .iter()
        .any(|folder| prefix.join(&src_crate_rel).starts_with(folder))
}

// no test_usage necessary
