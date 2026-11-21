use crate::rules;
use crate::DirRole;
use crate::Origin;
use crate::Project;
use crate::TestingRole;
use std::path::Path;

pub fn is_exempt(origin: Origin, rel_path: &Path, file: &syn::File, project: &Project) -> bool {
    let file_name = rel_path.file_name().and_then(|n| n.to_str()).unwrap_or("");

    if file_name == "main.rs" {
        return true;
    }

    if (file_name == "mod.rs" || file_name == "lib.rs")
        && rules::test_usage::TestUsage::is_pure_module_tree(file)
    {
        return true;
    }

    if file_name == "constants.rs" {
        return true;
    }

    if project.is_dunder_file_path(rel_path) {
        return true;
    }

    if matches!(
        project.dir_role(origin, rel_path),
        DirRole::Container | DirRole::Testing(TestingRole::Basic) | DirRole::Tests
    ) {
        return true;
    }

    rules::test_usage::TestUsage::is_type_only(file)
        || rules::test_usage::TestUsage::is_atomic_delegate_only(file)
}

#[cfg(test)]
mod tests {
    use super::is_exempt;
    use crate::DunderPath;
    use crate::Project;
    use std::path::Path;

    #[test]
    fn test_usage() {
        use crate::Origin;

        let project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        let plain: syn::File = syn::parse_str("pub fn poll_input(x: u32) {}").unwrap();
        assert!(!is_exempt(Origin::Src, Path::new("a.rs"), &plain, &project));
        assert!(!is_exempt(
            Origin::Src,
            Path::new("player/__bevy_systems__/poll_input.rs"),
            &plain,
            &project
        ));

        let type_only: syn::File = syn::parse_str("pub struct Bump;").unwrap();
        assert!(is_exempt(
            Origin::Src,
            Path::new("player/__basic__/enums.rs"),
            &type_only,
            &project
        ));
    }
}
