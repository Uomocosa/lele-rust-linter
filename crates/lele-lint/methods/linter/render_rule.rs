use crate::Linter;
use crate::Rule;
use std::fmt::Write;

pub fn render_rule(rule: &dyn Rule) -> String {
    let doc = rule.doc();
    let mut out = String::new();
    let _ = writeln!(out, "### {} `{}`\n", rule.code(), rule.name());
    let _ = writeln!(out, "{}\n", doc.summary);
    let _ = writeln!(out, "**Why:** {}\n", doc.why);
    let _ = writeln!(out, "**Bad** (reports {}):\n", rule.code());
    out.push_str(&Linter::render_files(doc.bad));
    let _ = writeln!(out, "**Good:**\n");
    out.push_str(&Linter::render_files(doc.good));
    out
}

#[cfg(test)]
mod tests {
    use super::render_rule;
    use crate::Linter;

    #[test]
    fn test_usage() {
        let rules = Linter::default().rules;
        let text = render_rule(rules[0].as_ref());
        assert!(text.starts_with(&format!("### {}", rules[0].code())));
        assert!(text.contains("**Bad**"));
        assert!(text.contains("```rust"));
    }
}
