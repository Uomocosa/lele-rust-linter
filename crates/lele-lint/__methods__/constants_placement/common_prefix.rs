use std::path::{Path, PathBuf};

pub fn common_prefix(a: &Path, b: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for (x, y) in a.components().zip(b.components()) {
        if x == y {
            out.push(x);
        } else {
            break;
        }
    }
    out
}

// no test_usage necessary
