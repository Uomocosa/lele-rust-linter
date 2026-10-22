use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

use crate::common;

#[derive(Clone, Copy, Deref)]
pub struct RawCode<'a>(pub &'a str);

#[atomic_delegates]
impl RawCode<'_> {
    pub fn find_comments(&self) -> Vec<common::CommentHit> {}
    pub fn is_annotated_as_helper(&self, fn_line: usize) -> bool {}
}

#[cfg(test)]
mod tests {
    use crate::RawCode;

    #[test]
    fn test_usage() {
        let code = RawCode("// needed helper: x\nfn helper() {}\n");
        assert_eq!(code.find_comments().len(), 1);
        assert!(code.is_annotated_as_helper(2));
    }
}
