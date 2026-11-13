use syn::spanned::Spanned;

pub fn statement_text(source: &str, stmt: &syn::Stmt) -> String {
    let span = stmt.span();
    let (start, end) = (span.start(), span.end());
    let mut parts = Vec::new();
    for line_no in start.line..=end.line {
        let Some(line) = source.lines().nth(line_no.saturating_sub(1)) else {
            break;
        };
        let chars: Vec<char> = line.chars().collect();
        let from = if line_no == start.line {
            start.column
        } else {
            0
        };
        let to = if line_no == end.line {
            end.column
        } else {
            chars.len()
        };
        let piece: String = chars
            .get(from..to.min(chars.len()))
            .unwrap_or_default()
            .iter()
            .collect();
        parts.push(piece);
    }
    parts.join("\n")
}

#[cfg(test)]
mod tests {
    use crate::Project;

    #[test]
    fn test_usage() {
        let source = "fn f() {\n    g(1);\n    tracing::info!(\n        x\n    );\n}\n";
        let file: syn::File = syn::parse_str(source).unwrap();
        let syn::Item::Fn(func) = &file.items[0] else {
            panic!("expected a function");
        };
        assert_eq!(
            Project::statement_text(source, &func.block.stmts[0]),
            "g(1);"
        );
        assert_eq!(
            Project::statement_text(source, &func.block.stmts[1]),
            "tracing::info!(\n        x\n    );"
        );
    }
}
