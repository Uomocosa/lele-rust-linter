use crate::scan;

#[must_use]
pub fn from_fn_body(body: &syn::Block, module_fns: &scan::ModuleFns) -> scan::SceneUsage {
    let start = syn::Expr::Block(syn::ExprBlock {
        attrs: Vec::new(),
        label: None,
        block: body.clone(),
    });
    let region = scan::ExprRegion::over(start, module_fns);
    let mut usage = scan::SceneUsage::default();
    for scene in region.scene_literals() {
        let kind = scan::SceneUsage::field(&scene, "kind").and_then(scan::SceneUsage::kind_name);
        let centered = scan::SceneUsage::field(&scene, "build").is_some_and(|build| {
            let reached = scan::ExprRegion::over(build.clone(), module_fns);
            usage.build_uses.extend(reached.build_uses());
            reached.centers_preview()
        });
        if let Some(kind) = kind.filter(|_| !centered) {
            usage.uncentered_kinds.push(kind);
        }
        if let Some(states) = scan::SceneUsage::field(&scene, "states") {
            let reached = scan::ExprRegion::over(states.clone(), module_fns);
            usage.state_count = usage.state_count.max(reached.count_states());
        }
    }
    usage.build_uses.sort();
    usage.build_uses.dedup();
    usage
}

#[cfg(test)]
mod tests {
    use super::from_fn_body;
    use crate::scan;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_str(
            "fn build(app: &mut App) { app.add_systems(Startup, super::gate_create); }
             fn scene() -> Scene {
                 Scene {
                     build,
                     states: vec![
                         State { label: String::from(\"off\"), apply: rest },
                         State { label: String::from(\"on\"), apply: type_name },
                     ],
                 }
             }",
        )
        .unwrap();
        let module_fns: scan::ModuleFns = file
            .items
            .iter()
            .filter_map(|item| match item {
                syn::Item::Fn(func) => Some((func.sig.ident.to_string(), func)),
                _ => None,
            })
            .collect();
        let body: syn::Block =
            syn::parse_str("{ run(&scene(), &config(), \"x\").expect(\"preview\"); }").unwrap();
        let usage = from_fn_body(&body, &module_fns);
        assert!(usage.build_uses.contains(&"gate_create".to_string()));
        assert_eq!(usage.state_count, 2);
    }
}
