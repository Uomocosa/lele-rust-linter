use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

#[derive(Clone, Copy, Deref)]
pub struct ScannedItem<'a>(pub &'a syn::Item);

#[atomic_delegates]
impl ScannedItem<'_> {
    pub fn is_test_module(&self) -> bool {}
}

#[cfg(test)]
mod tests {
    use super::ScannedItem;

    #[test]
    fn test_usage() {
        let item: syn::Item = syn::parse_str("#[cfg(test)] mod tests {}").unwrap();
        assert!(ScannedItem(&item).is_test_module());
        let plain: syn::Item = syn::parse_str("mod inner {}").unwrap();
        assert!(!ScannedItem(&plain).is_test_module());
    }
}
