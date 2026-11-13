use crate::DunderPath;
use std::ffi::OsStr;

pub fn dunder_named<'a>(dunders: &'a [DunderPath], ident: &str) -> Option<&'a DunderPath> {
    dunders
        .iter()
        .find(|dunder| dunder.file_stem() == Some(OsStr::new(ident)))
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::DunderPath;

    #[test]
    fn test_usage() {
        let dunders = DunderPath::defaults();
        assert!(
            rules::dunder_module_name::DunderModuleName::dunder_named(&dunders, "__basic__")
                .is_some()
        );
        assert!(
            rules::dunder_module_name::DunderModuleName::dunder_named(&dunders, "stock").is_none()
        );
    }
}
