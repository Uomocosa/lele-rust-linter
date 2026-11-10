use crate::rules;
use std::path::Path;

pub fn reads_opt_out(path: &Path) -> bool {
    let Ok(content) = std::fs::read_to_string(path) else {
        return false;
    };
    rules::test_usage::TestUsage::opt_out_at_end(&content)
}

// no test_usage necessary
