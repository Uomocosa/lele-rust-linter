use rules::test_usage_shape::PrimaryKind;

use crate::rules;

pub fn calls_primary(
    calls: &rules::test_usage_shape::TestUsageCalls,
    primary: &rules::test_usage_shape::PrimaryItem,
) -> bool {
    let installers = ["add_plugin", "add_plugins", "with_plugin"];
    let method_names: Vec<&str> = primary.method_names.iter().map(String::as_str).collect();
    let path_mentions_name =
        |segments: &[String]| segments.iter().any(|segment| segment == &primary.name);
    let any_call_mentions_name = || {
        calls
            .call_paths
            .iter()
            .any(|segments| path_mentions_name(segments))
            || calls
                .struct_paths
                .iter()
                .any(|segments| path_mentions_name(segments))
            || calls
                .path_firsts
                .iter()
                .any(|segment| segment == &primary.name)
    };
    let assert_mentions = |names: &[&str]| {
        calls.asserts.iter().any(|idents| {
            idents
                .iter()
                .any(|ident| ident == &primary.name || names.contains(&ident.as_str()))
        })
    };

    match primary.kind {
        PrimaryKind::Method => {
            calls.method_names.iter().any(|name| name == &primary.name)
                || calls
                    .call_paths
                    .iter()
                    .any(|segments| path_mentions_name(segments))
                || assert_mentions(&[])
        }
        PrimaryKind::Plugin => {
            calls
                .method_names
                .iter()
                .any(|name| installers.contains(&name.as_str()))
                || any_call_mentions_name()
                || calls.call_paths.iter().any(|segments| {
                    segments
                        .iter()
                        .any(|segment| installers.contains(&segment.as_str()))
                })
                || assert_mentions(&installers)
        }
        PrimaryKind::Struct | PrimaryKind::Enum => {
            any_call_mentions_name()
                || calls
                    .method_names
                    .iter()
                    .any(|name| method_names.contains(&name.as_str()))
                || assert_mentions(&method_names)
        }
        PrimaryKind::Fn => any_call_mentions_name() || assert_mentions(&[]),
    }
}

#[cfg(test)]
mod tests {
    use super::calls_primary;
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
        assert!(calls_primary(&calls, &primary));
        assert!(rules::test_usage_shape::TestUsageShape::calls_primary(
            &calls, &primary
        ));
    }
}
