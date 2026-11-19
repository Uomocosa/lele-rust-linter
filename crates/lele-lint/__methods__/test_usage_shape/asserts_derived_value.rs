use std::collections::BTreeSet;

use rules::test_usage_shape::PrimaryKind;

use crate::rules;

pub fn asserts_derived_value(
    _usage: &syn::ItemFn,
    calls: &rules::test_usage_shape::TestUsageCalls,
    primary: &rules::test_usage_shape::PrimaryItem,
) -> bool {
    let installers = ["add_plugin", "add_plugins", "with_plugin"];
    let method_names: Vec<&str> = primary.method_names.iter().map(String::as_str).collect();
    let mut produced: BTreeSet<String> = BTreeSet::new();
    produced.insert(primary.name.clone());
    produced.extend(primary.method_names.iter().cloned());

    let matches_segment = |segment: &str| {
        segment == primary.name.as_str()
            || (matches!(primary.kind, PrimaryKind::Plugin) && installers.contains(&segment))
    };
    let matches_method = |name: &str| match primary.kind {
        PrimaryKind::Method => name == primary.name.as_str(),
        PrimaryKind::Plugin => installers.contains(&name),
        PrimaryKind::Struct | PrimaryKind::Enum => method_names.contains(&name),
        PrimaryKind::Fn => false,
    };

    for (segments, idents) in calls.call_paths.iter().zip(&calls.call_idents) {
        if segments.iter().any(|segment| matches_segment(segment)) {
            produced.extend(idents.iter().cloned());
        }
    }
    for (name, idents) in calls.method_names.iter().zip(&calls.method_idents) {
        if matches_method(name) {
            produced.extend(idents.iter().cloned());
        }
    }

    loop {
        let mut added = false;
        for (bindings, scan) in &calls.locals {
            let from_call = rules::test_usage_shape::TestUsageShape::calls_primary(scan, primary);
            let from_chain = scan.idents.iter().any(|ident| produced.contains(ident));
            if !from_call && !from_chain {
                continue;
            }
            for binding in bindings {
                added |= produced.insert(binding.clone());
            }
            if from_call {
                for ident in &scan.idents {
                    added |= produced.insert(ident.clone());
                }
            }
        }
        if !added {
            break;
        }
    }

    calls
        .asserts
        .iter()
        .any(|idents| idents.iter().any(|ident| produced.contains(ident)))
}

#[cfg(test)]
mod tests {
    use super::asserts_derived_value;
    use crate::rules;
    use crate::testing;

    #[test]
    fn test_usage() {
        let file = testing::parsed_file(
            "#[test] fn test_usage() { let guest = Guest(\"Ada\".into()); assert_eq!(guest.greet(), \"Hello\"); }",
        );
        let Some(syn::Item::Fn(function)) = file.items.into_iter().next() else {
            panic!("expected a fn");
        };
        let calls = rules::test_usage_shape::TestUsageCalls::from(&function);
        let primary = rules::test_usage_shape::PrimaryItem {
            kind: rules::test_usage_shape::PrimaryKind::Struct,
            name: "Guest".to_string(),
            method_names: vec!["greet".to_string()],
        };
        assert!(asserts_derived_value(&function, &calls, &primary));
        assert!(
            rules::test_usage_shape::TestUsageShape::asserts_derived_value(
                &function, &calls, &primary
            )
        );
    }
}
