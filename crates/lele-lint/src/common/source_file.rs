use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

#[derive(Clone, Copy, Deref)]
pub struct SourceFile<'a>(pub &'a syn::File);

#[atomic_delegates]
impl SourceFile<'_> {
    pub fn primary_type_name(&self, file_stem: &str) -> Option<String> {}
}

#[cfg(test)]
mod tests {
    use super::SourceFile;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_str("pub struct AtomicFile;").unwrap();
        assert_eq!(
            SourceFile(&file).primary_type_name("atomic_file"),
            Some("AtomicFile".to_string())
        );
        assert_eq!(SourceFile(&file).primary_type_name("other"), None);
    }
}
