use std::collections::HashMap;
use std::path::PathBuf;

use crate::ModuleInfo;

pub type ModuleInfoMap = HashMap<PathBuf, ModuleInfo>;
pub type RustfmtKey = (PathBuf, usize, usize);
