use std::path::Path;

pub fn own_domain_of(mod_rs_path: &Path) -> String {
    mod_rs_path
        .components()
        .next()
        .map(|component| component.as_os_str().to_string_lossy().to_string())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use std::path::Path;

    #[test]
    fn test_usage() {
        assert_eq!(
            rules::no_cross_domain_reexport::NoCrossDomainReexport::own_domain_of(Path::new(
                "stock/mod.rs"
            )),
            "stock"
        );
    }
}
