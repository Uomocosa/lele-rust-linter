use std::collections::BTreeMap;
use std::collections::BTreeSet;

use atomic_delegate_macros::atomic_delegates;

#[derive(Debug, Clone, Default)]
pub struct DeclaredType {
    pub methods: BTreeSet<String>,
    pub cfgs: Vec<String>,
}

#[atomic_delegates]
impl DeclaredType {
    pub fn index_content(&self) -> String {}
    pub fn root_index_content(types: &BTreeMap<String, Self>) -> String {}
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::common::DeclaredType;

    #[test]
    fn test_usage() {
        let declared = DeclaredType {
            methods: ["add".to_string()].into(),
            cfgs: Vec::new(),
        };
        assert_eq!(declared.index_content(), "mod add;\npub use add::add;\n");
        let types: BTreeMap<String, DeclaredType> = [("adder".to_string(), declared)].into();
        assert_eq!(DeclaredType::root_index_content(&types), "pub mod adder;\n");
    }
}
