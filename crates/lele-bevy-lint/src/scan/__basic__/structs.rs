use std::collections::BTreeSet;

use crate::scan;

pub struct FoundVisual {
    pub visual: String,
    pub line: usize,
    pub owner: String,
}

pub struct Restyle {
    pub system: String,
    pub line: usize,
}

pub struct Row {
    pub file: String,
    pub markers: String,
    pub visual: String,
    pub anim: String,
    pub kinds: BTreeSet<scan::PreviewKind>,
}
