use std::path::Path;

use crate::common;
use crate::rules;
use crate::Entry;
use crate::EntryKind;
use crate::RawCode;
use crate::RuleViolation;

pub fn scan_entries(entries: &[Entry], base: &Path) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for entry in entries {
        if entry.kind != EntryKind::File {
            continue;
        }
        let Ok(content) = std::fs::read_to_string(&entry.absolute_path) else {
            continue;
        };
        for hit in RawCode(&content).find_comments() {
            if matches!(&hit.kind, common::CommentKind::Line(text) if rules::no_comments::NoComments::is_allowlisted(text))
            {
                continue;
            }
            violations.push(RuleViolation::from_site(
                base.join(&entry.relative_path),
                hit.line,
                0,
                rules::no_comments::NoComments::CODE,
                "comment in `src/`/`methods/` — delete it and put its meaning in names, types or the test"
                    .to_string(),
            ));
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::scan_entries;
    use crate::Entry;
    use crate::EntryKind;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("a.rs");
        std::fs::write(&path, "// stray\nfn a() {}\n// no test_usage necessary\n").unwrap();
        let entry = Entry {
            relative_path: PathBuf::from("a.rs"),
            absolute_path: path,
            kind: EntryKind::File,
        };
        let violations = scan_entries(&[entry], dir.path());
        assert_eq!(violations.len(), 1);
    }
}
