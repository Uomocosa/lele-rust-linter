use crate::scan;

#[must_use]
pub fn scene_usage(body: &syn::Block, module_fns: &scan::ModuleFns) -> scan::SceneUsage {
    let start = syn::Expr::Block(syn::ExprBlock {
        attrs: Vec::new(),
        label: None,
        block: body.clone(),
    });
    let region = scan::closure_over(start, module_fns);
    let mut usage = scan::SceneUsage::default();
    for scene in scan::scene_literals(&region) {
        if let Some(build) = field(&scene, "build") {
            let reached = scan::closure_over(build.clone(), module_fns);
            usage.build_uses.extend(scan::build_uses(&reached));
        }
        if let Some(states) = field(&scene, "states") {
            let reached = scan::closure_over(states.clone(), module_fns);
            usage.state_count = usage.state_count.max(scan::count_states(&reached));
        }
    }
    usage.build_uses.sort();
    usage.build_uses.dedup();
    usage
}

// needed helper: one named field of a struct literal (shorthand `build,` included)
fn field<'a>(literal: &'a syn::ExprStruct, name: &str) -> Option<&'a syn::Expr> {
    literal
        .fields
        .iter()
        .find(|value| matches!(&value.member, syn::Member::Named(ident) if ident == name))
        .map(|value| &value.expr)
}

#[cfg(test)]
mod tests {
    use super::scene_usage;
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
        let usage = scene_usage(&body, &module_fns);
        assert!(usage.build_uses.contains(&"gate_create".to_string()));
        assert_eq!(usage.state_count, 2);
    }
}
