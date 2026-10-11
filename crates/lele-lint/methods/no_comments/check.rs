use std::path::Path;

use crate::checkers;
use crate::common;
use crate::Entry;
use crate::EntryKind;
use crate::Project;
use crate::RuleViolation;

pub fn check(_self: &checkers::no_comments::NoComments, project: &Project) -> Vec<RuleViolation> {
    let mut violations = scan_entries(&project.entries, &project.src_dir);
    if let Some(methods_dir) = &project.methods_dir {
        violations.extend(scan_entries(&project.methods_entries, methods_dir));
    }
    violations
}

// needed helper: scan one entry set for disallowed comments
fn scan_entries(entries: &[Entry], base: &Path) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for entry in entries {
        if entry.kind != EntryKind::File {
            continue;
        }
        let Ok(content) = std::fs::read_to_string(&entry.absolute_path) else {
            continue;
        };
        for hit in common::find_comments(&content) {
            if matches!(&hit.kind, common::CommentKind::Line(text) if is_allowlisted(text)) {
                continue;
            }
            violations.push(RuleViolation::site(
                base.join(&entry.relative_path),
                hit.line,
                0,
                checkers::no_comments::NoComments::CODE,
                "comments are not allowed in the methods layout (code, tests and logs only)"
                    .to_string(),
            ));
        }
    }
    violations
}

// needed helper: the two sanctioned comment opt-outs
fn is_allowlisted(text: &str) -> bool {
    let trimmed = text.trim();
    trimmed == "// no test_usage necessary" || trimmed.starts_with("// needed helper:")
}

#[cfg(test)]
mod tests {
    use super::check;
    use crate::checkers;
    use crate::Project;

    #[test]
    fn test_usage() {
        let checker = checkers::no_comments::NoComments;
        let project = Project::default();
        assert!(check(&checker, &project).is_empty());
    }
}
