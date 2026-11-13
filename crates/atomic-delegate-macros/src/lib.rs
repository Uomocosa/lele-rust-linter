use lele_snake_case::DelegateFile;
use lele_snake_case::traits::ToLeleSnakeCase;
use proc_macro::TokenStream;
use quote::ToTokens;
use quote::format_ident;
use quote::quote;
use syn::spanned::Spanned;

const RESERVED_DELEGATE_METHODS: &[&str] = &["new"];

#[proc_macro_attribute]
pub fn atomic_delegates(attr: TokenStream, item: TokenStream) -> TokenStream {
    if attr.is_empty() {
        DelegateImpl::expanded(item.into())
            .unwrap_or_else(|error| error.to_compile_error())
            .into()
    } else {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            "`#[atomic_delegates]` takes no arguments",
        )
        .to_compile_error()
        .into()
    }
}

#[proc_macro_attribute]
pub fn atomic_delegate(attr: TokenStream, item: TokenStream) -> TokenStream {
    DelegateShell::expansion(attr.into(), item.into())
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}

pub(crate) struct DelegateImpl {
    impl_block: syn::ItemImpl,
    module: syn::Ident,
    body_suffix: String,
    trait_ident: Option<String>,
}

impl TryFrom<syn::ItemImpl> for DelegateImpl {
    type Error = syn::Error;

    fn try_from(impl_block: syn::ItemImpl) -> Result<Self, Self::Error> {
        let module = Self::module_ident(&impl_block)?;
        let body_suffix = Self::trait_args_suffix(impl_block.trait_.as_ref());
        let trait_ident = Self::trait_ident(impl_block.trait_.as_ref());
        Ok(Self {
            impl_block,
            module,
            body_suffix,
            trait_ident,
        })
    }
}

impl DelegateImpl {
    pub fn expanded(tokens: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
        let impl_block: syn::ItemImpl = syn::parse2(tokens)?;
        Self::try_from(impl_block)?.into_tokens()
    }

    pub(crate) fn module_ident(impl_block: &syn::ItemImpl) -> syn::Result<syn::Ident> {
        let type_ident = Self::self_type_ident(&impl_block.self_ty)?;
        Ok(format_ident!("{}", type_ident.to_string().to_snake_case()))
    }

    pub(crate) fn self_type_ident(self_type: &syn::Type) -> syn::Result<syn::Ident> {
        if let syn::Type::Path(type_path) = self_type
            && let Some(segment) = type_path.path.segments.last()
        {
            return Ok(segment.ident.clone());
        }
        Err(syn::Error::new(
            self_type.span(),
            "`#[atomic_delegates]` requires a named type",
        ))
    }

    pub(crate) fn trait_ident(
        trait_path: Option<&(Option<syn::token::Not>, syn::Path, syn::token::For)>,
    ) -> Option<String> {
        let (_, path, _) = trait_path?;
        path.segments
            .last()
            .map(|segment| segment.ident.to_string())
    }

    pub(crate) fn trait_args_suffix(
        trait_path: Option<&(Option<syn::token::Not>, syn::Path, syn::token::For)>,
    ) -> String {
        let Some((_, path, _)) = trait_path else {
            return String::new();
        };
        let Some(syn::PathArguments::AngleBracketed(args)) =
            path.segments.last().map(|segment| &segment.arguments)
        else {
            return String::new();
        };
        Self::suffix_from_names(&Self::arg_names(args))
    }

    pub(crate) fn arg_names(args: &syn::AngleBracketedGenericArguments) -> Vec<String> {
        args.args
            .iter()
            .filter_map(|arg| match arg {
                syn::GenericArgument::Type(ty) => Self::type_name(ty),
                _ => None,
            })
            .collect()
    }

    pub(crate) fn type_name(self_type: &syn::Type) -> Option<String> {
        match self_type {
            syn::Type::Reference(reference) => Self::type_name(&reference.elem),
            syn::Type::Paren(paren) => Self::type_name(&paren.elem),
            syn::Type::Slice(slice) => {
                Self::type_name(&slice.elem).map(|elem| format!("{elem}_slice"))
            }
            syn::Type::Path(path) => path
                .path
                .segments
                .last()
                .map(|segment| segment.ident.to_string()),
            _ => None,
        }
    }

    pub(crate) fn suffix_from_names(type_names: &[String]) -> String {
        type_names
            .iter()
            .map(|name| format!("_{}", name.to_snake_case()))
            .collect::<Vec<_>>()
            .concat()
    }

    pub(crate) fn into_tokens(mut self) -> syn::Result<proc_macro2::TokenStream> {
        self.impl_block
            .attrs
            .push(syn::parse_quote!(#[rustfmt::skip]));
        let replaced = self.replaced_items()?;
        let impl_block = syn::ItemImpl {
            items: replaced,
            ..self.impl_block
        };
        Ok(quote!(#impl_block))
    }

    pub(crate) fn replaced_items(&self) -> syn::Result<Vec<syn::ImplItem>> {
        let mut replacement = Vec::new();
        for item in &self.impl_block.items {
            replacement.extend(self.replaced_item(item)?);
        }
        Ok(replacement)
    }

    pub(crate) fn replaced_item(&self, item: &syn::ImplItem) -> syn::Result<Vec<syn::ImplItem>> {
        let method = match item {
            syn::ImplItem::Fn(method) => method,
            syn::ImplItem::Type(_) | syn::ImplItem::Const(_)
                if self.impl_block.trait_.is_some() =>
            {
                return Ok(vec![item.clone()]);
            }
            _ => {
                return Err(syn::Error::new(
                    item.span(),
                    "`#[atomic_delegates]` impl may only contain functions (plus associated types and consts in a trait impl)",
                ));
            }
        };
        Ok(vec![syn::ImplItem::Fn(
            DelegateShell {
                module: self.module.clone(),
                method: method.clone(),
                body_suffix: self.body_suffix.clone(),
                trait_ident: self.trait_ident.clone(),
                macro_name: "`#[atomic_delegates]`",
            }
            .expanded()?,
        )])
    }
}

pub(crate) struct DelegateShell {
    module: syn::Ident,
    method: syn::ImplItemFn,
    body_suffix: String,
    trait_ident: Option<String>,
    macro_name: &'static str,
}

impl DelegateShell {
    pub fn try_from_tokens(
        attr_tokens: proc_macro2::TokenStream,
        item_tokens: proc_macro2::TokenStream,
    ) -> syn::Result<Self> {
        let (type_ident, trait_ident) = Self::attr_args(attr_tokens)?;
        Ok(Self {
            module: format_ident!("{}", type_ident.to_string().to_snake_case()),
            method: syn::parse2(item_tokens)?,
            body_suffix: String::new(),
            trait_ident: trait_ident.map(|ident| ident.to_string()),
            macro_name: "`#[atomic_delegate]`",
        })
    }

    pub fn attr_args(
        attr_tokens: proc_macro2::TokenStream,
    ) -> syn::Result<(syn::Ident, Option<syn::Ident>)> {
        syn::parse::Parser::parse2(Self::parse_attr_args, attr_tokens).map_err(|_| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "`#[atomic_delegate]` takes a type name, optionally followed by its trait: `#[atomic_delegate(Foo)]` or `#[atomic_delegate(Foo, Rule)]`",
            )
        })
    }

    pub fn expansion(
        attr_tokens: proc_macro2::TokenStream,
        item_tokens: proc_macro2::TokenStream,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let delegate = Self::try_from_tokens(attr_tokens, item_tokens)?.expanded()?;
        Ok(quote!(#delegate))
    }

    pub fn expanded(self) -> syn::Result<syn::ImplItemFn> {
        if !self.method.block.stmts.is_empty() {
            return Err(syn::Error::new(
                self.method.block.span(),
                format!(
                    "{} method body must be an empty placeholder `{{}}`",
                    self.macro_name
                ),
            ));
        }
        self.reserved_violation()?;
        self.dispatching()
    }

    pub(crate) fn parse_attr_args(
        input: syn::parse::ParseStream,
    ) -> syn::Result<(syn::Ident, Option<syn::Ident>)> {
        let type_ident: syn::Ident = input.parse()?;
        if input.is_empty() {
            return Ok((type_ident, None));
        }
        input.parse::<syn::Token![,]>()?;
        let trait_ident: syn::Ident = input.parse()?;
        Ok((type_ident, Some(trait_ident)))
    }

    pub(crate) fn reserved_violation(&self) -> syn::Result<()> {
        if RESERVED_DELEGATE_METHODS.contains(&self.method.sig.ident.to_string().as_str()) {
            return Err(syn::Error::new(
                self.method.sig.ident.span(),
                "`new` is a constructor and must be defined in the struct file, not delegated",
            ));
        }
        Ok(())
    }

    pub(crate) fn dispatching(&self) -> syn::Result<syn::ImplItemFn> {
        let args = self.argument_tokens()?;
        let call = self.call_tokens(&args);
        let mut delegate = self.method.clone();
        delegate.block = syn::parse_quote!({ #call });
        Ok(delegate)
    }

    pub(crate) fn argument_tokens(&self) -> syn::Result<Vec<proc_macro2::TokenStream>> {
        let mut args = Vec::new();
        for input in &self.method.sig.inputs {
            args.push(Self::argument(input, self.macro_name)?);
        }
        Ok(args)
    }

    pub(crate) fn argument(
        input: &syn::FnArg,
        macro_name: &str,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let syn::FnArg::Typed(pattern) = input else {
            return Ok(quote!(self));
        };
        Self::pattern_tokens(pattern, macro_name)
    }

    pub(crate) fn pattern_tokens(
        pattern: &syn::PatType,
        macro_name: &str,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let syn::Pat::Ident(ident) = &*pattern.pat else {
            return Err(syn::Error::new(
                pattern.pat.span(),
                format!("{macro_name} parameters must be plain identifiers"),
            ));
        };
        if ident.by_ref.is_some() || ident.subpat.is_some() {
            return Err(syn::Error::new(
                ident.span(),
                format!("{macro_name} parameters must be plain identifiers"),
            ));
        }
        let name_ident = &ident.ident;
        Ok(quote!(#name_ident))
    }

    pub(crate) fn generic_tokens(&self) -> Vec<proc_macro2::TokenStream> {
        self.method
            .sig
            .generics
            .params
            .iter()
            .filter_map(|param| match param {
                syn::GenericParam::Type(ty) => Some(ty.ident.to_token_stream()),
                syn::GenericParam::Const(c) => Some(c.ident.to_token_stream()),
                syn::GenericParam::Lifetime(_) => None,
            })
            .collect()
    }

    pub(crate) fn call_tokens(
        &self,
        args: &[proc_macro2::TokenStream],
    ) -> proc_macro2::TokenStream {
        let call = Self::named_call(self, args);
        if self.method.sig.asyncness.is_some() {
            quote!(#call.await)
        } else {
            call
        }
    }

    pub(crate) fn named_call(&self, args: &[proc_macro2::TokenStream]) -> proc_macro2::TokenStream {
        let turbofish = Self::turbofish(&self.generic_tokens());
        let module = &self.module;
        let method = self.method.sig.ident.to_string();
        let body_fn = format_ident!(
            "{}",
            DelegateFile::stem(self.trait_ident.as_deref(), &method, &self.body_suffix)
        );
        quote!(crate::methods::#module::#body_fn #turbofish (#(#args),*))
    }

    pub(crate) fn turbofish(generics: &[proc_macro2::TokenStream]) -> proc_macro2::TokenStream {
        if generics.is_empty() {
            quote!()
        } else {
            quote!(::<#(#generics),*>)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DelegateImpl, DelegateShell};
    use lele_snake_case::traits::ToLeleSnakeCase;
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
        let expanded = DelegateImpl::expanded(input).unwrap().to_string();
        assert!(expanded.contains("crate :: methods :: click_counter :: increment (self)"));
        assert!(expanded.contains("crate :: methods :: click_counter :: add (self , delta)"));
        assert!(expanded.contains("crate :: methods :: click_counter :: poll (self) . await"));
        assert!(expanded.contains("# [rustfmt :: skip]"));
    }

    #[test]
    fn test_usage_forwards_method_generics() {
        let input = quote! {
            impl SuiteConfig {
                pub fn check<'a, C: ContractInterface, const N: usize>(&'a self, x: &'a [u8; N]) {}
            }
        };
        let expanded = DelegateImpl::expanded(input).unwrap().to_string();
        assert!(
            expanded.contains("crate :: methods :: suite_config :: check :: < C , N > (self , x)")
        );
    }

    #[test]
    fn test_usage_expands_trait_impl() {
        let input = quote! {
            impl Rule for Foo {
                fn check(&self, project: &Project) {}
            }
        };
        let expanded = DelegateImpl::expanded(input).unwrap().to_string();
        assert!(expanded.contains("impl Rule for Foo"));
        assert!(expanded.contains("crate :: methods :: foo :: rule_check (self , project)"));
        assert!(expanded.contains("# [rustfmt :: skip]"));
        let display = quote! {
            impl std::fmt::Display for Foo {
                fn fmt(&self, formatter: &mut Formatter) -> Result {}
            }
        };
        let expanded = DelegateImpl::expanded(display).unwrap().to_string();
        assert!(expanded.contains("crate :: methods :: foo :: display_fmt (self , formatter)"));
    }

    #[test]
    fn test_usage_generic_trait_names_body_after_type_args() {
        let celsius = quote! {
            impl From<Celsius> for Temperature {
                fn from(celsius: Celsius) -> Self {}
            }
        };
        let expanded = DelegateImpl::expanded(celsius).unwrap().to_string();
        assert!(expanded.contains("crate :: methods :: temperature :: from_celsius (celsius)"));
        let text = quote! {
            impl<'a> TryFrom<&'a str> for Temperature {
                type Error = Error;
                fn try_from(text: &'a str) -> Result<Self, Self::Error> {}
            }
        };
        let expanded = DelegateImpl::expanded(text).unwrap().to_string();
        assert!(expanded.contains("type Error = Error"));
        assert!(expanded.contains("crate :: methods :: temperature :: try_from_str (text)"));
        assert!(DelegateImpl::expanded(quote! { impl Foo { type Error = Error; } }).is_err());
        let text = quote! {
            impl<'a> TryFrom<&'a [HttpHeader]> for Temperature {
                fn try_from(text: &'a [HttpHeader]) -> Result<Self, Error> {}
            }
        };
        let expanded = DelegateImpl::expanded(text).unwrap().to_string();
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
        let expanded = DelegateShell::expansion(attr, input).unwrap().to_string();
        assert!(
            expanded.contains("crate :: methods :: constants_placement :: check (self , project)")
        );
    }

    #[test]
    fn test_usage_single_fn_trait_arg_names_body() {
        let input = quote! {
            fn check(&self, project: &Project) -> Vec<RuleViolation> {}
        };
        let with_trait = DelegateShell::expansion(quote!(Foo, Rule), input.clone())
            .unwrap()
            .to_string();
        assert!(with_trait.contains("crate :: methods :: foo :: rule_check (self , project)"));
        let without_trait = DelegateShell::expansion(quote!(Foo), input.clone())
            .unwrap()
            .to_string();
        assert!(without_trait.contains("crate :: methods :: foo :: check (self , project)"));
        assert!(DelegateShell::expansion(quote!(Foo, Rule, Extra), input).is_err());
    }

    #[test]
    fn test_usage_single_fn_rejects_real_body() {
        let attr = quote!(Foo);
        let input = quote! { fn f(&self) { let x = 1; } };
        assert!(DelegateShell::expansion(attr, input).is_err());
    }

    #[test]
    fn test_usage_single_fn_rejects_reserved_new() {
        let attr = quote!(Foo);
        let input = quote! { fn new() -> Self {} };
        assert!(DelegateShell::expansion(attr, input).is_err());
    }

    #[test]
    fn test_usage_single_fn_rejects_bad_arg() {
        let attr = quote!(foo::Bar);
        let input = quote! { fn f(&self) {} };
        assert!(DelegateShell::expansion(attr, input).is_err());
    }

    #[test]
    fn test_usage_rejects_real_body() {
        let input = quote! { impl Foo { fn f(&self) { let x = 1; } } };
        assert!(DelegateImpl::expanded(input).is_err());
    }

    #[test]
    fn test_usage_rejects_nonempty_trait_body() {
        let input =
            quote! { impl Rule for Foo { fn check(&self, project: &Project) { let x = 1; } } };
        assert!(DelegateImpl::expanded(input).is_err());
    }

    #[test]
    fn test_usage_rejects_reserved_new() {
        let input = quote! { impl Foo { pub fn new() -> Self {} } };
        assert!(DelegateImpl::expanded(input).is_err());
    }

    #[test]
    fn test_usage_snake_case() {
        assert_eq!("ClickCounter".to_snake_case(), "click_counter");
        assert_eq!("GamePlugin".to_snake_case(), "game_plugin");
        assert_eq!("HTTPServer".to_snake_case(), "http_server");
        assert_eq!("A".to_snake_case(), "a");
    }
}
