use crate::scan::basic::enums::{HarnessCall, PreviewKind};

pub struct FoundVisual {
    pub visual: String,
    pub line: usize,
    pub owner: String,
}

#[derive(Default)]
pub struct SceneUsage {
    pub build_uses: Vec<String>,
    pub state_count: usize,
    pub uncentered_kinds: Vec<String>,
}

pub struct Restyle {
    pub system: String,
    pub line: usize,
}

pub struct Preview {
    pub name: String,
    pub kind: PreviewKind,
    pub line: usize,
    pub is_test: bool,
    pub harness: HarnessCall,
    pub scene: SceneUsage,
}
