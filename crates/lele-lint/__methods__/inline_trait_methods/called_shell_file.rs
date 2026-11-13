use lele_snake_case::traits::ToLeleSnakeCase;

use crate::ImplBlock;
use crate::ItemMarker;
use crate::Project;

pub fn called_shell_file(
    project: &Project,
    file: &syn::File,
    impl_block: &syn::ItemImpl,
    method: &syn::ImplItemFn,
) -> Option<String> {
    let self_type = ImplBlock(impl_block).self_type_last()?;
    let is_self =
        |expr: &syn::Expr| matches!(expr, syn::Expr::Path(path) if path.path.is_ident("self"));
    let [syn::Stmt::Expr(expr, _)] = method.block.stmts.as_slice() else {
        return None;
    };
    let callee = match expr {
        syn::Expr::MethodCall(call) if is_self(&call.receiver) => call.method.to_string(),
        syn::Expr::Call(call) => {
            let syn::Expr::Path(func) = call.func.as_ref() else {
                return None;
            };
            let segments: Vec<String> = func
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect();
            let [owner, callee] = segments.as_slice() else {
                return None;
            };
            let owner_matches = owner == "Self" || *owner == self_type;
            if !owner_matches || !call.args.first().is_some_and(is_self) {
                return None;
            }
            callee.clone()
        }
        _ => return None,
    };
    file.items.iter().find_map(|item| {
        let syn::Item::Impl(shell_impl) = item else {
            return None;
        };
        let is_atomic = shell_impl
            .attrs
            .iter()
            .any(|attr| ItemMarker(attr).is_atomic_delegates());
        if !is_atomic
            || ImplBlock(shell_impl).self_type_last().as_deref() != Some(self_type.as_str())
        {
            return None;
        }
        shell_impl
            .items
            .iter()
            .find_map(|shell_item| match shell_item {
                syn::ImplItem::Fn(shell)
                    if shell.sig.ident == callee && shell.block.stmts.is_empty() =>
                {
                    Some(format!(
                        "{}/{}/{}.rs",
                        project.methods_root_name(),
                        self_type.to_snake_case(),
                        ImplBlock(shell_impl).delegate_body_name(&shell.sig.ident)
                    ))
                }
                _ => None,
            })
    })
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::Project;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_quote! {
            #[atomic_delegates]
            impl Foo { pub fn build(&self) {} }
            impl Plugin for Foo { fn build(&self) { self.build() } }
        };
        let syn::Item::Impl(trait_impl) = &file.items[1] else {
            panic!("expected an impl block");
        };
        let syn::ImplItem::Fn(method) = &trait_impl.items[0] else {
            panic!("expected a method");
        };
        let found = rules::inline_trait_methods::InlineTraitMethods::called_shell_file(
            &Project::default(),
            &file,
            trait_impl,
            method,
        );
        assert!(found.is_some_and(|path| path.ends_with("/foo/build.rs")));
    }
}
