use syn::visit::Visit;

use crate::scan;

pub fn local_names(expr: &syn::Expr) -> Vec<String> {
    let mut names = Vec::new();
    let mut collector = scan::LocalNames(&mut names);
    collector.visit_expr(expr);
    names
}

#[cfg(test)]
mod tests {
    use super::local_names;

    #[test]
    fn test_usage() {
        let expr: syn::Expr = syn::parse_str("run(&scene())").unwrap();
        let names = local_names(&expr);
        assert!(names.contains(&"run".to_string()));
        assert!(names.contains(&"scene".to_string()));
    }
}
