use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

use crate::common;

#[derive(Clone, Copy, Deref)]
pub struct RawCode<'a>(pub &'a str);

#[atomic_delegates]
impl RawCode<'_> {
    pub fn find_comments(&self) -> Vec<common::CommentHit> {}
    pub fn ends_with_test_usage_opt_out(&self) -> bool {}
}

#[atomic_delegates]
impl RawCode<'_> {
    pub(crate) fn line_of(source: &str, offset: usize) -> usize {}
}

#[cfg(test)]
mod tests {
    use crate::RawCode;

    #[test]
    fn test_usage() {
        let code = RawCode("// a comment\nfn helper() {}\n// no test_usage necessary\n");
        assert_eq!(code.find_comments().len(), 2);
        assert!(code.ends_with_test_usage_opt_out());
    }
}
