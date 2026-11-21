use std::path::Path;

use crate::rules;
use crate::CodeDunderRole;
use crate::Origin;
use crate::Project;

pub fn needs_no_test_usage(
    origin: Origin,
    rel_path: &Path,
    file: &syn::File,
    project: &Project,
) -> bool {
    if rules::test_usage::TestUsage::is_forbidden_file(rel_path, file) {
        return true;
    }
    project.dir_facts(origin, rel_path).code_dunder == Some(CodeDunderRole::BevySystems)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::needs_no_test_usage;
    use crate::DunderPath;
    use crate::Origin;
    use crate::Project;

    #[test]
    fn test_usage() {
        let project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        let system: syn::File = syn::parse_str("pub fn tick() {}").unwrap();
        assert!(needs_no_test_usage(
            Origin::Src,
            Path::new("enemy/__bevy_systems__/tick.rs"),
            &system,
            &project
        ));
        assert!(needs_no_test_usage(
            Origin::Src,
            Path::new("testing/__app__/__bevy_systems__/x.rs"),
            &system,
            &project
        ));
        assert!(!needs_no_test_usage(
            Origin::Src,
            Path::new("enemy/bevy_systems/tick.rs"),
            &system,
            &project
        ));
        assert!(!needs_no_test_usage(
            Origin::Src,
            Path::new("enemy/tick.rs"),
            &system,
            &project
        ));
        let plain: syn::File = syn::parse_quote! {
            pub struct Settings { pub port: u16 }
        };
        assert!(needs_no_test_usage(
            Origin::Src,
            Path::new("settings.rs"),
            &plain,
            &project
        ));
    }
}
