use atomic_delegate_macros::atomic_delegates;

#[derive(Default)]
pub struct Tree {
    pub value: u32,
    pub children: Vec<Tree>,
}

#[atomic_delegates]
impl Tree {
    pub fn total(&self) -> u32 {}
}

#[cfg(test)]
mod tests {
    use crate::tree;

    #[test]
    fn test_usage() {
        assert_eq!(tree::Tree::default().total(), 0);
    }
}
