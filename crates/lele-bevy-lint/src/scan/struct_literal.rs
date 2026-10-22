use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

#[derive(Clone, Copy, Deref)]
pub struct StructLiteral<'a>(pub &'a syn::ExprStruct);

#[atomic_delegates]
impl StructLiteral<'_> {
    pub fn is_type(&self, type_name: &str) -> bool {}
    pub fn has_field(&self, field: &str) -> bool {}
}

#[cfg(test)]
mod tests {
    use super::StructLiteral;

    #[test]
    fn test_usage() {
        let literal: syn::ExprStruct = syn::parse_str("scene::State { label, apply }").unwrap();
        assert!(StructLiteral(&literal).is_type("State"));
        assert!(StructLiteral(&literal).has_field("apply"));
        assert!(!StructLiteral(&literal).has_field("other"));
    }
}
