use crate::Category;
use crate::ExampleFile;
use crate::ItemMarker;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;

pub struct ModRsPurity;

impl ModRsPurity {
    pub const NAME: &'static str = "mod_rs_purity";
    pub const CODE: &'static str = "LELE023";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "`mod.rs` holds only `mod`/`pub mod` declarations and `pub use` re-exports.",
        why: "`mod.rs` is the folder's table of contents; logic hidden there is logic nobody looks for.",
        bad: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod greeting;
",
            },
            ExampleFile {
                path: "src/greeting/mod.rs",
                source: r#"use derive_more::Deref;

#[derive(Deref)]
pub struct Guest(pub String);

#[rustfmt::skip]
impl Guest {
    pub fn greet(&self) -> String { format!("Hello, {}!", **self) }
}
"#,
            },
        ],
        good: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod greeting;
",
            },
            ExampleFile {
                path: "src/greeting/mod.rs",
                source: r"mod guest;

pub use guest::Guest;
",
            },
            ExampleFile {
                path: "src/greeting/guest.rs",
                source: r#"use derive_more::Deref;

#[derive(Deref)]
pub struct Guest(pub String);

#[rustfmt::skip]
impl Guest {
    pub fn greet(&self) -> String { format!("Hello, {}!", **self) }
}

#[cfg(test)]
mod tests {
    use super::Guest;

    #[test]
    fn test_usage() {
        assert_eq!(Guest("Ada".to_string()).greet(), "Hello, Ada!");
    }
}
"#,
            },
        ],
    };
}

#[rustfmt::skip]
impl Rule for ModRsPurity {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(ModRsPurity)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl ModRsPurity {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(ModRsPurity));
    }
    pub(crate) fn violation_message(item: &syn::Item) -> Option<String> {
        match item {
            syn::Item::Mod(module) if module.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) => Some(
                "mod.rs may contain only `mod`/`pub mod` declarations and `pub use` re-exports — `#[cfg(test)] mod` test modules are not allowed"
                    .to_string(),
            ),
            syn::Item::Mod(_) => None,
            syn::Item::Use(item_use) => {
                if matches!(item_use.vis, syn::Visibility::Inherited) {
                    Some(
                        "mod.rs may contain only `pub use` re-exports — private `use` imports are not allowed; re-export with `pub use` or move the import into a function file"
                            .to_string(),
                    )
                } else {
                    None
                }
            }
            syn::Item::Struct(s) => Some(Self::named_message("struct", &s.ident.to_string(), &s.vis)),
            syn::Item::Enum(e) => Some(Self::named_message("enum", &e.ident.to_string(), &e.vis)),
            syn::Item::Fn(f) => Some(Self::named_message("fn", &f.sig.ident.to_string(), &f.vis)),
            syn::Item::Const(c) => Some(Self::named_message("const", &c.ident.to_string(), &c.vis)),
            syn::Item::Static(s) => Some(Self::named_message("static", &s.ident.to_string(), &s.vis)),
            syn::Item::Trait(t) => Some(Self::named_message("trait", &t.ident.to_string(), &t.vis)),
            syn::Item::Type(t) => Some(Self::named_message("type alias", &t.ident.to_string(), &t.vis)),
            syn::Item::Impl(_) => Some(
                "mod.rs may contain only `mod`/`pub mod` declarations and `pub use` re-exports — `impl` blocks are not allowed"
                    .to_string(),
            ),
            syn::Item::Macro(_) => Some(
                "mod.rs may contain only `mod`/`pub mod` declarations and `pub use` re-exports — macro definitions are not allowed"
                    .to_string(),
            ),
            _ => Some(
                "mod.rs may contain only `mod`/`pub mod` declarations and `pub use` re-exports — move this item to its own file"
                    .to_string(),
            ),
        }
    }
}

#[atomic_delegates]
impl ModRsPurity {
    pub(crate) fn named_message(kind: &str, name: &str, vis: &syn::Visibility) -> String {}
}

// no test_usage necessary
