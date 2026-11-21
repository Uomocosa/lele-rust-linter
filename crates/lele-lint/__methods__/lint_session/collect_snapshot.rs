use crate::Linter;
use crate::RuleSnapshot;

pub fn collect_snapshot(linter: &Linter) -> Vec<RuleSnapshot> {
    linter
        .rules
        .iter()
        .map(|rule| RuleSnapshot {
            name: rule.name().to_owned(),
            code: rule.code().to_owned(),
            namespace: rule.namespace().to_owned(),
            is_locked: rule.is_locked(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_snapshot;
    use crate::Linter;

    #[test]
    fn test_usage() {
        let linter = Linter::default();
        let before = linter.rules.len();
        let snapshot = collect_snapshot(&linter);
        assert_eq!(snapshot.len(), before);
        assert!(snapshot.iter().any(|entry| entry.name == "no_comments"));
    }
}
