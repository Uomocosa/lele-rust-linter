use std::path::PathBuf;

use crate::CollectionWrapperEntry;

pub(crate) fn deck_collection_wrapper_entry(reason: &str) -> CollectionWrapperEntry {
    CollectionWrapperEntry {
        type_name: "Deck".to_string(),
        file: PathBuf::from("src/cards/deck.rs"),
        reason: reason.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::deck_collection_wrapper_entry;

    #[test]
    fn test_usage() {
        assert_eq!(deck_collection_wrapper_entry("cap").type_name, "Deck");
    }
}
