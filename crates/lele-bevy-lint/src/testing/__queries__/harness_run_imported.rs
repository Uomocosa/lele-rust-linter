use crate::methods;
use crate::scan;

pub(crate) fn harness_run_imported(source: &str) -> bool {
    methods::scanned_file::imports_harness_run(&scan::ScannedFile(&syn::parse_str(source).unwrap()))
}

#[cfg(test)]
mod tests {
    use super::harness_run_imported;

    #[test]
    fn test_usage() {
        assert!(harness_run_imported("use lele_bevy_preview::run;"));
    }
}
