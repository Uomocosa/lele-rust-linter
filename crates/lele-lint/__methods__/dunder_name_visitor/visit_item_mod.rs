use crate::rules;
use crate::rules::dunder_module_name::dunder_name_visitor::DunderNameVisitor;

pub fn visit_item_mod(this: &mut DunderNameVisitor<'_>, item: &syn::ItemMod) {
    let ident = item.ident.to_string();
    let target = rules::dunder_module_name::DunderModuleName::path_attribute(&item.attrs);
    if let Some(dunder) =
        rules::dunder_module_name::DunderModuleName::dunder_named(this.dunders, &ident)
    {
        let path = target
            .unwrap_or_else(|| rules::dunder_module_name::DunderModuleName::default_path(dunder));
        this.hits.push(rules::dunder_module_name::DunderNameHit {
            line: item.ident.span().start().line,
            message: format!(
                "module `{ident}` keeps its dunders — declare `#[path = \"{path}\"] pub mod {};`",
                dunder.module_name()
            ),
        });
    } else if let Some(target) = target {
        if let Some(dunder) =
            rules::dunder_module_name::DunderModuleName::dunder_in_path(this.dunders, &target)
        {
            if ident != dunder.module_name() {
                this.hits.push(rules::dunder_module_name::DunderNameHit {
                    line: item.ident.span().start().line,
                    message: format!(
                        "`#[path = \"{target}\"]` points at a dunder — name the module `{}`, not `{ident}`",
                        dunder.module_name()
                    ),
                });
            }
        }
    }
    if let Some(message) = rules::dunder_module_name::DunderModuleName::mount_visibility_hit(
        this.dunders,
        &this.methods_mount_name,
        &ident,
        &item.vis,
    ) {
        this.hits.push(rules::dunder_module_name::DunderNameHit {
            line: item.ident.span().start().line,
            message,
        });
    }
    syn::visit::visit_item_mod(this, item);
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::DunderNameVisitor;

    #[test]
    fn test_usage() {
        let mut hits = Vec::new();
        let mut reexports = Vec::new();
        let mut visitor = DunderNameVisitor {
            dunders: &[],
            hits: &mut hits,
            reexports: &mut reexports,
            domain_prefix: Vec::new(),
            methods_mount_name: String::new(),
        };
        let file: syn::File = syn::parse_str("mod m { pub fn f() {} }\npub(crate) trait T { fn t(&self); }\nuse a::b::c;\nuse a::d as e;\n#[allow(dead_code)]\npub(crate) fn f(x: &u8) -> u8 { let y = g(1, 2); let s = S { a: 1 }; let z = x.to_string(); let w = y.m(); vec![1]; crate::a::b(); let q = w?; self::m(); let u = a::b::c; q }\nimpl X { fn g(&self) {} }\nimpl T for X { fn t(&self) { self.g(); Self::g(self); } }\n").unwrap();
        visitor.visit_file(&file);
    }
}
