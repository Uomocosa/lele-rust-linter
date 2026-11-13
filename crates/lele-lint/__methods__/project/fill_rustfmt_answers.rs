use crate::Error;
use crate::Project;
use crate::RustfmtKey;

pub fn fill_rustfmt_answers(project: &mut Project) -> Result<(), Error> {
    let candidates = project.collect_rustfmt_candidates()?;
    if candidates.is_empty() {
        return Ok(());
    }
    let output = project.run_rustfmt(&Project::rustfmt_source(&candidates))?;
    let keys: Vec<RustfmtKey> = candidates.into_iter().map(|(key, _)| key).collect();
    let max_width = project.rustfmt_max_width();
    project
        .rustfmt_one_liners
        .extend(Project::rustfmt_answers(&output, &keys, max_width)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::Error;
    use crate::Project;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let source = "pub struct Foo;\nimpl Foo {\n    fn one(&self) -> u8 { self.0 }\n    fn many(&self) -> Vec<u32> { self.items.iter().map(|item| item.value * 2 + item.offset).filter(|v| *v > 10).map(|v| v * 3).collect::<Vec<u32>>() }\n}\n";
        std::fs::write(dir.path().join("foo.rs"), source).unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            src_dir: dir.path().to_path_buf(),
            ..Project::default()
        };
        project
            .parsed_files
            .insert(PathBuf::from("foo.rs"), syn::parse_str(source).unwrap());
        project.fill_rustfmt_answers().unwrap();
        assert_eq!(project.rustfmt_one_liners.len(), 2);
        assert!(project.rustfmt_one_liners.values().any(|one| *one));
        assert!(project.rustfmt_one_liners.values().any(|one| !*one));
        let mut broken = Project {
            root: PathBuf::from("/nonexistent-lele-root"),
            src_dir: dir.path().to_path_buf(),
            parsed_files: project.parsed_files.clone(),
            ..Project::default()
        };
        assert!(matches!(
            broken.fill_rustfmt_answers(),
            Err(Error::RustfmtStart { .. })
        ));
    }
}
