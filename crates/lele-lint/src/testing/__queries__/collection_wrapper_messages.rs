use crate::methods;
use crate::rules;
use crate::testing;
use crate::CollectionWrapperEntry;

pub(crate) fn collection_wrapper_messages(
    files: &[(&str, &str)],
    entries: Vec<CollectionWrapperEntry>,
) -> Vec<String> {
    let (_dir, project) = testing::collection_wrapper_project(files, entries);
    methods::no_collection_newtype::rule_check(
        &rules::no_collection_newtype::NoCollectionNewtype,
        &project,
    )
    .into_iter()
    .map(|violation| violation.message)
    .collect()
}

#[cfg(test)]
mod tests {
    use super::collection_wrapper_messages;

    #[test]
    fn test_usage() {
        let messages = collection_wrapper_messages(
            &[("cards/deck.rs", "pub struct Deck(Vec<Card>);")],
            vec![],
        );
        assert_eq!(messages.len(), 1);
    }
}
