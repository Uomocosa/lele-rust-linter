use crate::rules;
use std::path::PathBuf;
use syn::visit::Visit;

pub fn file_use(
    file: PathBuf,
    crate_rel: PathBuf,
    parsed: &syn::File,
) -> rules::boundary_imports::FileUse {
    let mut uses = rules::boundary_imports::UseCollector::default();
    uses.visit_file(parsed);
    let mut paths = rules::boundary_imports::PathCollector {
        aliases: uses.aliases.clone(),
        hits: Vec::new(),
    };
    paths.visit_file(parsed);
    let mut hits = uses.hits;
    hits.extend(paths.hits);
    rules::boundary_imports::FileUse {
        file,
        crate_rel,
        hits,
    }
}

// no test_usage necessary
