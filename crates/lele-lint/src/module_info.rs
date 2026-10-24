use std::path::PathBuf;

use atomic_delegate_macros::atomic_delegates;

use crate::Entry;
use crate::ModDecl;
use crate::ModuleInfoMap;
use crate::Reexport;

#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub rel_path: PathBuf,
    pub declarations: Vec<ModDecl>,
    pub reexports: Vec<Reexport>,
}

#[atomic_delegates]
impl ModuleInfo {
    pub fn build(entries: &[Entry]) -> ModuleInfoMap {}
    pub(crate) fn parse_mod_rs(content: &str) -> (Vec<ModDecl>, Vec<Reexport>) {}
}

#[rustfmt::skip]
impl ModuleInfo {
    pub(crate) fn cfg_attribute(attrs: &[syn::Attribute]) -> Option<String> {
        attrs.iter().find_map(|attr| {
            if !attr.path().is_ident("cfg") {
                return None;
            }
            let meta = attr.parse_args::<syn::Meta>().ok()?;
            Some(quote::quote!(#meta).to_string())
        })
    }
    pub(crate) fn extract_reexport(tree: &syn::UseTree) -> Option<Reexport> {
        match tree {
            syn::UseTree::Path(p) => {
                let mut segments = vec![p.ident.to_string()];
                if let Some(child) = Self::extract_reexport(&p.tree) {
                    segments.extend(child.segments);
                    Some(Reexport {
                        segments,
                        is_glob: child.is_glob,
                    })
                } else {
                    None
                }
            }
            syn::UseTree::Name(n) => Some(Reexport {
                segments: vec![n.ident.to_string()],
                is_glob: false,
            }),
            syn::UseTree::Glob(_) => Some(Reexport {
                segments: Vec::new(),
                is_glob: true,
            }),
            syn::UseTree::Rename(r) => Some(Reexport {
                segments: vec![r.ident.to_string()],
                is_glob: false,
            }),
            syn::UseTree::Group(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::Entry;
    use crate::EntryKind;
    use crate::ModuleInfo;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let rel = PathBuf::from("player/mod.rs");
        let abs = dir.path().join("mod.rs");
        std::fs::write(
            &abs,
            "mod player;\nmod player_new;\npub mod bevy_systems;\npub use player::Player;\n",
        )
        .unwrap();
        let entries = vec![Entry {
            relative_path: rel.clone(),
            absolute_path: abs,
            kind: EntryKind::File,
        }];
        let map = ModuleInfo::build(&entries);
        let info = map.get(&rel).unwrap();
        assert_eq!(info.declarations.len(), 3);
        assert_eq!(info.declarations[0].name, "player");
        assert!(!info.declarations[0].is_public);
        assert_eq!(info.declarations[2].name, "bevy_systems");
        assert!(info.declarations[2].is_public);
        assert_eq!(info.reexports.len(), 1);
        assert_eq!(info.reexports[0].segments, vec!["player", "Player"]);
    }
}
