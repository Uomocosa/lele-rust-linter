use crate::rules;
use crate::Project;
use std::path::Path;

pub fn collect_file_uses(project: &Project) -> Vec<rules::boundary_imports::FileUse> {
    let mut files = Vec::new();
    for (rel, parsed) in &project.parsed_files {
        files.push(rules::boundary_imports::BoundaryImports::file_use(
            project.src_dir.join(rel),
            Path::new("src").join(rel),
            parsed,
        ));
    }
    if let Some(methods_dir) = &project.methods_dir {
        for (rel, parsed) in &project.methods_parsed_files {
            files.push(rules::boundary_imports::BoundaryImports::file_use(
                methods_dir.join(rel),
                Path::new("methods").join(rel),
                parsed,
            ));
        }
    }
    files
}

// no test_usage necessary
