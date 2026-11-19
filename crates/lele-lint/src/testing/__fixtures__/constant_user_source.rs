pub(crate) fn constant_user_source(const_name: &str) -> String {
    format!(
        "use crate::basic::constants::{const_name};\n\npub trait User {{ fn call(&self) -> String {{ crate::basic::constants::{const_name}.len().to_string() }} }}\n"
    )
}

#[cfg(test)]
mod tests {
    use super::constant_user_source;

    #[test]
    fn test_usage() {
        assert!(constant_user_source("ACRONYMS").contains("ACRONYMS"));
    }
}
