use std::collections::BTreeSet;

use crate::Config;
use crate::Error;
use crate::Linter;
use crate::RelaxedRule;
use crate::RuleLevel;

pub fn apply_rule_settings(
    linter: &mut Linter,
    config: &Config,
) -> Result<Vec<RelaxedRule>, Error> {
    let owned = linter
        .rules
        .iter()
        .map(|rule| rule.namespace().to_owned())
        .collect::<BTreeSet<_>>();
    let Some(section) = config.as_ref() else {
        return Ok(Vec::new());
    };
    let mut relaxed = Vec::new();
    let mut removed = BTreeSet::new();
    for (namespace, tables) in &section.rules {
        if !owned.contains(namespace.as_str()) {
            continue;
        }
        for (name, table) in tables {
            let found = linter
                .rules
                .iter()
                .find(|rule| rule.name() == name && rule.namespace() == namespace);
            let Some(rule) = found else {
                return Err(Error::UnknownRule {
                    namespace: namespace.clone(),
                    name: name.clone(),
                });
            };
            if !matches!(table.level, Some(RuleLevel::Off)) {
                continue;
            }
            if rule.is_locked() {
                return Err(Error::LockedRuleTurnedOff {
                    name: name.clone(),
                    code: rule.code().to_owned(),
                });
            }
            let has_reason = table
                .reason
                .as_ref()
                .is_some_and(|reason| !reason.trim().is_empty());
            if !has_reason {
                return Err(Error::SettingWithoutReason { name: name.clone() });
            }
            removed.insert((namespace.clone(), name.clone()));
            relaxed.push(RelaxedRule {
                name: name.clone(),
                code: rule.code().to_owned(),
                reason: table.reason.clone().unwrap_or_default(),
            });
        }
    }
    linter
        .rules
        .retain(|rule| !removed.contains(&(rule.namespace().to_owned(), rule.name().to_owned())));
    Ok(relaxed)
}

#[cfg(test)]
mod tests {
    use super::apply_rule_settings;
    use crate::Config;
    use crate::Error;
    use crate::LeleSection;
    use crate::Linter;

    #[test]
    fn test_usage() {
        let mut linter = Linter::default();
        let before = linter.rules.len();
        let section: LeleSection = toml::from_str(
            "[rules.lint.domain_import]\nlevel = \"off\"\nreason = \"legacy file\"\n",
        )
        .unwrap();
        let relaxed = apply_rule_settings(&mut linter, &Config(Some(section))).unwrap();
        assert_eq!(relaxed.len(), 1);
        assert_eq!(relaxed[0].name, "domain_import");
        assert_eq!(relaxed[0].reason, "legacy file");
        assert_eq!(linter.rules.len(), before - 1);
        assert!(linter
            .rules
            .iter()
            .all(|rule| rule.name() != "domain_import"));
    }

    #[test]
    fn off_without_reason_is_rejected() {
        let mut linter = Linter::default();
        let section: LeleSection =
            toml::from_str("[rules.lint.domain_import]\nlevel = \"off\"\n").unwrap();
        let error = apply_rule_settings(&mut linter, &Config(Some(section))).unwrap_err();
        assert!(matches!(error, Error::SettingWithoutReason { .. }));
    }

    #[test]
    fn off_on_locked_rule_is_rejected() {
        let mut linter = Linter::default();
        let section: LeleSection = toml::from_str(
            "[rules.lint.no_comments]\nlevel = \"off\"\nreason = \"want comments\"\n",
        )
        .unwrap();
        let error = apply_rule_settings(&mut linter, &Config(Some(section))).unwrap_err();
        assert!(matches!(error, Error::LockedRuleTurnedOff { .. }));
    }

    #[test]
    fn unknown_name_in_owned_namespace_is_rejected() {
        let mut linter = Linter::default();
        let section: LeleSection =
            toml::from_str("[rules.lint.no_such_rule]\nlevel = \"off\"\nreason = \"typo\"\n")
                .unwrap();
        let error = apply_rule_settings(&mut linter, &Config(Some(section))).unwrap_err();
        assert!(matches!(error, Error::UnknownRule { .. }));
    }

    #[test]
    fn unowned_namespace_is_ignored() {
        let mut linter = Linter::default();
        let before = linter.rules.len();
        let section: LeleSection =
            toml::from_str("[rules.bevy.bevy_ui]\nlevel = \"off\"\nreason = \"no ui\"\n").unwrap();
        let relaxed = apply_rule_settings(&mut linter, &Config(Some(section))).unwrap();
        assert_eq!(relaxed.len(), 0);
        assert_eq!(linter.rules.len(), before);
    }

    #[test]
    fn deny_level_keeps_the_rule() {
        let mut linter = Linter::default();
        let before = linter.rules.len();
        let section: LeleSection =
            toml::from_str("[rules.lint.domain_import]\nlevel = \"deny\"\n").unwrap();
        let relaxed = apply_rule_settings(&mut linter, &Config(Some(section))).unwrap();
        assert_eq!(relaxed.len(), 0);
        assert_eq!(linter.rules.len(), before);
    }
}
