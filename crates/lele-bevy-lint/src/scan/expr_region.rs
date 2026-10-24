use atomic_delegate_macros::atomic_delegates;

use crate::scan;

pub struct ExprRegion<'a> {
    pub exprs: Vec<syn::Expr>,
    pub module_fns: &'a scan::ModuleFns<'a>,
}

#[atomic_delegates]
impl<'a> ExprRegion<'a> {
    pub fn over(start: syn::Expr, module_fns: &'a scan::ModuleFns<'a>) -> scan::ExprRegion<'a> {}
    pub fn build_uses(&self) -> Vec<String> {}
    pub fn centers_preview(&self) -> bool {}
    pub fn count_states(&self) -> usize {}
    pub fn scene_literals(&self) -> Vec<syn::ExprStruct> {}
    pub fn registered_paths(&self) -> Vec<Vec<String>> {}
    pub(crate) fn local_names(expr: &syn::Expr) -> Vec<String> {}
}

#[cfg(test)]
mod tests {
    use super::ExprRegion;
    use crate::scan;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_str("fn build(app: &mut App) {}").unwrap();
        let module_fns: scan::ModuleFns = file
            .items
            .iter()
            .filter_map(|item| match item {
                syn::Item::Fn(func) => Some((func.sig.ident.to_string(), func)),
                _ => None,
            })
            .collect();
        let start: syn::Expr = syn::parse_str("build(app)").unwrap();
        let region = ExprRegion::over(start, &module_fns);
        assert_eq!(region.exprs.len(), 2);
    }
}
