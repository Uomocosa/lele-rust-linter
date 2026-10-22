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
    pub fn detect(body: &syn::Block, module_fns: &scan::ModuleFns) -> Self {}
}

#[cfg(test)]
mod tests {
    use crate::scan::ModuleFns;
    use crate::scan::SceneUsage;

    #[test]
    fn test_usage() {
        let body: syn::Block = syn::parse_quote! {{ let _ = 1; }};
        let usage = SceneUsage::detect(&body, &ModuleFns::new());
        assert_eq!(usage.state_count, 0);
        assert_eq!(usage.build_uses, Vec::<String>::new());
    }
}
