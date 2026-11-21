use crate::rules;
use crate::RuleRelation;

pub fn rule_relations(
    _self: &rules::no_free_functions::NoFreeFunctions,
) -> &'static [RuleRelation] {
    &[RuleRelation {
        defers_to: "bevy_folder",
        context: "Bevy systems in `__bevy_systems__/` are free functions by design",
    }]
}

#[cfg(test)]
mod tests {
    use super::rule_relations;
    use crate::rules;

    #[test]
    fn test_usage() {
        let found = rule_relations(&rules::no_free_functions::NoFreeFunctions);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].defers_to, "bevy_folder");
    }
}
