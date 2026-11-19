use syn::visit::Visit;

use crate::rules;

pub(crate) fn crate_path_hit_lines(source: &str) -> Vec<usize> {
    let file = syn::parse_file(source).unwrap();
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

#[cfg(test)]
mod tests {
    use super::crate_path_hit_lines;

    #[test]
    fn test_usage() {
        assert_eq!(
            crate_path_hit_lines("use crate::boxes;\npub fn f(id: crate::boxes::Id) {}\n"),
            vec![2]
        );
    }
}
