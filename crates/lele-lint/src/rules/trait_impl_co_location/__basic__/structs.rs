pub(crate) struct ImplHit {
    pub(crate) line: usize,
    pub(crate) trait_name: String,
    pub(crate) type_name: Option<String>,
    pub(crate) is_blanket: bool,
}

// no test_usage necessary
