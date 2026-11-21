use crate::ConfigReportRow;
use crate::ConfigSync;
use crate::RelaxedRule;
use crate::RuleSnapshot;

pub fn render_config_report(snapshot: &[RuleSnapshot], relaxed: &[RelaxedRule]) -> String {
    let mut rows: Vec<ConfigReportRow> = Vec::new();
    for entry in snapshot {
        rows.push(ConfigSync::report_row(entry, relaxed));
    }
    let mut code_width = "CODE".len();
    let mut name_width = "NAME".len();
    let mut namespace_width = "NAMESPACE".len();
    let mut locked_width = "LOCKED".len();
    let mut level_width = "LEVEL".len();
    let mut exempts_width = "EXEMPTS".len();
    for row in &rows {
        code_width = code_width.max(row.code.len());
        name_width = name_width.max(row.name.len());
        namespace_width = namespace_width.max(row.namespace.len());
        locked_width = locked_width.max(row.locked.len());
        level_width = level_width.max(row.level.len());
        exempts_width = exempts_width.max(row.exempts.len());
    }
    let mut out = format!(
        "{:<code_width$}  {:<name_width$}  {:<namespace_width$}  {:<locked_width$}  {:<level_width$}  {:<exempts_width$}  REASON\n",
        "CODE", "NAME", "NAMESPACE", "LOCKED", "LEVEL", "EXEMPTS",
    );
    for row in &rows {
        let mut line = format!(
            "{:<code_width$}  {:<name_width$}  {:<namespace_width$}  {:<locked_width$}  {:<level_width$}  {:<exempts_width$}",
            row.code, row.name, row.namespace, row.locked, row.level, row.exempts,
        );
        if !row.reason.is_empty() {
            line.push_str("  ");
            line.push_str(&row.reason);
        }
        line = line.trim_end().to_owned();
        out.push_str(&line);
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::render_config_report;
    use crate::RelaxedRule;
    use crate::RuleSnapshot;

    #[test]
    fn test_usage() {
        let snapshot = vec![
            RuleSnapshot {
                name: String::from("domain_import"),
                code: String::from("LELE016"),
                namespace: String::from("lint"),
                is_locked: false,
            },
            RuleSnapshot {
                name: String::from("no_comments"),
                code: String::from("LELECORE015"),
                namespace: String::from("lint"),
                is_locked: true,
            },
        ];
        let report = render_config_report(&snapshot, &[]);
        let rows: Vec<&str> = report.lines().collect();
        assert_eq!(rows.len(), 3);
        assert_eq!(
            rows[0],
            "CODE         NAME           NAMESPACE  LOCKED  LEVEL  EXEMPTS  REASON"
        );
        assert_eq!(
            rows[1],
            "LELE016      domain_import  lint               deny   0"
        );
        assert_eq!(
            rows[2],
            "LELECORE015  no_comments    lint       locked  deny   0"
        );
        assert_eq!(rows[1].find("lint"), rows[2].find("lint"));
    }

    #[test]
    fn columns_line_up_across_rows() {
        let snapshot = vec![
            RuleSnapshot {
                name: String::from("a"),
                code: String::from("LELE1"),
                namespace: String::from("lint"),
                is_locked: false,
            },
            RuleSnapshot {
                name: String::from("much_longer_name"),
                code: String::from("LELECORE015"),
                namespace: String::from("lint"),
                is_locked: true,
            },
        ];
        let report = render_config_report(&snapshot, &[]);
        let rows: Vec<&str> = report.lines().collect();
        assert_eq!(rows.len(), 3);
        assert!(rows[1].starts_with("LELE1        a                 lint"));
        assert!(rows[2].starts_with("LELECORE015  much_longer_name  lint"));
        assert_eq!(rows[1].find("lint"), rows[2].find("lint"));
        assert_eq!(rows[1].find("deny"), rows[2].find("deny"));
    }

    #[test]
    fn off_rule_shows_off_and_its_reason() {
        let snapshot = vec![RuleSnapshot {
            name: String::from("domain_import"),
            code: String::from("LELE016"),
            namespace: String::from("lint"),
            is_locked: false,
        }];
        let relaxed = vec![RelaxedRule::Off {
            name: String::from("domain_import"),
            code: String::from("LELE016"),
            reason: String::from("legacy"),
        }];
        let report = render_config_report(&snapshot, &relaxed);
        assert!(
            report.contains("LELE016  domain_import  lint               off    0        legacy\n")
        );
    }

    #[test]
    fn exempt_rule_shows_count_and_first_reason() {
        let snapshot = vec![RuleSnapshot {
            name: String::from("no_comments"),
            code: String::from("LELECORE015"),
            namespace: String::from("lint"),
            is_locked: true,
        }];
        let relaxed = vec![
            RelaxedRule::Exempt {
                name: String::from("no_comments"),
                code: String::from("LELECORE015"),
                path: std::path::PathBuf::from("src/legacy"),
                reason: String::from("mirror"),
                replaced_by: None,
            },
            RelaxedRule::Exempt {
                name: String::from("no_comments"),
                code: String::from("LELECORE015"),
                path: std::path::PathBuf::from("src/other"),
                reason: String::from("vendored"),
                replaced_by: None,
            },
        ];
        let report = render_config_report(&snapshot, &relaxed);
        assert!(report
            .contains("LELECORE015  no_comments  lint       locked  deny   2        mirror\n"));
    }
}
