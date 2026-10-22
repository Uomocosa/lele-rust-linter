use crate::scan;
use derive_more::{Deref, DerefMut};
use syn::visit::Visit;

use crate::scan::basic::constants::{HARNESS_CRATE, HARNESS_RUN};

#[must_use]
pub fn imports_harness_run(file: &scan::ScannedFile) -> bool {
    let mut found = false;
    let mut visitor = UseFinder(&mut found);
    visitor.visit_file(file);
    found
}

// needed helper: checks every `use` item, test modules included, for the harness `run`
#[derive(Deref, DerefMut)]
struct UseFinder<'a>(&'a mut bool);

impl<'ast> Visit<'ast> for UseFinder<'_> {
    fn visit_item_use(&mut self, node: &'ast syn::ItemUse) {
        if imports_run(&node.tree, false) {
            ***self = true;
        }
    }
}

// needed helper: walks one use tree, remembering whether it is under the harness crate
fn imports_run(tree: &syn::UseTree, is_under_harness: bool) -> bool {
    match tree {
        syn::UseTree::Path(path) => {
            imports_run(&path.tree, is_under_harness || path.ident == HARNESS_CRATE)
        }
        syn::UseTree::Name(name) => is_under_harness && name.ident == HARNESS_RUN,
        syn::UseTree::Rename(rename) => is_under_harness && rename.ident == HARNESS_RUN,
        syn::UseTree::Glob(_) => is_under_harness,
        syn::UseTree::Group(group) => group
            .items
            .iter()
            .any(|item| imports_run(item, is_under_harness)),
    }
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
