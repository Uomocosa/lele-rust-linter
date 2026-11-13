use crate::FnBody;
use crate::PlacementKind;

pub fn placement_kind(block: FnBody, rustfmt_one_line: bool) -> PlacementKind {
    if block.stmts.is_empty() {
        return PlacementKind::OneLiner;
    }
    if block.stmts.len() > 3 {
        return PlacementKind::Delegate;
    }
    if block.stmts.len() > 1 {
        return PlacementKind::FewStatements;
    }
    if block.is_simple_block() {
        return PlacementKind::SimpleBlock;
    }
    if rustfmt_one_line && !block.is_control_statement() {
        return PlacementKind::OneLiner;
    }
    PlacementKind::Delegate
}

#[cfg(test)]
mod tests {
    use crate::FnBody;
    use crate::PlacementKind;

    #[test]
    fn test_usage() {
        let empty: syn::Block = syn::parse_quote! {{}};
        assert_eq!(
            FnBody(&empty).placement_kind(false),
            PlacementKind::OneLiner
        );
        let simple: syn::Block = syn::parse_quote! {{ match x { 0 => 1, _ => 2 } }};
        assert_eq!(
            FnBody(&simple).placement_kind(true),
            PlacementKind::SimpleBlock
        );
        let few: syn::Block = syn::parse_quote! {{ let a = 1; let b = 2; a + b }};
        assert_eq!(
            FnBody(&few).placement_kind(false),
            PlacementKind::FewStatements
        );
        let many: syn::Block =
            syn::parse_quote! {{ let a = 1; let b = 2; let c = 3; let d = 4; d }};
        assert_eq!(FnBody(&many).placement_kind(false), PlacementKind::Delegate);
        let line: syn::Block = syn::parse_quote! {{ self.0 }};
        assert_eq!(FnBody(&line).placement_kind(true), PlacementKind::OneLiner);
        assert_eq!(FnBody(&line).placement_kind(false), PlacementKind::Delegate);
    }
}
