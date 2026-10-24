use std::path::Path;
use std::path::PathBuf;

use atomic_delegate_macros::atomic_delegates;
use serde::Deserialize;

use derive_more::Deref;

use crate::DunderPath;
use crate::Error;
use crate::LeleSection;
use crate::Project;

pub const CONFIG_FILENAME: &str = "lele.toml";

#[derive(Deserialize, Debug, Default, Deref)]
pub struct Config(pub Option<LeleSection>);

#[atomic_delegates]
impl TryFrom<&Path> for Config {
    type Error = Error;
    fn try_from(path: &Path) -> Result<Self, Self::Error> {}
}

#[atomic_delegates]
impl Config {
    pub fn dunder_paths(&self) -> Vec<DunderPath> {}
    pub fn resolve(explicit: Option<&Path>, project: &Project) -> Result<(Config, PathBuf), Error> {
    }
    pub fn load_at(path: &Path, fallback: &Path) -> Result<(Config, PathBuf), Error> {}
}

// no test_usage necessary
