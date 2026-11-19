use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::PathBuf;

pub(crate) fn define_type(
    definitions: &mut BTreeMap<String, BTreeSet<PathBuf>>,
    rel_path: &str,
    name: &str,
) {
    definitions
        .entry(name.to_string())
        .or_default()
        .insert(PathBuf::from(rel_path));
}

#[cfg(test)]
mod tests {
    use super::define_type;
    use std::collections::BTreeMap;

    #[test]
    fn test_usage() {
        let mut definitions = BTreeMap::new();
        define_type(&mut definitions, "stock/unix_time.rs", "UnixTime");
        assert!(definitions.contains_key("UnixTime"));
    }
}
