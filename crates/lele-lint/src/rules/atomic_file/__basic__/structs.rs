use crate::rules;

pub(crate) struct PubItem {
    pub(crate) name: String,
    pub(crate) kind: rules::atomic_file::PubItemKind,
}

// no test_usage necessary
