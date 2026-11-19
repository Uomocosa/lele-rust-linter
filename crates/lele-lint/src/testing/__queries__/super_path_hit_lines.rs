use syn::visit::Visit;

use crate::rules;

pub(crate) fn super_path_hit_lines(source: &str) -> Vec<usize> {
    let file = syn::parse_file(source).unwrap();
    let mut hits = Vec::new();
    {
        let mut visitor = rules::no_super_imports::SuperPathVisitor(&mut hits);
        visitor.visit_file(&file);
    }
    hits.into_iter().map(|hit| hit.line).collect()
}

#[cfg(test)]
mod tests {
    use super::super_path_hit_lines;

    #[test]
    fn test_usage() {
        assert_eq!(super_path_hit_lines("use super::config_load;\n"), vec![1]);
    }
}
