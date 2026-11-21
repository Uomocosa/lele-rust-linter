use std::path::Path;

use crate::rules;
use crate::CodeDunderRole;
use crate::DirFacts;
use crate::Origin;
use crate::Project;
use crate::TestingRole;

pub fn dir_facts(project: &Project, origin: Origin, rel_path: &Path) -> DirFacts {
    let none = DirFacts {
        origin,
        testing_role: None,
        code_dunder: None,
        is_in_container: false,
        is_in_tests: false,
        is_in_flat_dir: false,
    };
    match origin {
        Origin::Methods | Origin::Examples => return none,
        Origin::Src => {}
    }
    let rel_crate = rules::flat_dir::FlatDir::crate_relative_path(project, rel_path);
    let mut is_in_flat_dir = false;
    for flat in &project.flat_dirs {
        let flat_rel = rules::flat_dir::FlatDir::crate_relative_flat_dir(project, flat);
        if rel_crate.starts_with(&flat_rel) && rel_crate != flat_rel {
            is_in_flat_dir = true;
            break;
        }
    }
    let dirs: Vec<String> = rel_path
        .components()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect();
    let dirs = dirs.get(..dirs.len().saturating_sub(1)).unwrap_or(&[]);
    let testing_role = if dirs.first().is_some_and(|first| first == "testing") {
        match dirs.get(1).map(String::as_str) {
            Some("__app__") => Some(TestingRole::App),
            Some("__queries__") => Some(TestingRole::Queries),
            Some("__fixtures__") => Some(TestingRole::Fixtures),
            Some("__fakes__") => Some(TestingRole::Fakes),
            Some("__basic__") => Some(TestingRole::Basic),
            _ => Some(TestingRole::Root),
        }
    } else {
        None
    };
    let is_in_tests = dirs.contains(&"tests".to_string());
    let mut code_dunder = None;
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
            continue;
        }
        if dunder.module_name() == "traits" {
            code_dunder = Some(CodeDunderRole::Traits);
            break;
        }
        if dunder.module_name() == "bevy_systems" {
            code_dunder = Some(CodeDunderRole::BevySystems);
            break;
        }
    }
    let is_in_container = project.is_in_dunder_dir(rel_path);
    DirFacts {
        origin,
        testing_role,
        code_dunder,
        is_in_container,
        is_in_tests,
        is_in_flat_dir,
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    use rstest::rstest;

    use super::dir_facts;
    use crate::CodeDunderRole;
    use crate::DirFacts;
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
            dir_facts(&project, Origin::Src, Path::new("stock/item.rs")),
            DirFacts {
                origin: Origin::Src,
                testing_role: None,
                code_dunder: None,
                is_in_container: false,
                is_in_tests: false,
                is_in_flat_dir: false,
            }
        );
        assert_eq!(
            dir_facts(&project, Origin::Src, Path::new("stock/__basic__/enums.rs")),
            DirFacts {
                origin: Origin::Src,
                testing_role: None,
                code_dunder: None,
                is_in_container: true,
                is_in_tests: false,
                is_in_flat_dir: false,
            }
        );
    }

    #[rstest]
    #[case(Origin::Src, "lib.rs", None, None, false, false, false)]
    #[case(Origin::Src, "stock/item.rs", None, None, false, false, false)]
    #[case(
        Origin::Src,
        "stock/__basic__/enums.rs",
        None,
        None,
        true,
        false,
        false
    )]
    #[case(
        Origin::Src,
        "stock/__traits__/greetable.rs",
        None,
        Some(CodeDunderRole::Traits),
        false,
        false,
        false
    )]
    #[case(
        Origin::Src,
        "stock/__bevy_systems__/poll.rs",
        None,
        Some(CodeDunderRole::BevySystems),
        false,
        false,
        false
    )]
    #[case(
        Origin::Src,
        "stock/__bevy_systems__/__basic__/x.rs",
        None,
        Some(CodeDunderRole::BevySystems),
        true,
        false,
        false
    )]
    #[case(
        Origin::Src,
        "stock/__traits__/__basic__/x.rs",
        None,
        Some(CodeDunderRole::Traits),
        true,
        false,
        false
    )]
    #[case(
        Origin::Src,
        "testing/__app__/__bevy_systems__/x.rs",
        Some(TestingRole::App),
        Some(CodeDunderRole::BevySystems),
        false,
        false,
        false
    )]
    #[case(
        Origin::Src,
        "testing/__app__/__basic__/x.rs",
        Some(TestingRole::App),
        None,
        true,
        false,
        false
    )]
    #[case(
        Origin::Src,
        "testing/mod.rs",
        Some(TestingRole::Root),
        None,
        false,
        false,
        false
    )]
    #[case(
        Origin::Src,
        "testing/__fixtures__/cards.rs",
        Some(TestingRole::Fixtures),
        None,
        false,
        false,
        false
    )]
    #[case(
        Origin::Src,
        "testing/__basic__/constants.rs",
        Some(TestingRole::Basic),
        None,
        true,
        false,
        false
    )]
    #[case(
        Origin::Src,
        "testing/__app__/sub/x.rs",
        Some(TestingRole::App),
        None,
        false,
        false,
        false
    )]
    #[case(Origin::Src, "foo/testing/x.rs", None, None, false, false, false)]
    #[case(Origin::Src, "foo/tests/helper.rs", None, None, false, true, false)]
    #[case(Origin::Src, "bevy_systems/x.rs", None, None, false, false, false)]
    #[case(Origin::Src, "rules_flat/item.rs", None, None, false, false, true)]
    #[case(Origin::Methods, "stock/increment.rs", None, None, false, false, false)]
    #[case(Origin::Examples, "demo.rs", None, None, false, false, false)]
    fn dir_facts_reports_independent_facts(
        #[case] origin: Origin,
        #[case] rel: &str,
        #[case] testing_role: Option<TestingRole>,
        #[case] code_dunder: Option<CodeDunderRole>,
        #[case] is_in_container: bool,
        #[case] is_in_tests: bool,
        #[case] is_in_flat_dir: bool,
    ) {
        let project = Project {
            dunder_paths: DunderPath::defaults(),
            flat_dirs: vec![FlatDirEntry {
                path: PathBuf::from("src/rules_flat"),
                reason: "one file per item".to_string(),
            }],
            ..Project::default()
        };
        assert_eq!(
            dir_facts(&project, origin, Path::new(rel)),
            DirFacts {
                origin,
                testing_role,
                code_dunder,
                is_in_container,
                is_in_tests,
                is_in_flat_dir,
            }
        );
    }
}
