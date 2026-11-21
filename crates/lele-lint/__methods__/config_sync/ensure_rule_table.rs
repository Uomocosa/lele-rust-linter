use crate::traits::Rule;
use crate::ConfigSync;
use crate::SyncOutcome;

pub fn ensure_rule_table(
    doc: &mut toml_edit::DocumentMut,
    rule: &dyn Rule,
    outcome: &mut SyncOutcome,
) {
    let root = doc.as_table_mut();
    let Some(lele) = ConfigSync::child_table(root, "lele") else {
        return;
    };
    let Some(rules) = ConfigSync::child_table(lele, "rules") else {
        return;
    };
    let Some(namespaced) = ConfigSync::child_table(rules, rule.namespace()) else {
        return;
    };
    let code = rule.code();
    let locked = rule.is_locked();
    let head = if locked {
        format!("# {code} · locked · ")
    } else {
        format!("# {code} · ")
    };
    let max_summary = 119usize.saturating_sub(head.chars().count()).min(100);
    let summary = ConfigSync::first_sentence(rule.doc().summary, max_summary);
    let comment = format!("{head}{summary}");
    let qualified = format!("lele.rules.{}.{name}", rule.namespace(), name = rule.name());
    let Some(item) = namespaced.get_mut(rule.name()) else {
        let mut created = toml_edit::Table::new();
        created.insert("level", toml_edit::value("deny"));
        created.decor_mut().set_prefix(format!("\n{comment}\n"));
        namespaced.insert(rule.name(), toml_edit::Item::Table(created));
        outcome.missing.push(qualified);
        return;
    };
    let Some(table) = item.as_table_mut() else {
        return;
    };
    table.set_implicit(false);
    let prefix = table
        .decor()
        .prefix()
        .map(|raw| raw.as_str().unwrap_or(""))
        .unwrap_or("");
    let mut lines: Vec<&str> = prefix.split('\n').collect();
    if lines.last().is_some_and(|line| line.is_empty()) {
        lines.pop();
    }
    let code_prefix = format!("# {code}");
    let stale = lines.last().is_some_and(|line| {
        let trimmed = line.trim_start();
        trimmed.starts_with("# LELE") || trimmed.starts_with(&code_prefix)
    });
    if stale {
        lines.pop();
    } else if lines == [""] {
        lines.push("");
    }
    lines.push(comment.as_str());
    let updated = lines.join("\n") + "\n";
    if updated != prefix {
        table.decor_mut().set_prefix(updated);
        outcome.refreshed.push(qualified);
    }
}

#[cfg(test)]
mod tests {
    use super::ensure_rule_table;
    use crate::Linter;
    use crate::SyncOutcome;

    #[test]
    fn test_usage() {
        let linter = Linter::default();
        let rule = linter
            .rules
            .iter()
            .find(|rule| rule.name() == "domain_import")
            .unwrap();
        let mut doc: toml_edit::DocumentMut = String::new().parse().unwrap();
        let mut outcome = SyncOutcome::default();
        ensure_rule_table(&mut doc, rule.as_ref(), &mut outcome);
        assert_eq!(outcome.missing, vec!["lele.rules.lint.domain_import"]);
        assert!(doc.to_string().contains("level = \"deny\""));
        assert!(doc.to_string().contains("# LELE016 ·"));
    }
}
