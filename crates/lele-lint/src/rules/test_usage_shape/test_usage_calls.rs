use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::visit::Visit;

#[derive(Default)]
pub(crate) struct TestUsageCalls {
    pub(crate) call_paths: Vec<Vec<String>>,
    pub(crate) call_idents: Vec<Vec<String>>,
    pub(crate) method_names: Vec<String>,
    pub(crate) method_idents: Vec<Vec<String>>,
    pub(crate) struct_paths: Vec<Vec<String>>,
    pub(crate) struct_idents: Vec<Vec<String>>,
    pub(crate) path_firsts: Vec<String>,
    pub(crate) idents: Vec<String>,
    pub(crate) asserts: Vec<Vec<String>>,
    pub(crate) locals: Vec<(Vec<String>, Self)>,
}

#[derive(derive_more::Deref, derive_more::DerefMut)]
struct CallVisitor<'a>(&'a mut TestUsageCalls);

impl From<&syn::ItemFn> for TestUsageCalls {
    fn from(function: &syn::ItemFn) -> Self {
        let mut calls = Self::default();
        CallVisitor(&mut calls).visit_block(&function.block);
        calls
    }
}

#[atomic_delegate_macros::atomic_delegates]
impl TestUsageCalls {
    pub(crate) fn idents_in(tokens: &TokenStream) -> Vec<String> {}
    pub(crate) fn push_idents(tokens: &TokenStream, idents: &mut Vec<String>) {}
    pub(crate) fn path_segments(path: &syn::Path) -> Vec<String> {}
    pub(crate) fn pattern_bindings(pattern: &syn::Pat, bindings: &mut Vec<String>) {}
}

impl<'ast> Visit<'ast> for CallVisitor<'_> {
    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        let segments = match &*node.func {
            syn::Expr::Path(path) => TestUsageCalls::path_segments(&path.path),
            _ => Vec::new(),
        };
        self.call_paths.push(segments);
        self.call_idents
            .push(TestUsageCalls::idents_in(&node.to_token_stream()));
        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        self.method_names.push(node.method.to_string());
        self.method_idents
            .push(TestUsageCalls::idents_in(&node.to_token_stream()));
        syn::visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_struct(&mut self, node: &'ast syn::ExprStruct) {
        self.struct_paths
            .push(TestUsageCalls::path_segments(&node.path));
        self.struct_idents
            .push(TestUsageCalls::idents_in(&node.to_token_stream()));
        syn::visit::visit_expr_struct(self, node);
    }

    fn visit_expr_path(&mut self, node: &'ast syn::ExprPath) {
        let segments = TestUsageCalls::path_segments(&node.path);
        if let Some(first) = segments.first() {
            self.path_firsts.push(first.clone());
        }
        syn::visit::visit_expr_path(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        let is_assert = node
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident.to_string().starts_with("assert"));
        if is_assert {
            self.asserts.push(TestUsageCalls::idents_in(&node.tokens));
        }
        syn::visit::visit_macro(self, node);
    }

    fn visit_local(&mut self, node: &'ast syn::Local) {
        let mut bindings = Vec::new();
        TestUsageCalls::pattern_bindings(&node.pat, &mut bindings);
        let mut initializer = TestUsageCalls::default();
        if let Some(init) = &node.init {
            CallVisitor(&mut initializer).visit_expr(&init.expr);
            initializer.idents = TestUsageCalls::idents_in(&init.expr.to_token_stream());
        }
        self.locals.push((bindings, initializer));
        syn::visit::visit_local(self, node);
    }
}

// no test_usage necessary
