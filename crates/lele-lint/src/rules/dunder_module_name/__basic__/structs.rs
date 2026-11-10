pub(crate) struct DunderNameHit {
    pub(crate) line: usize,
    pub(crate) message: String,
}

pub struct ReexportHit {
    pub line: usize,
    pub mount: String,
    pub item: String,
    pub raw: String,
    pub suggestion: String,
}

// no test_usage necessary
