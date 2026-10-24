use crate::rules;
use std::path::PathBuf;

pub(crate) struct FileUse {
    pub(crate) file: PathBuf,
    pub(crate) crate_rel: PathBuf,
    pub(crate) hits: Vec<rules::boundary_imports::PathHit>,
}

pub(crate) struct PathHit {
    pub(crate) path: String,
    pub(crate) line: usize,
    pub(crate) glob: bool,
}

// no test_usage necessary
