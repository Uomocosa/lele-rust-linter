use crate::rules;
use crate::RuleRelation;

pub fn rule_relations(
    _self: &rules::method_file_co_location::MethodFileCoLocation,
) -> &'static [RuleRelation] {
    &[RuleRelation {
        defers_to: "bevy_folder",
        context: "a `__bevy_systems__/` file may start with a type's stem",
    }]
}

#[cfg(test)]
mod tests {
    use super::rule_relations;
    use crate::rules;

    #[test]
    fn test_usage() {
        let found = rule_relations(&rules::method_file_co_location::MethodFileCoLocation);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].defers_to, "bevy_folder");
    }
}
