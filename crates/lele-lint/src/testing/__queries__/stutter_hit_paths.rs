use std::collections::HashSet;

use syn::visit::Visit;

use crate::rules;

struct Collector<'a> {
    hits: &'a mut Vec<rules::no_stuttered_path::StutterHit>,
    stems: &'a HashSet<String>,
}

impl<'ast> Visit<'ast> for Collector<'_> {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        rules::no_stuttered_path::NoStutteredPath::collect_stutter(path, self.stems, self.hits);
        syn::visit::visit_path(self, path);
    }
}

pub(crate) fn stutter_hit_paths(source: &str, stems: &HashSet<String>) -> Vec<String> {
    let file = syn::parse_file(source).unwrap();
    let mut hits = Vec::new();
    {
        let mut visitor = Collector {
            hits: &mut hits,
            stems,
        };
        visitor.visit_file(&file);
    }
    std::mem::take(&mut hits)
        .into_iter()
        .map(|hit| hit.path)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::stutter_hit_paths;
    use std::collections::HashSet;

    #[test]
    fn test_usage() {
        let stems = HashSet::from(["diagnostic".to_string()]);
        assert_eq!(
            stutter_hit_paths("pub fn f(d: Vec<diagnostic::Diagnostic>) {}", &stems),
            vec!["diagnostic::Diagnostic"]
        );
    }
}
