use crate::rules;
use crate::RuleRelation;

pub fn rule_relations(
    _self: &rules::constants_placement::ConstantsPlacement,
) -> &'static [RuleRelation] {
    &[
        RuleRelation {
            defers_to: "flat_dir",
            context: "never moves a constant down into a declared flat dir; `clamp_flat_dir`",
        },
        RuleRelation {
            defers_to: "testing_layout",
            context: "files under `src/testing/` are neither users nor checked; their placement follows the testing roles",
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::rule_relations;
    use crate::rules;

    #[test]
    fn test_usage() {
        let found = rule_relations(&rules::constants_placement::ConstantsPlacement);
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].defers_to, "flat_dir");
        assert_eq!(found[1].defers_to, "testing_layout");
    }
}
