use crate::VISUAL_IDENTS;
use crate::scan;
use crate::scan::ui_scanner::UiScanner;
use syn::visit::Visit;

const SPAWN_METHODS: [&str; 4] = ["spawn", "spawn_batch", "insert", "with_child"];

pub fn visit_expr_method_call(this: &mut UiScanner<'_, '_>, node: &syn::ExprMethodCall) {
    if SPAWN_METHODS.contains(&node.method.to_string().as_str()) {
        let mut collected = Vec::new();
        {
            let mut names = scan::PathNames(&mut collected);
            for arg in &node.args {
                names.visit_expr(arg);
                if let Some(bundle) = scan::ScannedFile::called_sibling(arg, this.module_fns) {
                    names.visit_block(&bundle.block);
                }
            }
        }
        if let Some(visual) = collected.iter().find(|name| {
            VISUAL_IDENTS.contains(&name.as_str()) || this.extra.contains(name.as_str())
        }) {
            this.found.push(scan::FoundVisual {
                visual: visual.clone(),
                line: node.method.span().start().line,
                owner: this.owner.clone(),
            });
        }
    }
    syn::visit::visit_expr_method_call(this, node);
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::UiScanner;

    #[test]
    fn test_usage() {
        let mut found = Vec::new();
        let mut owner = String::new();
        let module_fns = std::collections::HashMap::new();
        let extra = std::collections::BTreeSet::new();
        let mut visitor = UiScanner {
            found: &mut found,
            owner: &mut owner,
            module_fns: &module_fns,
            extra: &extra,
        };
        let file: syn::File = syn::parse_str("mod m { pub fn f() {} }\npub(crate) trait T { fn t(&self); }\nuse a::b::c;\nuse a::d as e;\n#[allow(dead_code)]\npub(crate) fn f(x: &u8) -> u8 { let y = g(1, 2); let s = S { a: 1 }; let z = x.to_string(); let w = y.m(); vec![1]; crate::a::b(); let q = w?; self::m(); let u = a::b::c; q }\nimpl X { fn g(&self) {} }\nimpl T for X { fn t(&self) { self.g(); Self::g(self); } }\n").unwrap();
        visitor.visit_file(&file);
    }
}
