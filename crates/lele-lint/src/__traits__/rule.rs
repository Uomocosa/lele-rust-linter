use crate::Project;
use crate::RuleDoc;
use crate::RuleRelation;
use crate::RuleViolation;

pub trait Rule {
    fn name(&self) -> &'static str;
    fn code(&self) -> &'static str;
    fn doc(&self) -> RuleDoc;
    fn check(&self, project: &Project) -> Vec<RuleViolation>;
    fn relations(&self) -> &'static [RuleRelation] {
        &[]
    }
    fn namespace(&self) -> &'static str {
        if self.code().starts_with("LELEBEVY") {
            "bevy"
        } else if self.code().starts_with("LELE") {
            "lint"
        } else {
            "local"
        }
    }
    fn is_locked(&self) -> bool {
        self.code().starts_with("LELECORE")
    }
}

// no test_usage necessary
