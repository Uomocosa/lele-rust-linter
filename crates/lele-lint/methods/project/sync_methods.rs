use std::collections::BTreeMap;

use crate::common;
use crate::Error;
use crate::Project;

pub fn sync_methods(project: &Project) -> Result<(), Error> {
    let Some(methods_dir) = &project.methods_dir else {
        return Err(Error::NoMethodsDir);
    };
    let declared = project.collect_declared();
    if !project.is_multi_root() {
        Project::write_if_changed(
            &methods_dir.join("mod.rs"),
            &common::DeclaredType::root_index_content(&declared),
        )?;
        for (type_snake, declared_type) in &declared {
            let dir = methods_dir.join(type_snake);
            std::fs::create_dir_all(&dir)?;
            Project::write_if_changed(&dir.join("mod.rs"), &declared_type.index_content())?;
        }
        return Ok(());
    }
    let root_names = project.layout_root_names();
    Project::write_if_changed(
        &methods_dir.join("mod.rs"),
        &common::DeclaredType::glue_index_content(&root_names),
    )?;
    for root in &root_names {
        let dir = methods_dir.join(root);
        std::fs::create_dir_all(&dir)?;
        let in_root: BTreeMap<String, common::DeclaredType> = declared
            .iter()
            .filter(|(type_snake, _)| !project.methods_in_root(type_snake, root).is_empty())
            .map(|(type_snake, declared_type)| (type_snake.clone(), declared_type.clone()))
            .collect();
        Project::write_if_changed(
            &dir.join("mod.rs"),
            &common::DeclaredType::root_index_content(&in_root),
        )?;
        for (type_snake, declared_type) in &in_root {
            let type_dir = dir.join(type_snake);
            std::fs::create_dir_all(&type_dir)?;
            let methods = project.methods_in_root(type_snake, root);
            let scoped = common::DeclaredType {
                methods,
                cfgs: declared_type.cfgs.clone(),
            };
            Project::write_if_changed(&type_dir.join("mod.rs"), &scoped.index_content())?;
        }
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
