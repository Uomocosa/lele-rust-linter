use atomic_delegate_macros::atomic_delegates;

use crate::scan;

#[derive(Default)]
pub struct SceneUsage {
    pub build_uses: Vec<String>,
    pub state_count: usize,
    pub uncentered_kinds: Vec<String>,
}

#[atomic_delegates]
impl SceneUsage {
    pub fn from_fn_body(body: &syn::Block, module_fns: &scan::ModuleFns) -> Self {}
}

#[rustfmt::skip]
impl SceneUsage {
    pub(crate) fn kind_name(expr: &syn::Expr) -> Option<String> {
        let syn::Expr::Path(path) = expr else {
            return None;
        };
        path.path.segments.last().map(|segment| segment.ident.to_string())
    }
    pub(crate) fn field<'a>(literal: &'a syn::ExprStruct, name: &str) -> Option<&'a syn::Expr> {
        literal.fields.iter().find(|value| matches!(&value.member, syn::Member::Named(ident) if ident == name)).map(|value| &value.expr)
    }
}

#[cfg(test)]
mod tests {
    use crate::scan::ModuleFns;
    use crate::scan::SceneUsage;

    #[test]
    fn test_usage() {
        let body: syn::Block = syn::parse_quote! {{ let _ = 1; }};
        let usage = SceneUsage::from_fn_body(&body, &ModuleFns::new());
        assert_eq!(usage.state_count, 0);
        assert_eq!(usage.build_uses, Vec::<String>::new());
    }
}
