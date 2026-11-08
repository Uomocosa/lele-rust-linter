use crate::Config;
use crate::Entry;
use crate::Error;
use crate::ModuleInfo;
use crate::Project;

pub fn apply_layout(project: &mut Project, config: &Config) -> Result<(), Error> {
    project.dunder_paths = config.dunder_paths();
    if let Some(section) = config.as_ref() {
        project
            .clippy_allow_whitelist
            .clone_from(&section.lint.clippy_allow_whitelist);
        project.entry_points.clone_from(&section.lint.entry_points);
        project.boundaries.clone_from(&section.boundary);
    }

    let roots = config.layout_roots();
    project.layout_roots.clone_from(&roots);
    for root_rel in &roots {
        let abs = project.root.join(root_rel);
        if !abs.is_dir() {
            return Err(Error::NoScanFolder(abs));
        }
    }
    for extra_rel in roots.iter().skip(1) {
        let abs_dir = project.root.join(extra_rel);
        if abs_dir == project.src_dir {
            continue;
        }
        let inner = Entry::walk(&abs_dir, &abs_dir)?;
        let mut prefixed = Vec::new();
        for entry in inner {
            let prefixed_rel = extra_rel.join(&entry.relative_path);
            prefixed.push(Entry {
                relative_path: prefixed_rel,
                absolute_path: entry.absolute_path,
                kind: entry.kind,
            });
        }
        let extra_info = ModuleInfo::build(&prefixed);
        let (parsed, failures) = Entry::parse_all(&prefixed);
        project.extra_src_dirs.push(abs_dir);
        project.entries.extend(prefixed);
        project.module_info.extend(extra_info);
        project.parsed_files.extend(parsed);
        project.parse_failures.extend(failures);
    }

    let methods_root = config.layout_methods_root();
    let methods_dir = project.root.join(&methods_root);
    if !methods_dir.is_dir() {
        return Ok(());
    }

    let entries = Entry::walk(&methods_dir, &methods_dir)?;
    let (parsed_files, parse_failures) = Entry::parse_all(&entries);
    project.methods_dir = Some(methods_dir);
    project.methods_entries = entries;
    project.methods_parsed_files = parsed_files;
    project.parse_failures.extend(parse_failures);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::apply_layout;
    use crate::Config;
    use crate::LeleSection;
    use crate::Project;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("src")).unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            src_dir: dir.path().join("src"),
            ..Project::default()
        };
        apply_layout(&mut project, &Config::default()).unwrap();
        assert!(project.methods_dir.is_none());
        std::fs::create_dir_all(dir.path().join("methods")).unwrap();
        std::fs::write(
            dir.path().join("methods").join("probe.rs"),
            "pub fn probe() {}\n",
        )
        .unwrap();
        apply_layout(&mut project, &Config::default()).unwrap();
        assert!(project.methods_dir.is_some());
    }

    #[test]
    fn test_usage_scans_extra_root_with_prefixed_rel() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("src")).unwrap();
        std::fs::create_dir_all(dir.path().join("ui")).unwrap();
        std::fs::write(dir.path().join("src").join("lib.rs"), "pub mod ui;\n").unwrap();
        std::fs::write(
            dir.path().join("ui").join("discovery.rs"),
            "pub struct Discovery;\n#[cfg(test)] mod tests { #[test] fn test_usage() {} }\n",
        )
        .unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            src_dir: dir.path().join("src"),
            ..Project::default()
        };
        let section: LeleSection = toml::from_str("[layout]\nroots = [\"src\", \"ui\"]\n").unwrap();
        apply_layout(&mut project, &Config(Some(section))).unwrap();
        assert!(project
            .parsed_files
            .contains_key(&std::path::PathBuf::from("ui/discovery.rs")));
        assert_eq!(project.extra_src_dirs.len(), 1);
    }

    #[test]
    fn test_usage_custom_methods_root_is_used() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("src")).unwrap();
        std::fs::create_dir_all(dir.path().join("__methods__")).unwrap();
        std::fs::write(
            dir.path().join("__methods__").join("probe.rs"),
            "pub fn probe() {}\n",
        )
        .unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            src_dir: dir.path().join("src"),
            ..Project::default()
        };
        let section: LeleSection =
            toml::from_str("[layout]\nmethods_root = \"__methods__\"\n").unwrap();
        apply_layout(&mut project, &Config(Some(section))).unwrap();
        assert!(project
            .methods_dir
            .is_some_and(|dir| dir.ends_with("__methods__")));
        assert!(project
            .methods_parsed_files
            .contains_key(&std::path::PathBuf::from("probe.rs")));
    }

    #[test]
    fn test_usage_missing_extra_root_errors() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("src")).unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            src_dir: dir.path().join("src"),
            ..Project::default()
        };
        let section: LeleSection =
            toml::from_str("[layout]\nroots = [\"src\", \"missing\"]\n").unwrap();
        assert!(apply_layout(&mut project, &Config(Some(section))).is_err());
    }
}
