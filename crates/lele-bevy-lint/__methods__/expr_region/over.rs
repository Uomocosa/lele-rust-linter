use std::collections::HashSet;

use crate::scan;

pub fn over<'a>(start: syn::Expr, module_fns: &'a scan::ModuleFns<'a>) -> scan::ExprRegion<'a> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut pending = vec![start.clone()];
    let mut exprs = vec![start];
    while let Some(expr) = pending.pop() {
        for name in scan::ExprRegion::local_names(&expr) {
            let Some(func) = module_fns.get(&name) else {
                continue;
            };
            if !seen.insert(name) {
                continue;
            }
            let body = syn::Expr::Block(syn::ExprBlock {
                attrs: Vec::new(),
                label: None,
                block: (*func.block).clone(),
            });
            pending.push(body.clone());
            exprs.push(body);
        }
    }
    scan::ExprRegion { exprs, module_fns }
}

#[cfg(test)]
mod tests {
    use super::over;
    use crate::scan;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_str(
            "fn scene() -> Scene { Scene { build: preview_build } }
             fn preview_build(app: &mut App) { app.add_systems(Startup, super::spawn_root); }
             fn unrelated() {}",
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
        let start: syn::Expr = syn::parse_str("run(&scene(), &config)").unwrap();
        assert_eq!(over(start, &module_fns).exprs.len(), 3);
    }
}
