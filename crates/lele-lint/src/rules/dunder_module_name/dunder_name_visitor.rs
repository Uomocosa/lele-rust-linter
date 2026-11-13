use crate::rules;
use crate::DunderKind;
use crate::DunderPath;
use atomic_delegate_macros::atomic_delegate;
use syn::visit::Visit;

pub(crate) struct DunderNameVisitor<'a> {
    pub(crate) dunders: &'a [DunderPath],
    pub(crate) hits: &'a mut Vec<rules::dunder_module_name::DunderNameHit>,
    pub(crate) reexports: &'a mut Vec<rules::dunder_module_name::ReexportHit>,
    pub(crate) domain_prefix: Vec<String>,
    pub(crate) methods_mount_name: String,
}

#[rustfmt::skip]
impl<'ast> Visit<'ast> for DunderNameVisitor<'_> {
    #[atomic_delegate(DunderNameVisitor, Visit)]
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {}
    #[atomic_delegate(DunderNameVisitor, Visit)]
    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {}
    #[atomic_delegate(DunderNameVisitor, Visit)]
    fn visit_path(&mut self, path: &'ast syn::Path) {}
    #[atomic_delegate(DunderNameVisitor, Visit)]
    fn visit_use_path(&mut self, path: &'ast syn::UsePath) {}
    fn visit_use_name(&mut self, name: &'ast syn::UseName) { self.flag_segment(&name.ident); }
    fn visit_use_rename(&mut self, rename: &'ast syn::UseRename) { self.flag_segment(&rename.ident); }
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

    pub(crate) fn code_dunder_mount(&self, segments: &[String]) -> Option<String> {
        let mount = segments.iter().find(|segment| {
            self.dunders.iter().any(|dunder| {
                dunder.is_dunder_dir()
                    && dunder.kind() == DunderKind::Code
                    && dunder.module_name() == segment.as_str()
            })
        });
        mount.cloned()
    }
}

// no test_usage necessary
