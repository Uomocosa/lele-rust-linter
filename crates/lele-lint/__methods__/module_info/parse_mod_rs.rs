use crate::ModDecl;
use crate::ModuleInfo;
use crate::Reexport;

pub fn parse_mod_rs(content: &str) -> (Vec<ModDecl>, Vec<Reexport>) {
    let file = match syn::parse_file(content) {
        Ok(f) => f,
        Err(_) => return (Vec::new(), Vec::new()),
    };

    let mut decls = Vec::new();
    let mut reexports = Vec::new();

    for item in file.items {
        match item {
            syn::Item::Mod(m) => {
                decls.push(ModDecl {
                    name: m.ident.to_string(),
                    is_public: matches!(m.vis, syn::Visibility::Public(_)),
                    cfg: ModuleInfo::cfg_attribute(&m.attrs),
                });
            }
            syn::Item::Use(u) => {
                if matches!(u.vis, syn::Visibility::Public(_)) {
                    if let Some(r) = ModuleInfo::extract_reexport(&u.tree) {
                        reexports.push(r);
                    }
                }
            }
            _ => {}
        }
    }

    (decls, reexports)
}

// no test_usage necessary
