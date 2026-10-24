use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;

pub trait Rule {
    fn name(&self) -> &'static str;
    fn code(&self) -> &'static str;
    fn doc(&self) -> RuleDoc;
    fn check(&self, project: &Project) -> Vec<RuleViolation>;
}

// no test_usage necessary
