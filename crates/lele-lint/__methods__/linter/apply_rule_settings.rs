use std::collections::BTreeSet;

use crate::AllowWhitelistEntry;
use crate::CollectionWrapperEntry;
use crate::Config;
use crate::Error;
use crate::GenericExemptEntry;
use crate::Linter;
use crate::NoDelegatesEntry;
use crate::PathExemption;
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
            if !matches!(table.level, Some(RuleLevel::Off)) && table.exempt.is_empty() {
                continue;
            }
            if matches!(table.level, Some(RuleLevel::Off)) {
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
                relaxed.push(RelaxedRule::Off {
                    name: name.clone(),
                    code: rule.code().to_owned(),
                    reason: table.reason.clone().unwrap_or_default(),
                });
            }
            if table.exempt.is_empty() {
                continue;
            }
            if name == "no_allow_attributes" {
                for exempt in &table.exempt {
                    let entry: AllowWhitelistEntry = toml::Value::Table(exempt.clone())
                        .try_into()
                        .map_err(|source| Error::BadRuleExempt {
                            rule: "lint.no_allow_attributes".to_string(),
                            source,
                        })?;
                    relaxed.push(RelaxedRule::Exempt {
                        name: name.clone(),
                        code: rule.code().to_owned(),
                        path: entry.file,
                        reason: entry.reason,
                        replaced_by: None,
                    });
                }
                continue;
            }
            if name == "no_collection_newtype" {
                for exempt in &table.exempt {
                    let entry: CollectionWrapperEntry = toml::Value::Table(exempt.clone())
                        .try_into()
                        .map_err(|source| Error::BadRuleExempt {
                            rule: "lint.no_collection_newtype".to_string(),
                            source,
                        })?;
                    relaxed.push(RelaxedRule::Exempt {
                        name: name.clone(),
                        code: rule.code().to_owned(),
                        path: entry.file,
                        reason: entry.reason,
                        replaced_by: None,
                    });
                }
                continue;
            }
            if name == "atomic_delegates" {
                for exempt in &table.exempt {
                    let entry: NoDelegatesEntry = toml::Value::Table(exempt.clone())
                        .try_into()
                        .map_err(|source| Error::BadRuleExempt {
                            rule: "lint.atomic_delegates".to_string(),
                            source,
                        })?;
                    relaxed.push(RelaxedRule::Exempt {
                        name: name.clone(),
                        code: rule.code().to_owned(),
                        path: entry.crate_path,
                        reason: entry.reason,
                        replaced_by: None,
                    });
                }
                continue;
            }
            for exempt in &table.exempt {
                let entry: GenericExemptEntry = toml::Value::Table(exempt.clone())
                    .try_into()
                    .map_err(|source| Error::BadRuleExempt {
                        rule: format!("{namespace}.{name}"),
                        source,
                    })?;
                if entry.reason.trim().is_empty() {
                    return Err(Error::ExemptWithoutReason { name: name.clone() });
                }
                if let Some(replaced_by) = &entry.replaced_by {
                    let known = linter.rules.iter().any(|rule| rule.name() == replaced_by);
                    if !known {
                        return Err(Error::UnknownReplacement {
                            name: name.clone(),
                            replaced_by: replaced_by.clone(),
                        });
                    }
                }
                linter.exemptions.push(PathExemption {
                    code: rule.code().to_owned(),
                    root: entry.path.clone(),
                    reason: entry.reason.clone(),
                    replaced_by: entry.replaced_by.clone(),
                });
                relaxed.push(RelaxedRule::Exempt {
                    name: name.clone(),
                    code: rule.code().to_owned(),
                    path: entry.path,
                    reason: entry.reason,
                    replaced_by: entry.replaced_by,
                });
            }
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
    use crate::RelaxedRule;

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
        let RelaxedRule::Off { name, reason, .. } = &relaxed[0] else {
            panic!("expected an Off entry");
        };
        assert_eq!(name, "domain_import");
        assert_eq!(reason, "legacy file");
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
    fn unknown_exempt_only_name_is_rejected() {
        let mut linter = Linter::default();
        let section: LeleSection = toml::from_str(
            "[[rules.lint.no_such_rule.exempt]]\npath = \"src/legacy\"\nreason = \"typo\"\n",
        )
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

    #[test]
    fn generic_exempt_pushes_exemption_and_banner_entry() {
        let mut linter = Linter::default();
        let section: LeleSection = toml::from_str(
            "[[rules.lint.no_comments.exempt]]\npath = \"crates/rule-engine/__rules__\"\nreason = \"generated mirror\"\nreplaced_by = \"test_usage\"\n",
        )
        .unwrap();
        let relaxed = apply_rule_settings(&mut linter, &Config(Some(section))).unwrap();
        assert_eq!(relaxed.len(), 1);
        let RelaxedRule::Exempt {
            name,
            code,
            path,
            reason,
            replaced_by,
        } = &relaxed[0]
        else {
            panic!("expected an Exempt entry");
        };
        assert_eq!(name, "no_comments");
        assert_eq!(code, "LELECORE015");
        assert_eq!(
            path,
            &std::path::PathBuf::from("crates/rule-engine/__rules__")
        );
        assert_eq!(reason, "generated mirror");
        assert_eq!(replaced_by.as_deref(), Some("test_usage"));
        assert_eq!(linter.exemptions.len(), 1);
        assert_eq!(linter.exemptions[0].code, "LELECORE015");
    }

    #[test]
    fn exempt_with_unknown_replacement_is_rejected() {
        let mut linter = Linter::default();
        let section: LeleSection = toml::from_str(
            "[[rules.lint.no_comments.exempt]]\npath = \"src/legacy\"\nreason = \"mirror\"\nreplaced_by = \"no_such_rule\"\n",
        )
        .unwrap();
        let error = apply_rule_settings(&mut linter, &Config(Some(section))).unwrap_err();
        assert!(matches!(error, Error::UnknownReplacement { .. }));
    }

    #[test]
    fn exempt_without_reason_is_rejected() {
        let mut linter = Linter::default();
        let section: LeleSection = toml::from_str(
            "[[rules.lint.no_comments.exempt]]\npath = \"src/legacy\"\nreason = \"\"\n",
        )
        .unwrap();
        let error = apply_rule_settings(&mut linter, &Config(Some(section))).unwrap_err();
        assert!(matches!(error, Error::ExemptWithoutReason { .. }));
    }

    #[test]
    fn bad_generic_exempt_table_is_rejected() {
        let mut linter = Linter::default();
        let section: LeleSection = toml::from_str(
            "[[rules.lint.no_comments.exempt]]\npath = \"src/legacy\"\nreason = \"mirror\"\ntypo = 1\n",
        )
        .unwrap();
        let error = apply_rule_settings(&mut linter, &Config(Some(section))).unwrap_err();
        assert!(matches!(error, Error::BadRuleExempt { .. }));
    }

    #[test]
    fn only_flag_with_unrelated_table_succeeds() {
        let mut linter = Linter::default();
        let section: LeleSection =
            toml::from_str("[rules.lint.domain_import]\nlevel = \"off\"\nreason = \"legacy\"\n")
                .unwrap();
        let relaxed = apply_rule_settings(&mut linter, &Config(Some(section))).unwrap();
        assert_eq!(relaxed.len(), 1);
        let unknown = linter.retain_codes(&[String::from("no_comments")]);
        assert_eq!(unknown, Vec::<String>::new());
        assert!(linter.rules.iter().all(|rule| rule.name() == "no_comments"));
    }
}
