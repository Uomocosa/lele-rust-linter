use crate::scan::basic::constants::HARNESS_CRATE;
use crate::scan::basic::constants::HARNESS_RUN;

pub fn is_harness_path(func: &syn::Expr, is_run_imported: bool) -> bool {
    let syn::Expr::Path(path) = func else {
        return false;
    };
    let segments = &path.path.segments;
    let Some(last) = segments.last() else {
        return false;
    };
    if last.ident != HARNESS_RUN {
        return false;
    }
    if segments.len() == 1 {
        return is_run_imported;
    }
    segments
        .iter()
        .any(|segment| segment.ident == HARNESS_CRATE)
}

#[cfg(test)]
mod tests {
    use super::is_harness_path;
    use crate::scan;

    #[test]
    fn test_usage() {
        let func: syn::Expr = syn::parse_str("lele_bevy_preview::run").unwrap();
        assert!(is_harness_path(&func, false));
        let bare: syn::Expr = syn::parse_str("run").unwrap();
        assert!(is_harness_path(&bare, true));
        assert!(!is_harness_path(&bare, false));
        let _ = scan::HarnessCall::Missing;
    }
}
