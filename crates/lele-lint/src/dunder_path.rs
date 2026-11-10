use std::path::PathBuf;

use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;
use serde::Deserialize;

use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq, Deref, Deserialize)]
#[serde(try_from = "PathBuf")]
pub struct DunderPath(pub(crate) PathBuf);

impl DunderPath {
    pub fn defaults() -> Vec<Self> {
        ["__basic__", "__bevy_systems__"]
            .into_iter()
            .filter_map(|name| Self::try_from(PathBuf::from(name)).ok())
            .collect()
    }
}

#[atomic_delegates]
impl TryFrom<PathBuf> for DunderPath {
    type Error = Error;
    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {}
}

#[rustfmt::skip]
impl DunderPath {
    pub fn module_name(&self) -> &str {
        self.file_stem()
            .and_then(|stem| stem.to_str())
            .and_then(|stem| stem.strip_prefix("__"))
            .and_then(|rest| rest.strip_suffix("__"))
            .unwrap_or_default()
    }
    pub fn is_dunder_dir(&self) -> bool { self.extension().is_none() }
    pub fn is_dunder_file(&self) -> bool { self.extension().is_some() }
    pub fn is_bevy_systems_dir(&self) -> bool {
        self.is_dunder_dir() && self.module_name() == "bevy_systems"
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::DunderPath;

    #[test]
    fn test_usage() {
        let dir = DunderPath::try_from(PathBuf::from("__basic__")).unwrap();
        assert_eq!(dir.module_name(), "basic");
        assert!(dir.is_dunder_dir());
        assert!(!dir.is_dunder_file());

        let file = DunderPath::try_from(PathBuf::from("src/__notes__.md")).unwrap();
        assert_eq!(file.module_name(), "notes");
        assert!(file.is_dunder_file());
        assert!(!file.is_dunder_dir());
    }

    #[test]
    fn test_usage_defaults_carry_the_systems_mount_name() {
        let defaults = DunderPath::defaults();
        assert_eq!(defaults.len(), 2);
        assert_eq!(defaults[0].module_name(), "basic");
        assert_eq!(defaults[1].module_name(), "bevy_systems");
        assert!(defaults[1].is_bevy_systems_dir());
        assert!(!defaults[0].is_bevy_systems_dir());
    }
}
