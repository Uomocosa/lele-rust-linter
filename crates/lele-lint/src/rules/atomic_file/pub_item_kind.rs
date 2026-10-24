pub(crate) enum PubItemKind {
    Struct,
    Enum,
    Fn,
}

#[rustfmt::skip]
impl PubItemKind {
    pub(crate) fn kind_str(&self) -> &'static str {
        match self {
            PubItemKind::Struct => "struct",
            PubItemKind::Enum => "enum",
            PubItemKind::Fn => "fn",
        }
    }
}

// no test_usage necessary
