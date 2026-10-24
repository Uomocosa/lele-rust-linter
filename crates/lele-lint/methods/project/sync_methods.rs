use crate::common;
use crate::Error;
use crate::Project;

pub fn sync_methods(project: &Project) -> Result<(), Error> {
    let Some(methods_dir) = &project.methods_dir else {
        return Err(Error::NoMethodsDir);
    };
    let declared = project.collect_declared();
    Project::write_if_changed(
        &methods_dir.join("mod.rs"),
        &common::DeclaredType::root_index_content(&declared),
    )?;
    for (type_snake, declared_type) in &declared {
        let dir = methods_dir.join(type_snake);
        std::fs::create_dir_all(&dir)?;
        Project::write_if_changed(&dir.join("mod.rs"), &declared_type.index_content())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Project;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("mod.rs");
        Project::write_if_changed(&path, "pub mod a;\n").unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "pub mod a;\n");
        Project::write_if_changed(&path, "pub mod a;\n").unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "pub mod a;\n");
    }
}
