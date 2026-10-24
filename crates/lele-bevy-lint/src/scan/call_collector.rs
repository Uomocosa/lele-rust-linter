use syn::visit::Visit;

use crate::scan;

pub(crate) struct CallCollector<'a> {
    pub(crate) calls: &'a mut Vec<String>,
    pub(crate) systems: &'a mut Vec<String>,
}

#[rustfmt::skip]
impl<'ast> Visit<'ast> for CallCollector<'_> {
    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = &*node.func
            && let Some(last) = path.path.segments.last()
        {
            self.calls.push(last.ident.to_string());
        }
        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if let Some(idents) = scan::AppMethodCall(node).system_args() {
            self.calls.extend(idents.iter().cloned());
            self.systems.extend(idents);
            self.visit_expr(&node.receiver);
        } else if let Some(idents) = scan::AppMethodCall(node).observer_args() {
            self.calls.extend(idents.iter().cloned());
            self.systems.extend(idents);
            self.visit_expr(&node.receiver);
        } else {
            syn::visit::visit_expr_method_call(self, node);
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::CallCollector;

    #[test]
    fn test_usage() {
        let block: syn::Block = syn::parse_quote! {{
            setup();
            app.add_systems(Update, tick);
        }};
        let mut calls = Vec::new();
        let mut systems = Vec::new();
        let mut collector = CallCollector {
            calls: &mut calls,
            systems: &mut systems,
        };
        collector.visit_block(&block);
        assert!(calls.contains(&"setup".to_string()));
        assert!(systems.contains(&"tick".to_string()));
    }
}
