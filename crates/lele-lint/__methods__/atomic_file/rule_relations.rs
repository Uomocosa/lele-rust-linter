use crate::rules;
use crate::RuleRelation;

pub fn rule_relations(_self: &rules::atomic_file::AtomicFile) -> &'static [RuleRelation] {
    &[RuleRelation {
        defers_to: "bevy_folder",
        context: "files in `__bevy_systems__/` are not method files (no method-file inference)",
    }]
}

#[cfg(test)]
mod tests {
    use super::rule_relations;
    use crate::rules;

    #[test]
    fn test_usage() {
        let found = rule_relations(&rules::atomic_file::AtomicFile);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].defers_to, "bevy_folder");
    }
}
