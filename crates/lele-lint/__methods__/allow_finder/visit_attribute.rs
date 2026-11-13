use crate::rules;
use crate::rules::no_allow_attributes::allow_finder::AllowFinder;
use crate::RuleViolation;
use syn::spanned::Spanned;

pub fn visit_attribute(this: &mut AllowFinder<'_>, attr: &syn::Attribute) {
    let kind = if attr.path().is_ident("allow") {
        "allow"
    } else if attr.path().is_ident("expect") {
        "expect"
    } else {
        return;
    };
    let lint_paths = rules::no_allow_attributes::NoAllowAttributes::lint_paths(attr);
    if this.is_whitelisted(&lint_paths) {
        return;
    }
    let start = attr.span().start();
    this.violations.push(RuleViolation::from_site(this.file.clone(), start.line, start.column, rules::no_allow_attributes::NoAllowAttributes::CODE, format!(
            "`{kind}` attribute is banned — remove it and fix the code the lint points at; a `[[lele.lint.clippy_allow_whitelist]]` entry in lele.toml needs the user's approval"
        )));
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::AllowFinder;

    #[test]
    fn test_usage() {
        let mut visitor = AllowFinder {
            file: std::path::PathBuf::new(),
            config_base: std::path::PathBuf::new(),
            clippy_allow_whitelist: &[],
            violations: Vec::new(),
        };
        let file: syn::File = syn::parse_str("mod m { pub fn f() {} }\npub(crate) trait T { fn t(&self); }\nuse a::b::c;\nuse a::d as e;\n#[allow(dead_code)]\npub(crate) fn f(x: &u8) -> u8 { let y = g(1, 2); let s = S { a: 1 }; let z = x.to_string(); let w = y.m(); vec![1]; crate::a::b(); let q = w?; self::m(); let u = a::b::c; q }\nimpl X { fn g(&self) {} }\nimpl T for X { fn t(&self) { self.g(); Self::g(self); } }\n").unwrap();
        visitor.visit_file(&file);
    }
}
