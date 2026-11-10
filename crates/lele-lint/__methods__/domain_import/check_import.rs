use crate::rules;
use crate::DunderPath;
use crate::Identifier;

pub fn check_import(item_use: &syn::ItemUse, dunder_paths: &[DunderPath]) -> Option<String> {
    let segments = rules::domain_import::DomainImport::collect_use_segments(&item_use.tree);
    let first = segments.first()?;

    if first == "crate" && segments.len() >= 3 {
        if segments.iter().any(|segment| {
            dunder_paths
                .iter()
                .any(|dunder| dunder.is_dunder_dir() && dunder.module_name() == segment)
        }) {
            return None;
        }
        if let [.., prev, last] = &segments[..] {
            if Identifier(last).is_stuttered_in(prev) {
                return None;
            }
        }
        let direct = segments.join("::");
        let second = segments.get(1)?;
        if !rules::domain_import::DomainImport::is_pub_use(item_use) {
            return Some(format!(
                "`use {direct};` imports the item — write `use crate::{second};` and qualify the item with it"
            ));
        }
    }

    None
}

// no test_usage necessary
