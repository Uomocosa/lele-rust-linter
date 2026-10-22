use std::path::Path;

use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

#[derive(Clone, Copy, Deref)]
pub struct SourcePath<'a>(pub &'a Path);

#[atomic_delegates]
impl SourcePath<'_> {
    pub fn is_crate_root(&self) -> bool {}
    pub fn is_index_file(&self) -> bool {}
    pub fn module_path(&self) -> Vec<String> {}
}

#[cfg(test)]
mod tests {
    use super::SourcePath;
    use std::path::Path;

    #[test]
    fn test_usage() {
        assert!(SourcePath(Path::new("lib.rs")).is_crate_root());
        assert!(!SourcePath(Path::new("player/player.rs")).is_crate_root());
        assert!(SourcePath(Path::new("player/mod.rs")).is_index_file());
        assert_eq!(
            SourcePath(Path::new("player/player.rs")).module_path(),
            vec!["player".to_string(), "player".to_string()]
        );
    }
}
