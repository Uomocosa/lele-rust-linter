use crate::rules;
use std::path::Path;

pub fn suggestion(rel_path: &Path, hit: &str) -> (String, String) {
    let Some(rest) = hit.strip_prefix("super::") else {
        return rules::no_super_imports::NoSuperImports::generic_suggestion();
    };
    if rest.starts_with("super::") {
        return rules::no_super_imports::NoSuperImports::generic_suggestion();
    }
    let mut parts = rel_path.components();
    let first = parts.next().and_then(|c| c.as_os_str().to_str());
    match first {
        Some(domain) if parts.next().is_some() => {
            (format!("use crate::{domain};"), format!("{domain}::{rest}"))
        }
        _ => {
            let head = rest.split("::").next().unwrap_or(rest);
            (format!("use crate::{head};"), rest.to_string())
        }
    }
}

// no test_usage necessary
