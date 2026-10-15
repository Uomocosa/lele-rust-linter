use std::collections::HashMap;

pub type ModuleFns<'a> = HashMap<String, &'a syn::ItemFn>;
