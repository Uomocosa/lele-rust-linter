use crate::common;
use crate::Entry;
use crate::ModuleInfo;
use crate::ModuleInfoMap;

pub fn build(entries: &[Entry]) -> ModuleInfoMap {
    let mut map = ModuleInfoMap::new();

    for entry in entries {
        if !common::SourcePath(&entry.relative_path).is_index_file() {
            continue;
        }

        let content = match std::fs::read_to_string(&entry.absolute_path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let (declarations, reexports) = ModuleInfo::parse_mod_rs(&content);

        map.insert(
            entry.relative_path.clone(),
            ModuleInfo {
                rel_path: entry.relative_path.clone(),
                declarations,
                reexports,
            },
        );
    }

    map
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::build;
    use crate::Entry;
    use crate::EntryKind;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let rel = PathBuf::from("clicker/mod.rs");
        let abs = dir.path().join("mod.rs");
        std::fs::write(&abs, "mod foo;\npub use foo::Foo;\n").unwrap();
        let entries = vec![Entry {
            relative_path: rel.clone(),
            absolute_path: abs,
            kind: EntryKind::File,
        }];
        let map = build(&entries);
        let info = map.get(&rel).unwrap();
        assert_eq!(info.declarations.len(), 1);
        assert_eq!(info.declarations[0].name, "foo");
        assert_eq!(info.reexports.len(), 1);
        assert_eq!(info.reexports[0].segments, vec!["foo", "Foo"]);
    }
}
