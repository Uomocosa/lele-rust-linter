use syn::visit::Visit;

use crate::scan;

#[must_use]
pub fn imports_harness_run(file: &scan::ScannedFile) -> bool {
    let mut found = false;
    let mut visitor = scan::UseFinder(&mut found);
    visitor.visit_file(file);
    found
}

#[cfg(test)]
mod tests {
    use crate::scan;
    use crate::testing;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_str("use lele_bevy_preview::run;").unwrap();
        let imported = super::imports_harness_run(&scan::ScannedFile(&file));
        assert!(imported);
    }

    #[test]
    fn test_usage_detects_the_harness_import() {
        assert!(testing::harness_run_imported("use lele_bevy_preview::run;"));
        assert!(testing::harness_run_imported(
            "mod tests { use lele_bevy_preview::{run, scene::Scene}; }"
        ));
        assert!(testing::harness_run_imported("use lele_bevy_preview::*;"));
        assert!(!testing::harness_run_imported(
            "use lele_bevy_preview::scene::Scene;"
        ));
        assert!(!testing::harness_run_imported("use crate::server::run;"));
    }
}
