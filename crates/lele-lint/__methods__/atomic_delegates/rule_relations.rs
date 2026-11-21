use crate::rules;
use crate::RuleRelation;

pub fn rule_relations(_self: &rules::atomic_delegates::AtomicDelegates) -> &'static [RuleRelation] {
    &[RuleRelation {
        defers_to: "inline_trait_methods",
        context: "methods listed in `[[lele.project.inline_trait_methods]]` stay inline instead of delegated",
    }]
}

#[cfg(test)]
mod tests {
    use super::rule_relations;
    use crate::rules;

    #[test]
    fn test_usage() {
        let found = rule_relations(&rules::atomic_delegates::AtomicDelegates);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].defers_to, "inline_trait_methods");
    }
}
