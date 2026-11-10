use crate::rules;

pub fn matches_hit(forbidden: &str, hit: &rules::boundary_imports::PathHit) -> bool {
    let prefix = format!("{}::", hit.path);
    if hit.glob {
        return forbidden == hit.path || forbidden.starts_with(&prefix);
    }
    forbidden == hit.path || hit.path.starts_with(&format!("{forbidden}::"))
}

// no test_usage necessary
