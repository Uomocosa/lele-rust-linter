#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PreviewKind {
    Png,
    Mp4,
    Scene,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HarnessCall {
    Missing,
    Unchecked,
    Checked,
}
