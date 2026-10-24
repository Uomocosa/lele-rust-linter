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
    use super::imports_harness_run;
    use crate::scan;

    fn imports(source: &str) -> bool {
        imports_harness_run(&scan::ScannedFile(&syn::parse_str(source).unwrap()))
    }

    #[test]
    fn test_usage() {
        assert!(imports("use lele_bevy_preview::run;"));
        assert!(imports(
            "mod tests { use lele_bevy_preview::{run, scene::Scene}; }"
        ));
        assert!(imports("use lele_bevy_preview::*;"));
        assert!(!imports("use lele_bevy_preview::scene::Scene;"));
        assert!(!imports("use crate::server::run;"));
    }
}
