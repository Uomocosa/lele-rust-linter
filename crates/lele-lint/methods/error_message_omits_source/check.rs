use syn::spanned::Spanned;

use crate::rules;
use crate::ItemMarker;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &rules::error_message_omits_source::ErrorMessageOmitsSource,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for source in project.content_sources() {
        for item in &source.file.items {
            let syn::Item::Enum(item_enum) = item else {
                continue;
            };
            if !item_enum
                .attrs
                .iter()
                .any(|attr| ItemMarker(attr).derives_any_of(&["Error"]))
            {
                continue;
            }
            for variant in &item_enum.variants {
                let Some(message) = variant
                    .attrs
                    .iter()
                    .find(|attr| attr.path().is_ident("error"))
                else {
                    continue;
                };
                let Some(source_field) =
                    rules::error_message_omits_source::ErrorMessageOmitsSource::source_field_name(
                        &variant.fields,
                    )
                else {
                    continue;
                };
                if rules::error_message_omits_source::ErrorMessageOmitsSource::message_prints_field(
                    message,
                    &source_field,
                ) {
                    violations.push(RuleViolation::from_site(
                        project.absolute_path(source.origin, source.relative_path),
                        message.span().start().line,
                        0,
                        rules::error_message_omits_source::ErrorMessageOmitsSource::CODE,
                        format!(
                            "`{}::{}` prints its source `{source_field}` — reporters already print the `source()` chain; drop it from the message, or write `#[error(transparent)]` to forward the source's message",
                            item_enum.ident, variant.ident
                        ),
                    ));
                }
            }
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::check;
    use crate::rules::error_message_omits_source::ErrorMessageOmitsSource;
    use crate::Project;

    fn count(src: &str) -> usize {
        let mut project = Project::default();
        project
            .parsed_files
            .insert(PathBuf::from("error.rs"), syn::parse_str(src).unwrap());
        check(&ErrorMessageOmitsSource, &project).len()
    }

    #[test]
    fn test_usage() {
        let bad = "#[derive(Debug, thiserror::Error)]\npub enum Error {\n    \
                   #[error(\"i/o error: {0}\")]\n    Io(#[from] std::io::Error),\n}\n";
        let good = "#[derive(Debug, thiserror::Error)]\npub enum Error {\n    \
                    #[error(\"i/o error\")]\n    Io(#[from] std::io::Error),\n}\n";
        assert_eq!(count(bad), 1);
        assert_eq!(count(good), 0);
    }

    #[test]
    fn test_usage_flags_named_source_and_format_arg() {
        let src = "#[derive(Debug, Error)]\npub enum Error {\n    \
                   #[error(\"read {path}: {source}\")]\n    Read { path: String, source: std::io::Error },\n    \
                   #[error(\"parse: {}\", .0)]\n    Parse(#[source] std::num::ParseIntError),\n}\n";
        assert_eq!(count(src), 2);
    }

    #[test]
    fn test_usage_allows_transparent_and_plain_fields() {
        let src = "#[derive(Debug, Error)]\npub enum Error {\n    \
                   #[error(transparent)]\n    Io(#[from] std::io::Error),\n    \
                   #[error(\"bad port {0}\")]\n    Port(u16),\n    \
                   #[error(\"read {path}\")]\n    Read { path: String, #[source] cause: std::io::Error },\n}\n";
        assert_eq!(count(src), 0);
    }
}

// no test_usage necessary
