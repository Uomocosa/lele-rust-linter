use crate::Entry;
use crate::Error;
use crate::ModuleInfo;
use crate::Project;
use std::path::{Path, PathBuf};

pub fn try_from_folders(base_path: &Path, folders: &[PathBuf]) -> Result<Project, Error> {
    let mut entries = Vec::new();
    for folder in folders {
        let rel_dir = folder.strip_prefix("/").unwrap_or(folder.as_path());
        let abs_dir = base_path.join(rel_dir);
        if !abs_dir.exists() || !abs_dir.is_dir() {
            return Err(Error::NoScanFolder(abs_dir));
        }
        entries.extend(Entry::walk(&abs_dir, base_path)?);
    }
    let module_info = ModuleInfo::build(&entries);
    let (parsed_files, parse_failures) = Entry::parse_all(&entries);
    let owned_base = base_path.to_path_buf();
    Ok(Project {
        root: owned_base.clone(),
        src_dir: owned_base,
        entries,
        module_info,
        parsed_files,
        parse_failures,
        ..Project::default()
    })
}

// no test_usage necessary
