use crate::rules;

pub(crate) fn mod_rs_violation_message(code: &str) -> Option<String> {
    let item: syn::Item = syn::parse_str(code).unwrap();
    rules::mod_rs_purity::ModRsPurity::violation_message(&item)
}

#[cfg(test)]
mod tests {
    use super::mod_rs_violation_message;

    #[test]
    fn test_usage() {
        assert!(mod_rs_violation_message("mod player;").is_none());
        assert!(mod_rs_violation_message("use crate::rule;").is_some());
    }
}
