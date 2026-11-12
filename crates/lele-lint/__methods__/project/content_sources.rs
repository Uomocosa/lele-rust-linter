use crate::Origin;
use crate::ParsedSource;
use crate::Project;

pub fn content_sources(project: &Project) -> impl Iterator<Item = ParsedSource<'_>> {
    project
        .sources()
        .filter(|source| !matches!(source.origin, Origin::Examples))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::content_sources;
    use crate::Origin;
    use crate::Project;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project
            .parsed_files
            .insert(PathBuf::from("a.rs"), syn::parse_str("").unwrap());
        project
            .example_parsed_files
            .insert(PathBuf::from("c.rs"), syn::parse_str("").unwrap());
        let origins: Vec<Origin> = content_sources(&project)
            .map(|source| source.origin)
            .collect();
        assert_eq!(origins.len(), 1);
        assert!(!origins.contains(&Origin::Examples));
    }
}
