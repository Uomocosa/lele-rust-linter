use syn::visit::Visit;

use crate::scan;

pub fn self_calls(block: &syn::Block) -> Vec<String> {
    let mut names = Vec::new();
    let mut collector = scan::SelfCalls(&mut names);
    collector.visit_block(block);
    names
}

#[cfg(test)]
mod tests {
    use super::self_calls;

    #[test]
    fn test_usage() {
        let block: syn::Block = syn::parse_quote! {{ self.build_plugin(app); }};
        assert_eq!(self_calls(&block), vec!["build_plugin".to_string()]);
    }
}
