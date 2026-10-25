use crate::rules;
use crate::RuleViolation;
use std::path::PathBuf;
use syn::visit::Visit;

pub(crate) struct PositionalVisitor {
    pub(crate) file_path: PathBuf,
    pub(crate) violations: Vec<RuleViolation>,
}

impl<'ast> Visit<'ast> for PositionalVisitor {
    fn visit_expr_field(&mut self, node: &'ast syn::ExprField) {
        if matches!(node.member, syn::Member::Unnamed(_)) {
            self.violations.push(RuleViolation::from_file(
                self.file_path.clone(),
                rules::no_positional::NoPositional::CODE,
                "positional field access — deref a single-field newtype (`*value`) or use named fields".to_string(),
            ));
        }
        syn::visit::visit_expr_field(self, node);
    }
}

// no test_usage necessary
