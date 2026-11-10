use lele_lint::Project;
use lele_lint::RuleViolation;
use lele_lint::rules::dunder_module_name::DunderModuleName;

use crate::rules;

const OWNED_MOUNT: &str = "bevy_systems";

pub fn check(_self: &rules::bevy_export::BevyExport, project: &Project) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for source in project.content_sources() {
        for hit in
            DunderModuleName::dunder_reexport_hits(source.relative_path, source.file, project)
        {
            if hit.mount != OWNED_MOUNT {
                continue;
            }
            violations.push(RuleViolation::from_site(
                project.absolute_path(source.origin, source.relative_path),
                hit.line,
                0,
                rules::bevy_export::BevyExport::CODE,
                format!(
                    "pub use {raw} re-exports bevy_systems items; remove it — access via `{suggestion}`",
                    raw = hit.raw,
                    suggestion = hit.suggestion
                ),
            ));
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::check;
    use lele_lint::DunderPath;
    use lele_lint::Project;
    use std::path::PathBuf;

    use crate::rules::bevy_export::BevyExport;

    fn messages(source: &str) -> Vec<String> {
        let mut project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("inventory/mod.rs"),
            syn::parse_str(source).unwrap(),
        );
        check(&BevyExport, &project)
            .into_iter()
            .map(|violation| violation.message)
            .collect()
    }

    #[test]
    fn test_usage() {
        let found = messages(
            "#[path = \"__bevy_systems__/mod.rs\"]\npub mod bevy_systems;\n\npub use bevy_systems::poll_inv;\n",
        );
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "pub use bevy_systems::poll_inv re-exports bevy_systems items; remove it — access via `inventory::bevy_systems::poll_inv`"
        );
    }

    #[test]
    fn test_usage_ignores_trait_mounts_and_private_uses() {
        assert_eq!(
            messages("pub use traits::Greetable;\nuse bevy_systems::poll_inv;\n"),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_records_mount_partition() {
        assert_eq!(super::OWNED_MOUNT, "bevy_systems");
    }
}
