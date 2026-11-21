use std::path::Path;

use crate::rules;
use crate::DirRole;
use crate::Origin;
use crate::Project;
use crate::TestingRole;

pub fn dir_role(project: &Project, origin: Origin, rel_path: &Path) -> DirRole {
    match origin {
        Origin::Methods => return DirRole::Methods,
        Origin::Examples => return DirRole::Examples,
        Origin::Src => {}
    }
    let rel_crate = rules::flat_dir::FlatDir::crate_relative_path(project, rel_path);
    for flat in &project.flat_dirs {
        let flat_rel = rules::flat_dir::FlatDir::crate_relative_flat_dir(project, flat);
        if rel_crate.starts_with(&flat_rel) && rel_crate != flat_rel {
            return DirRole::Flat;
        }
    }
    let dirs: Vec<String> = rel_path
        .components()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect();
    let dirs = dirs.get(..dirs.len().saturating_sub(1)).unwrap_or(&[]);
    if dirs.first().is_some_and(|first| first == "testing") {
        match dirs.get(1).map(String::as_str) {
            Some("__app__") => return DirRole::Testing(TestingRole::App),
            Some("__queries__") => return DirRole::Testing(TestingRole::Queries),
            Some("__fixtures__") => return DirRole::Testing(TestingRole::Fixtures),
            Some("__fakes__") => return DirRole::Testing(TestingRole::Fakes),
            Some("__basic__") => return DirRole::Testing(TestingRole::Basic),
            _ => return DirRole::Testing(TestingRole::Root),
        }
    }
    if dirs.contains(&"tests".to_string()) {
        return DirRole::Tests;
    }
    for component in dirs.iter().rev() {
        let Some(dunder) = project.dunder_paths.iter().find(|dunder| {
            dunder.is_dunder_dir()
                && dunder
                    .file_name()
                    .is_some_and(|name| name == component.as_str())
        }) else {
            continue;
        };
        if dunder.is_container() {
            return DirRole::Container;
        } else if dunder.module_name() == "traits" {
            return DirRole::Traits;
        } else if dunder.module_name() == "bevy_systems" {
            return DirRole::BevySystems;
        }
    }
    DirRole::Domain
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    use rstest::rstest;

    use super::dir_role;
    use crate::DirRole;
    use crate::DunderPath;
    use crate::FlatDirEntry;
    use crate::Origin;
    use crate::Project;
    use crate::TestingRole;

    #[test]
    fn test_usage() {
        let project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        assert_eq!(
            dir_role(&project, Origin::Src, Path::new("stock/item.rs")),
            DirRole::Domain
        );
        assert_eq!(
            dir_role(&project, Origin::Src, Path::new("stock/__basic__/enums.rs")),
            DirRole::Container
        );
    }

    #[rstest]
    #[case(Origin::Src, "lib.rs", DirRole::Domain)]
    #[case(Origin::Src, "stock/item.rs", DirRole::Domain)]
    #[case(Origin::Src, "stock/__basic__/enums.rs", DirRole::Container)]
    #[case(Origin::Src, "stock/__traits__/greetable.rs", DirRole::Traits)]
    #[case(Origin::Src, "stock/__bevy_systems__/poll.rs", DirRole::BevySystems)]
    #[case(
        Origin::Src,
        "stock/__bevy_systems__/__basic__/x.rs",
        DirRole::Container
    )]
    #[case(Origin::Src, "testing/mod.rs", DirRole::Testing(TestingRole::Root))]
    #[case(
        Origin::Src,
        "testing/__fixtures__/cards.rs",
        DirRole::Testing(TestingRole::Fixtures)
    )]
    #[case(
        Origin::Src,
        "testing/__basic__/constants.rs",
        DirRole::Testing(TestingRole::Basic)
    )]
    #[case(
        Origin::Src,
        "testing/__app__/sub/x.rs",
        DirRole::Testing(TestingRole::App)
    )]
    #[case(Origin::Src, "foo/testing/x.rs", DirRole::Domain)]
    #[case(Origin::Src, "foo/tests/helper.rs", DirRole::Tests)]
    #[case(Origin::Src, "bevy_systems/x.rs", DirRole::Domain)]
    #[case(Origin::Src, "rules_flat/item.rs", DirRole::Flat)]
    #[case(Origin::Methods, "stock/increment.rs", DirRole::Methods)]
    #[case(Origin::Examples, "demo.rs", DirRole::Examples)]
    fn dir_role_classifies_folders(
        #[case] origin: Origin,
        #[case] rel: &str,
        #[case] expected: DirRole,
    ) {
        let project = Project {
            dunder_paths: DunderPath::defaults(),
            flat_dirs: vec![FlatDirEntry {
                path: PathBuf::from("src/rules_flat"),
                reason: "one file per item".to_string(),
            }],
            ..Project::default()
        };
        assert_eq!(dir_role(&project, origin, Path::new(rel)), expected);
    }
}
