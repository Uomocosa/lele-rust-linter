use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;

pub struct ConstructorNaming;

impl ConstructorNaming {
    pub const NAME: &'static str = "constructor_naming";
    pub const CODE: &'static str = "LELE037";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Types,
        summary: "An associated fn with two or more arguments and no `self` that returns `Self` is named `from_*`; one returning `Result<Self, _>` or `Option<Self>` is named `try_from_*`. `new`, trait impls and fns taking a `Self` argument (merges) are exempt.",
        why: "Every constructor reads the same way at the call site: `from_` says it builds the type, `try_` says it can fail.",
        bad: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod reading;
",
            },
            ExampleFile {
                path: "src/reading/mod.rs",
                source: r"mod reading;

pub use reading::Reading;
",
            },
            ExampleFile {
                path: "src/reading/reading.rs",
                source: r#"pub struct Reading {
    pub sensor: String,
    pub celsius: i32,
}

impl Reading {
    pub fn at(sensor: &str, celsius: i32) -> Self {
        Self {
            sensor: sensor.to_string(),
            celsius,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::reading;

    #[test]
    fn test_usage() {
        let reading = reading::Reading::at("kitchen", 21);
        assert_eq!(reading.celsius, 21);
    }
}
"#,
            },
        ],
        good: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod reading;
",
            },
            ExampleFile {
                path: "src/reading/mod.rs",
                source: r"mod reading;

pub use reading::Reading;
",
            },
            ExampleFile {
                path: "src/reading/reading.rs",
                source: r#"pub struct Reading {
    pub sensor: String,
    pub celsius: i32,
}

impl Reading {
    pub fn from_sensor_and_celsius(sensor: &str, celsius: i32) -> Self {
        Self {
            sensor: sensor.to_string(),
            celsius,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::reading;

    #[test]
    fn test_usage() {
        let reading = reading::Reading::from_sensor_and_celsius("kitchen", 21);
        assert_eq!(reading.celsius, 21);
    }
}
"#,
            },
        ],
    };
}

#[rustfmt::skip]
impl Rule for ConstructorNaming {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(ConstructorNaming)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl ConstructorNaming {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(ConstructorNaming));
    }
    pub(crate) fn takes_self_typed_argument(sig: &syn::Signature, self_name: &str) -> bool {
        sig.inputs.iter().any(|input| {
            let syn::FnArg::Typed(typed) = input else {
                return false;
            };
            quote::ToTokens::to_token_stream(&*typed.ty)
                .into_iter()
                .any(|token| matches!(token, proc_macro2::TokenTree::Ident(ident) if ident == "Self" || ident == self_name))
        })
    }
    pub(crate) fn is_self(ty: &syn::Type, self_name: &str) -> bool { matches!(ty, syn::Type::Path(path) if path.qself.is_none() && path.path.segments.len() == 1 && path.path.segments.last().is_some_and(|segment| { segment.arguments.is_none() && (segment.ident == "Self" || segment.ident == self_name) })) }
}

#[atomic_delegates]
impl ConstructorNaming {
    pub(crate) fn advice(sig: &syn::Signature, self_name: &str, shape: &str) -> Option<String> {}
    pub(crate) fn constructed_shape(
        output: &syn::ReturnType,
        self_name: &str,
    ) -> Option<&'static str> {
    }
}

// no test_usage necessary
