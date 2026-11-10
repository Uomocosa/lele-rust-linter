use syn::visit::Visit;

use crate::common;
use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check(_self: &rules::no_crate_paths::NoCratePaths, project: &Project) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for source in project.content_sources() {
        let rel_path = source.relative_path;
        let file = source.file;
        if common::SourcePath(rel_path).is_crate_root() {
            continue;
        }
        let mut hits = Vec::new();
        {
            let mut visitor = rules::no_crate_paths::CratePathVisitor(&mut hits);
            visitor.visit_file(file);
        }
        for hit in std::mem::take(&mut hits) {
            violations.push(RuleViolation::from_site(project.absolute_path(source.origin, rel_path), hit.line, 0, rules::no_crate_paths::NoCratePaths::CODE, format!(
                    "`{}` path used outside a top-level `use` declaration — add `use crate::<module>;` at the top of the file and reference `<module>::…` instead",
                    hit.path
                )));
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::check;
    use crate::rules;
    use crate::Project;
    use syn::visit::Visit;

    #[test]
    fn test_usage() {
        let rule = rules::no_crate_paths::NoCratePaths;
        let project = Project::default();
        assert!(check(&rule, &project).is_empty());
    }

    fn hit_lines(src: &str) -> Vec<usize> {
        let file = syn::parse_file(src).unwrap();
        let mut hits = Vec::new();
        {
            let mut visitor = rules::no_crate_paths::CratePathVisitor(&mut hits);
            visitor.visit_file(&file);
        }
        std::mem::take(&mut hits)
            .into_iter()
            .map(|hit| hit.line)
            .collect()
    }

    #[test]
    fn test_usage_flags_inline_crate_path() {
        let src = r"
            use crate::boxes;
            pub fn connect(own_id: crate::boxes::PlayerId) {}
        ";
        assert_eq!(hit_lines(src), vec![3]);
    }

    #[test]
    fn test_usage_allows_top_level_use() {
        let src = "use crate::boxes;\nfn f() -> boxes::PlayerId { boxes::PlayerId::default() }\n";
        assert_eq!(hit_lines(src), Vec::<usize>::new());
    }

    #[test]
    fn test_usage_allows_pub_crate_visibility() {
        let src = "pub(crate) mod boxes;\npub(crate) use boxes::PlayerId;\n";
        assert_eq!(hit_lines(src), Vec::<usize>::new());
    }

    #[test]
    fn test_usage_allows_super_and_self() {
        let src = "use super::player;\nfn f() { super::player::Player::default(); }\n";
        assert_eq!(hit_lines(src), Vec::<usize>::new());
    }
}
