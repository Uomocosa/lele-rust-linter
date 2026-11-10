use crate::rules;
use crate::DunderPath;
use std::ffi::OsStr;
use std::path::Path;

pub fn dunder_in_path<'a>(dunders: &'a [DunderPath], target: &str) -> Option<&'a DunderPath> {
    let target = Path::new(target);
    let module = if target.file_name() == Some(OsStr::new("mod.rs")) {
        target.parent()?.file_name()?
    } else {
        target.file_stem()?
    };
    rules::dunder_module_name::DunderModuleName::dunder_named(dunders, module.to_str()?)
}

// no test_usage necessary
