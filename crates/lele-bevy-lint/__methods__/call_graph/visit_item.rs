use crate::scan;

pub fn visit_item(graph: &mut scan::CallGraph, item: &syn::Item) {
    match item {
        syn::Item::Fn(func) => {
            graph.record_call(&func.sig.ident.to_string(), &func.block);
        }
        syn::Item::Impl(impl_block) => {
            for item in &impl_block.items {
                if let syn::ImplItem::Fn(method) = item {
                    graph.record_call(&method.sig.ident.to_string(), &method.block);
                }
            }
        }
        syn::Item::Mod(module) => {
            if let Some((_, inner)) = &module.content {
                for item in inner {
                    graph.visit_item(item);
                }
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::visit_item;
    use crate::scan::CallGraph;

    #[test]
    fn test_usage() {
        let mut graph = CallGraph::from(&lele_lint::Project::default());
        let item: syn::Item = syn::parse_quote! {
            mod inner {
                fn setup() { extra(); }
            }
        };
        visit_item(&mut graph, &item);
        assert!(graph.reaches_production("extra"));
        assert!(!graph.reaches_production("missing"));
    }
}
