use syn::spanned::Spanned;

pub fn is_one_line_body(method: &syn::ImplItemFn) -> bool {
    let span = method.block.span();
    span.start().line == span.end().line
}

// no test_usage necessary
