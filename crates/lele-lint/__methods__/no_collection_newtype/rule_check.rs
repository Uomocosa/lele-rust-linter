use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn rule_check(
    _self: &rules::no_collection_newtype::NoCollectionNewtype,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for source in project.content_sources() {
        let file_path = project.absolute_path(source.origin, source.relative_path);
        violations.extend(source.scan_struct_items(&file_path, |struct_def, path| {
            rules::no_collection_newtype::NoCollectionNewtype::check_struct_for_project(
                struct_def,
                path,
                project,
                &source.file.items,
            )
        }));
    }

    violations.extend(rules::no_collection_newtype::NoCollectionNewtype::entry_violations(project));
    violations
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use std::path::Path;

    use syn::ItemStruct;

    fn parse(code: &str) -> ItemStruct {
        syn::parse_str(code).unwrap()
    }

    #[test]
    fn test_usage() {
        let s = parse("pub struct ExampleNames(pub Vec<String>);");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("Vec<ExampleName>"));
    }

    #[test]
    fn test_usage_singular_newtype_passes() {
        let s = parse("pub struct ExampleName(pub String);");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_map_newtype_is_rejected() {
        let s = parse("pub struct Scores(pub HashMap<String, u32>);");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_usage_multi_field_vec_passes() {
        let s = parse("pub struct Player { pub names: Vec<String> }");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_serde_exempt() {
        let s = parse("#[derive(Deserialize)] pub struct Tags(pub Vec<String>);");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_singular_names() {
        assert_eq!(
            rules::no_collection_newtype::NoCollectionNewtype::singular_name("ExampleNames"),
            "ExampleName"
        );
        assert_eq!(
            rules::no_collection_newtype::NoCollectionNewtype::singular_name("Tags"),
            "Tag"
        );
        assert_eq!(
            rules::no_collection_newtype::NoCollectionNewtype::singular_name("Entries"),
            "Entry"
        );
        assert_eq!(
            rules::no_collection_newtype::NoCollectionNewtype::singular_name("Score"),
            "ScoreItem"
        );
    }

    #[test]
    fn test_usage_borrowed_slice_rejected() {
        let s = parse("pub struct ExampleNames<'a>(pub &'a [String]);");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_usage_mut_slice_rejected() {
        let s = parse("pub struct ExampleNames<'a>(pub &'a mut [String]);");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_usage_boxed_slice_rejected() {
        let s = parse("pub struct ExampleNames(pub Box<[String]>);");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_usage_unsigned_elements_exempt() {
        for code in [
            "pub struct Bytes(pub Vec<u8>);",
            "pub struct Counts(pub Vec<usize>);",
            "pub struct Words(pub Vec<u32>);",
        ] {
            let s = parse(code);
            let violations = rules::no_collection_newtype::NoCollectionNewtype::check_struct(
                &s,
                Path::new("x.rs"),
            );
            assert!(violations.is_empty());
        }
    }

    #[test]
    fn test_usage_unsigned_slice_exempt() {
        let s = parse("pub struct Bytes<'a>(pub &'a [u8]);");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_relationship_target_exempt() {
        let s = parse(
            "#[relationship_target(relationship = InRoom)] pub struct RoomMembers(pub Vec<Entity>);",
        );
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_plain_entity_collection_is_violation() {
        let s = parse("pub struct Members(pub Vec<Entity>);");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_usage_resource_derive_exempts_collection() {
        let s = parse(
            "#[derive(Resource, Serialize, Deserialize)] pub struct PeerIndex(pub std::collections::HashMap<String, u32>);",
        );
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_component_derive_disables_serde_exemption() {
        let s = parse("#[derive(Component, Serialize)] pub struct Members(pub Vec<Entity>);");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_usage_resource_wrapping_vec_passes() {
        let s = parse("#[derive(Resource)] pub struct PeerNames(pub Vec<String>);");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_pathed_resource_wrapping_vec_passes() {
        let s = parse("#[derive(bevy::prelude::Resource)] pub struct PeerNames(pub Vec<String>);");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_component_wrapping_vec_is_violation() {
        let s = parse("#[derive(Component)] pub struct Members(pub Vec<String>);");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_usage_message_wrapping_vec_is_violation() {
        let s = parse("#[derive(Message)] pub struct ChatLog(pub Vec<String>);");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert_eq!(violations.len(), 1);
    }

    use crate::CollectionWrapperEntry;
    use crate::Project;
    use std::path::PathBuf;

    const DECK_FILE: &str = "src/cards/deck.rs";
    const WELL_FORMED_DECK: &str = "#[derive(Deref)] pub struct Deck(Vec<Card>); impl TryFrom<Vec<Card>> for Deck { type Error = Vec<Card>; fn try_from(cards: Vec<Card>) -> Result<Self, Self::Error> { Ok(Self(cards)) } }";

    fn deck_entry(reason: &str) -> CollectionWrapperEntry {
        CollectionWrapperEntry {
            type_name: "Deck".to_string(),
            file: PathBuf::from(DECK_FILE),
            reason: reason.to_string(),
        }
    }

    fn project_with(
        files: &[(&str, &str)],
        entries: Vec<CollectionWrapperEntry>,
    ) -> (tempfile::TempDir, Project) {
        let dir = tempfile::tempdir().unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            src_dir: dir.path().join("src"),
            collection_wrappers: entries,
            ..Project::default()
        };
        for (rel, source) in files {
            let path = dir.path().join("src").join(rel);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, source).unwrap();
            project
                .parsed_files
                .insert(PathBuf::from(rel), syn::parse_str(source).unwrap());
        }
        (dir, project)
    }

    fn messages_for(files: &[(&str, &str)], entries: Vec<CollectionWrapperEntry>) -> Vec<String> {
        let (_dir, project) = project_with(files, entries);
        super::rule_check(&rules::no_collection_newtype::NoCollectionNewtype, &project)
            .into_iter()
            .map(|violation| violation.message)
            .collect()
    }

    #[test]
    fn test_listing_unlisted_wrapper_asks_the_user() {
        let messages = messages_for(&[("cards/deck.rs", "pub struct Deck(Vec<Card>);")], vec![]);
        assert_eq!(messages.len(), 1);
        assert!(
            messages[0].contains("ask the user for a `[[lele.lint.collection_wrappers]]` entry")
        );
    }

    #[test]
    fn test_listing_well_formed_wrapper_is_clean() {
        let messages = messages_for(
            &[("cards/deck.rs", WELL_FORMED_DECK)],
            vec![deck_entry("at most 52 cards")],
        );
        assert_eq!(messages, Vec::<String>::new());
    }

    #[test]
    fn test_listing_public_field_is_rejected() {
        let messages = messages_for(
            &[(
                "cards/deck.rs",
                "pub struct Deck(pub Vec<Card>); impl Deck { pub fn len(&self) -> usize { 0 } }",
            )],
            vec![deck_entry("cap")],
        );
        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("its field is public"));
    }

    #[test]
    fn test_listing_deref_mut_is_rejected() {
        let messages = messages_for(
            &[("cards/deck.rs", "#[derive(Deref, DerefMut)] pub struct Deck(Vec<Card>); impl Deck { pub fn len(&self) -> usize { 0 } }")],
            vec![deck_entry("cap")],
        );
        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("derives `DerefMut`"));
    }

    #[test]
    fn test_listing_without_impl_is_rejected() {
        let messages = messages_for(
            &[("cards/deck.rs", "pub struct Deck(Vec<Card>);")],
            vec![deck_entry("cap")],
        );
        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("has no impl that enforces its invariant"));
    }

    #[test]
    fn test_listing_impl_for_another_type_does_not_count() {
        let messages = messages_for(
            &[("cards/deck.rs", "pub struct Deck(Vec<Card>); impl From<Deck> for Vec<Card> { fn from(deck: Deck) -> Self { Vec::new() } }")],
            vec![deck_entry("cap")],
        );
        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("has no impl that enforces its invariant"));
    }

    #[test]
    fn test_listing_inherent_impl_with_fn_counts() {
        let messages = messages_for(
            &[("cards/deck.rs", "#[derive(Deref)] pub struct Deck(Vec<Card>); impl Deck { pub fn len(&self) -> usize { 0 } }")],
            vec![deck_entry("cap")],
        );
        assert_eq!(messages, Vec::<String>::new());
    }

    #[test]
    fn test_listing_stale_entry_is_reported() {
        let messages = messages_for(
            &[("cards/deck.rs", "pub struct Hand(Vec<Card>);")],
            vec![deck_entry("cap")],
        );
        assert!(messages.iter().any(|message| message.contains(
            "stale [[lele.lint.collection_wrappers]] entry: no collection wrapper `Deck` in `src/cards/deck.rs`"
        )));
    }

    #[test]
    fn test_listing_entry_for_missing_file_is_stale() {
        let messages = messages_for(&[], vec![deck_entry("cap")]);
        assert_eq!(messages.len(), 1, "{messages:?}");
        assert!(messages[0].contains("file `src/cards/deck.rs` does not exist"));
    }

    #[test]
    fn test_listing_empty_reason_is_reported() {
        let messages = messages_for(
            &[("cards/deck.rs", WELL_FORMED_DECK)],
            vec![deck_entry("  ")],
        );
        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("entry for `Deck` has no reason"));
    }

    #[test]
    fn test_listing_wrong_file_is_unlisted_and_stale() {
        let messages = messages_for(
            &[
                ("cards/deck.rs", "pub struct Hand(Vec<Card>);"),
                ("cards/hand.rs", WELL_FORMED_DECK),
            ],
            vec![deck_entry("cap")],
        );
        assert!(messages
            .iter()
            .any(|message| message.contains("ask the user")));
        assert!(messages.iter().any(|message| message.contains("stale")));
    }

    #[test]
    fn test_listing_exempt_resource_is_clean() {
        let messages = messages_for(
            &[(
                "cards/deck.rs",
                "#[derive(Resource)] pub struct Deck(pub Vec<Card>);",
            )],
            vec![deck_entry("cap")],
        );
        assert_eq!(messages, Vec::<String>::new());
    }
}
