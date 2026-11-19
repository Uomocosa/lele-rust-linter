use crate::rules;

pub(crate) struct PrimaryItem {
    pub(crate) kind: rules::test_usage_shape::PrimaryKind,
    pub(crate) name: String,
    pub(crate) method_names: Vec<String>,
}
