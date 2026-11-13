use crate::scan;

pub fn visit_path(full_paths: &mut scan::full_paths::FullPaths<'_, '_>, node: &syn::Path) {
    let segments = node
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect();
    full_paths.push(segments);
    syn::visit::visit_path(full_paths, node);
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use crate::scan::full_paths::FullPaths;
    use crate::scan::path_collector::PathCollector;

    #[test]
    fn test_usage() {
        let mut paths = Vec::new();
        let mut collector = PathCollector(&mut paths);
        let mut visitor = FullPaths(&mut collector);
        let file: syn::File = syn::parse_str("mod m { pub fn f() {} }\npub(crate) trait T { fn t(&self); }\nuse a::b::c;\nuse a::d as e;\n#[allow(dead_code)]\npub(crate) fn f(x: &u8) -> u8 { let y = g(1, 2); let s = S { a: 1 }; let z = x.to_string(); let w = y.m(); vec![1]; crate::a::b(); let q = w?; self::m(); let u = a::b::c; q }\nimpl X { fn g(&self) {} }\nimpl T for X { fn t(&self) { self.g(); Self::g(self); } }\n").unwrap();
        visitor.visit_file(&file);
    }
}
