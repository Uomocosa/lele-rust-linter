use syn::visit::Visit;

use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &rules::no_stringified_errors::NoStringifiedErrors,
    project: &Project,
) -> Vec<RuleViolation> {
    let error_enums = rules::no_stringified_errors::NoStringifiedErrors::own_error_enums(project);
    if error_enums.is_empty() {
        return Vec::new();
    }
    let mut violations = Vec::new();
    for source in project.content_sources() {
        let mut hits = Vec::new();
        {
            let mut visitor = rules::no_stringified_errors::StringifiedErrorVisitor {
                error_enums: &error_enums,
                impl_self: None,
                is_from_impl_for_error_enum: false,
                hits: &mut hits,
            };
            visitor.visit_file(source.file);
        }
        for hit in hits {
            violations.push(RuleViolation::from_site(
                project.absolute_path(source.origin, source.relative_path),
                hit.line,
                0,
                rules::no_stringified_errors::NoStringifiedErrors::CODE,
                format!(
                    "`{}` is stored as text in `{}` — keep the error itself: a `#[source] source` field (plus context fields) or `#[from]`",
                    hit.error_name, hit.variant
                ),
            ));
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::check;
    use crate::rules::no_stringified_errors::NoStringifiedErrors;
    use crate::Location;
    use crate::Project;

    const ERROR_ENUM: &str = "#[derive(Debug, thiserror::Error)]\npub enum Error {\n    #[error(\"x\")]\n    Io(String),\n}\n";

    fn lines(src: &str) -> Vec<usize> {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("error.rs"),
            syn::parse_str(ERROR_ENUM).unwrap(),
        );
        project
            .parsed_files
            .insert(PathBuf::from("probe.rs"), syn::parse_str(src).unwrap());
        let mut found: Vec<usize> = check(&NoStringifiedErrors, &project)
            .into_iter()
            .map(|violation| match violation.location {
                Location::Site { line, .. } => line,
                Location::File => 0,
            })
            .collect();
        found.sort_unstable();
        found
    }

    #[test]
    fn test_usage() {
        assert_eq!(
            lines("fn f() { a().map_err(|e| Error::Io(e.to_string())); }\n"),
            vec![1]
        );
        assert_eq!(
            lines("fn f() { a().map_err(|e| Error::Io(e)); }\n"),
            Vec::<usize>::new()
        );
    }

    #[test]
    fn test_usage_flags_format_and_inline_args() {
        let src = "fn f() {\n    a().map_err(|e| Error::Io(format!(\"bad: {}\", e)));\n    \
                   b().map_err(|error| Error::Io(format!(\"bad: {error}\")));\n}\n";
        assert_eq!(lines(src), vec![2, 3]);
    }

    #[test]
    fn test_usage_flags_from_impl_and_error_helper() {
        let src = "impl From<std::io::Error> for Error {\n    \
                   fn from(e: std::io::Error) -> Self { Self::Io(e.to_string()) }\n}\n\
                   fn io(error: &std::io::Error) -> Error { Error::Io(format!(\"{error}\")) }\n";
        assert_eq!(lines(src), vec![2, 4]);
    }

    #[test]
    fn test_usage_ignores_foreign_targets_and_plain_strings() {
        let src =
            "fn parse(v: &str) -> Result<u8, String> { v.parse().map_err(|e| format!(\"{e}\")) }\n\
                   fn refused(name: &str) -> Error { Error::Io(format!(\"{name}\")) }\n\
                   fn g() { a().map_err(|e| Other::X(e.to_string())); }\n";
        assert_eq!(lines(src), Vec::<usize>::new());
    }
}

// no test_usage necessary
