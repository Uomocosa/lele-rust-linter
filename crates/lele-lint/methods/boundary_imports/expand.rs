use std::collections::HashMap;

pub fn expand(raw: &str, aliases: &HashMap<String, String>) -> String {
    let Some((first, rest)) = raw.split_once("::") else {
        return aliases.get(raw).cloned().unwrap_or_else(|| raw.to_string());
    };
    match aliases.get(first) {
        Some(full) if rest.is_empty() => full.clone(),
        Some(full) => format!("{full}::{rest}"),
        None => raw.to_string(),
    }
}

// no test_usage necessary
