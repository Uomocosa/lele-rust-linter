use crate::rules;
use crate::DunderPath;
use syn::visit::Visit;

pub(crate) struct DunderNameVisitor<'a> {
    pub(crate) dunders: &'a [DunderPath],
    pub(crate) hits: &'a mut Vec<rules::dunder_module_name::DunderNameHit>,
}

#[rustfmt::skip]
impl<'ast> Visit<'ast> for DunderNameVisitor<'_> {
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        let ident = item.ident.to_string();
        let target = rules::dunder_module_name::DunderModuleName::path_attribute(&item.attrs);
        if let Some(dunder) = rules::dunder_module_name::DunderModuleName::dunder_named(self.dunders, &ident) {
            let path = target.unwrap_or_else(|| rules::dunder_module_name::DunderModuleName::default_path(dunder));
            self.hits.push(rules::dunder_module_name::DunderNameHit {
                line: item.ident.span().start().line,
                message: format!(
                    "module `{ident}` keeps its dunders — declare `#[path = \"{path}\"] pub mod {};`",
                    dunder.module_name()
                ),
            });
        } else if let Some(target) = target {
            if let Some(dunder) = rules::dunder_module_name::DunderModuleName::dunder_in_path(self.dunders, &target) {
                if ident != dunder.module_name() {
                    self.hits.push(rules::dunder_module_name::DunderNameHit {
                        line: item.ident.span().start().line,
                        message: format!(
                            "`#[path = \"{target}\"]` points at a dunder — name the module `{}`, not `{ident}`",
                            dunder.module_name()
                        ),
                    });
                }
            }
        }
        syn::visit::visit_item_mod(self, item);
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        for segment in &path.segments {
            self.flag_segment(&segment.ident);
        }
        syn::visit::visit_path(self, path);
    }

    fn visit_use_path(&mut self, path: &'ast syn::UsePath) {
        self.flag_segment(&path.ident);
        syn::visit::visit_use_path(self, path);
    }

    fn visit_use_name(&mut self, name: &'ast syn::UseName) {
        self.flag_segment(&name.ident);
    }

    fn visit_use_rename(&mut self, rename: &'ast syn::UseRename) {
        self.flag_segment(&rename.ident);
    }
}

impl DunderNameVisitor<'_> {
    pub(crate) fn flag_segment(&mut self, ident: &syn::Ident) {
        let name = ident.to_string();
        if let Some(dunder) =
            rules::dunder_module_name::DunderModuleName::dunder_named(self.dunders, &name)
        {
            self.hits.push(rules::dunder_module_name::DunderNameHit {
                line: ident.span().start().line,
                message: format!("`{name}` in a path — write `{}`", dunder.module_name()),
            });
        }
    }
}

// no test_usage necessary
