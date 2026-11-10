use crate::common;
use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &rules::no_cross_domain_reexport::NoCrossDomainReexport,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for (mod_rs_path, info) in &project.module_info {
        if common::SourcePath(mod_rs_path).is_crate_root() {
            continue;
        }
        let own_domain =
            rules::no_cross_domain_reexport::NoCrossDomainReexport::own_domain_of(mod_rs_path);

        for reexport in &info.reexports {
            if let Some(target_domain) =
                rules::no_cross_domain_reexport::NoCrossDomainReexport::reexport_target_domain(
                    &reexport.segments,
                )
            {
                if target_domain != own_domain && !target_domain.is_empty() {
                    let reexported_path = reexport.segments.join("::");
                    let module_file = project
                        .entries
                        .iter()
                        .find(|e| e.relative_path == *mod_rs_path)
                        .map(|e| e.absolute_path.clone());

                    violations.push(RuleViolation::from_file(
                        module_file.unwrap_or_else(|| project.root.join("src").join(mod_rs_path)),
                        rules::no_cross_domain_reexport::NoCrossDomainReexport::CODE,
                        format!(
                            "cross-domain re-export `pub use {}` in mod.rs — move it to lib.rs",
                            reexported_path
                        ),
                    ));
                }
            }
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use std::path::Path;

    #[test]
    fn test_usage() {
        assert_eq!(
            rules::no_cross_domain_reexport::NoCrossDomainReexport::reexport_target_domain(&[
                "crate".into(),
                "bad_positional".into(),
                "Positional".into()
            ]),
            Some("bad_positional".into())
        );
        assert_eq!(
            rules::no_cross_domain_reexport::NoCrossDomainReexport::reexport_target_domain(&[
                "config".into(),
                "Config".into()
            ]),
            None
        );
        assert_eq!(
            rules::no_cross_domain_reexport::NoCrossDomainReexport::reexport_target_domain(&[]),
            None
        );
        assert_eq!(
            rules::no_cross_domain_reexport::NoCrossDomainReexport::reexport_target_domain(&[
                "event".into(),
                "PlayerEvent".into()
            ]),
            None
        );
    }

    #[test]
    fn test_usage_own_domain_uses_top_level_segment() {
        assert_eq!(
            rules::no_cross_domain_reexport::NoCrossDomainReexport::own_domain_of(Path::new(
                "discovery/room_peers/mod.rs"
            )),
            "discovery"
        );
        assert_eq!(
            rules::no_cross_domain_reexport::NoCrossDomainReexport::own_domain_of(Path::new(
                "player/mod.rs"
            )),
            "player"
        );
    }
}
