use lele_snake_case::ToLeleSnakeCase;
use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::spanned::Spanned;

const RESERVED_DELEGATE_METHODS: &[&str] = &["new"];

#[proc_macro_attribute]
pub fn atomic_delegates(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "`#[atomic_delegates]` takes no arguments",
        )
        .to_compile_error()
        .into();
    }
    match expand(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn atomic_delegate(attr: TokenStream, item: TokenStream) -> TokenStream {
    match expand_fn(attr.into(), item.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn expand_fn(
    attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let type_ident: syn::Ident = syn::parse2(attr).map_err(|_| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            "`#[atomic_delegate]` takes a single type name, e.g. `#[atomic_delegate(Foo)]`",
        )
    })?;
    let method: syn::ImplItemFn = syn::parse2(item)?;
    let module = format_ident!("{}", type_ident.to_string().to_snake_case());
    let delegate = expand_method(&module, &method, "", "`#[atomic_delegate]`")?;
    Ok(quote!(#delegate))
}

// needed helper: shared shell-to-dispatch expansion for one method
fn expand_method(
    module: &syn::Ident,
    method: &syn::ImplItemFn,
    body_suffix: &str,
    macro_name: &str,
) -> syn::Result<syn::ImplItemFn> {
    if !method.block.stmts.is_empty() {
        return Err(syn::Error::new(
            method.block.span(),
            format!("{macro_name} method body must be an empty placeholder `{{}}`"),
        ));
    }
    let name = &method.sig.ident;
    if RESERVED_DELEGATE_METHODS.contains(&name.to_string().as_str()) {
        return Err(syn::Error::new(
            name.span(),
            "`new` is a constructor and must be defined in the struct file, not delegated",
        ));
    }
    let mut args: Vec<proc_macro2::TokenStream> = Vec::new();
    for input in &method.sig.inputs {
        match input {
            syn::FnArg::Receiver(_) => args.push(quote!(self)),
            syn::FnArg::Typed(typed) => {
                let syn::Pat::Ident(pat) = &*typed.pat else {
                    return Err(syn::Error::new(
                        typed.pat.span(),
                        format!("{macro_name} parameters must be plain identifiers"),
                    ));
                };
                if pat.by_ref.is_some() || pat.subpat.is_some() {
                    return Err(syn::Error::new(
                        pat.span(),
                        format!("{macro_name} parameters must be plain identifiers"),
                    ));
                }
                let ident = &pat.ident;
                args.push(quote!(#ident));
            }
        }
    }
    let body_fn = format_ident!("{}{}", name, body_suffix);
    let call = quote!(crate::methods::#module::#body_fn(#(#args),*));
    let body = if method.sig.asyncness.is_some() {
        quote!(#call.await)
    } else {
        call
    };
    let mut delegate = method.clone();
    delegate.block = syn::parse_quote!({ #body });
    Ok(delegate)
}

fn expand(item: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let impl_block: syn::ItemImpl = syn::parse2(item)?;

    let type_ident = last_type_ident(&impl_block.self_ty)?;
    let module = format_ident!("{}", type_ident.to_string().to_snake_case());
    let body_suffix = trait_args_suffix(impl_block.trait_.as_ref());

    let mut out = impl_block.clone();
    out.attrs.push(syn::parse_quote!(#[rustfmt::skip]));
    out.items.clear();

    for item in &impl_block.items {
        let method = match item {
            syn::ImplItem::Fn(method) => method,
            syn::ImplItem::Type(_) | syn::ImplItem::Const(_) if impl_block.trait_.is_some() => {
                out.items.push(item.clone());
                continue;
            }
            _ => {
                return Err(syn::Error::new(
                    item.span(),
                    "`#[atomic_delegates]` impl may only contain functions (plus associated types and consts in a trait impl)",
                ));
            }
        };
        let delegate = expand_method(&module, method, &body_suffix, "`#[atomic_delegates]`")?;
        out.items.push(syn::ImplItem::Fn(delegate));
    }

    Ok(quote!(#out))
}

// needed helper: `From<Celsius>` dispatches `from` to `from_celsius`, so two `From` impls never share a body file
fn trait_args_suffix(
    trait_: Option<&(Option<syn::token::Not>, syn::Path, syn::token::For)>,
) -> String {
    fn type_name(ty: &syn::Type) -> Option<String> {
        match ty {
            syn::Type::Reference(reference) => type_name(&reference.elem),
            syn::Type::Paren(paren) => type_name(&paren.elem),
            syn::Type::Slice(slice) => type_name(&slice.elem).map(|elem| format!("{elem}_slice")),
            syn::Type::Path(path) => path
                .path
                .segments
                .last()
                .map(|segment| segment.ident.to_string()),
            _ => None,
        }
    }

    let Some((_, path, _)) = trait_ else {
        return String::new();
    };
    let Some(syn::PathArguments::AngleBracketed(args)) =
        path.segments.last().map(|segment| &segment.arguments)
    else {
        return String::new();
    };
    args.args
        .iter()
        .filter_map(|arg| match arg {
            syn::GenericArgument::Type(ty) => type_name(ty),
            _ => None,
        })
        .map(|name| format!("_{}", name.to_snake_case()))
        .collect::<Vec<_>>()
        .concat()
}

// needed helper: last path segment of an impl self type
fn last_type_ident(ty: &syn::Type) -> syn::Result<syn::Ident> {
    if let syn::Type::Path(type_path) = ty
        && let Some(segment) = type_path.path.segments.last()
    {
        return Ok(segment.ident.clone());
    }
    Err(syn::Error::new(
        ty.span(),
        "`#[atomic_delegates]` requires a named type",
    ))
}

#[cfg(test)]
mod tests {
    use super::{expand, expand_fn};
    use lele_snake_case::ToLeleSnakeCase;
    use quote::quote;

    #[test]
    fn test_usage() {
        let input = quote! {
            impl ClickCounter {
                pub fn increment(&mut self) {}
                pub fn add(&mut self, delta: i32) {}
                pub async fn poll(&mut self) {}
            }
        };
        let expanded = expand(input).unwrap().to_string();
        assert!(expanded.contains("crate :: methods :: click_counter :: increment (self)"));
        assert!(expanded.contains("crate :: methods :: click_counter :: add (self , delta)"));
        assert!(expanded.contains("crate :: methods :: click_counter :: poll (self) . await"));
        assert!(expanded.contains("# [rustfmt :: skip]"));
    }

    #[test]
    fn test_usage_expands_trait_impl() {
        let input = quote! {
            impl Rule for Foo {
                fn check(&self, project: &Project) {}
            }
        };
        let expanded = expand(input).unwrap().to_string();
        assert!(expanded.contains("impl Rule for Foo"));
        assert!(expanded.contains("crate :: methods :: foo :: check (self , project)"));
        assert!(expanded.contains("# [rustfmt :: skip]"));
    }

    #[test]
    fn test_usage_generic_trait_names_body_after_type_args() {
        let celsius = quote! {
            impl From<Celsius> for Temperature {
                fn from(celsius: Celsius) -> Self {}
            }
        };
        let expanded = expand(celsius).unwrap().to_string();
        assert!(expanded.contains("crate :: methods :: temperature :: from_celsius (celsius)"));
        let text = quote! {
            impl<'a> TryFrom<&'a str> for Temperature {
                type Error = Error;
                fn try_from(text: &'a str) -> Result<Self, Self::Error> {}
            }
        };
        let expanded = expand(text).unwrap().to_string();
        assert!(expanded.contains("type Error = Error"));
        assert!(expanded.contains("crate :: methods :: temperature :: try_from_str (text)"));
        assert!(expand(quote! { impl Foo { type Error = Error; } }).is_err());
        let text = quote! {
            impl<'a> TryFrom<&'a [HttpHeader]> for Temperature {
                fn try_from(text: &'a [HttpHeader]) -> Result<Self, Error> {}
            }
        };
        let expanded = expand(text).unwrap().to_string();
        assert!(
            expanded
                .contains("crate :: methods :: temperature :: try_from_http_header_slice (text)")
        );
    }

    #[test]
    fn test_usage_expands_single_fn() {
        let attr = quote!(ConstantsPlacement);
        let input = quote! {
            fn check(&self, project: &Project) -> Vec<RuleViolation> {}
        };
        let expanded = expand_fn(attr, input).unwrap().to_string();
        assert!(
            expanded.contains("crate :: methods :: constants_placement :: check (self , project)")
        );
    }

    #[test]
    fn test_usage_single_fn_rejects_real_body() {
        let attr = quote!(Foo);
        let input = quote! { fn f(&self) { let x = 1; } };
        assert!(expand_fn(attr, input).is_err());
    }

    #[test]
    fn test_usage_single_fn_rejects_reserved_new() {
        let attr = quote!(Foo);
        let input = quote! { fn new() -> Self {} };
        assert!(expand_fn(attr, input).is_err());
    }

    #[test]
    fn test_usage_single_fn_rejects_bad_arg() {
        let attr = quote!(foo::Bar);
        let input = quote! { fn f(&self) {} };
        assert!(expand_fn(attr, input).is_err());
    }

    #[test]
    fn test_usage_rejects_real_body() {
        let input = quote! { impl Foo { fn f(&self) { let x = 1; } } };
        assert!(expand(input).is_err());
    }

    #[test]
    fn test_usage_rejects_nonempty_trait_body() {
        let input =
            quote! { impl Rule for Foo { fn check(&self, project: &Project) { let x = 1; } } };
        assert!(expand(input).is_err());
    }

    #[test]
    fn test_usage_rejects_reserved_new() {
        let input = quote! { impl Foo { pub fn new() -> Self {} } };
        assert!(expand(input).is_err());
    }

    #[test]
    fn test_usage_snake_case() {
        assert_eq!("ClickCounter".to_snake_case(), "click_counter");
        assert_eq!("GamePlugin".to_snake_case(), "game_plugin");
        assert_eq!("HTTPServer".to_snake_case(), "http_server");
        assert_eq!("A".to_snake_case(), "a");
    }
}
