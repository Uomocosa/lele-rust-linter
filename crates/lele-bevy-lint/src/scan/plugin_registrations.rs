use std::path::PathBuf;

use derive_more::{Deref, DerefMut};
use lele_lint::Project;
use syn::visit::Visit;

use crate::scan;

#[must_use]
pub fn plugin_registrations(project: &Project, file: &syn::File) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    for impl_block in file.items.iter().filter_map(|item| match item {
        syn::Item::Impl(impl_block) => Some(impl_block),
        _ => None,
    }) {
        let Some(plugin) = scan::plugin_type_of(impl_block) else {
            continue;
        };
        let delegate_dir = PathBuf::from(lele_snake_case::to_snake_case(&plugin));
        for method in impl_block.items.iter().filter_map(|item| match item {
            syn::ImplItem::Fn(method) if method.sig.ident == "build" => Some(method),
            _ => None,
        }) {
            paths.extend(scan::registered_paths(&[block_expr(&method.block)]));
            for delegate in self_calls(&method.block) {
                let rel_path = delegate_dir.join(format!("{delegate}.rs"));
                let Some(delegate_file) = project.methods_parsed_files.get(&rel_path) else {
                    continue;
                };
                let bodies: Vec<syn::Expr> = delegate_file
                    .items
                    .iter()
                    .filter_map(|item| match item {
                        syn::Item::Fn(func) => Some(block_expr(&func.block)),
                        _ => None,
                    })
                    .collect();
                paths.extend(scan::registered_paths(&bodies));
            }
        }
    }
    paths
}

// needed helper: a block as an expression, so one visitor walks fn bodies and exprs alike
fn block_expr(block: &syn::Block) -> syn::Expr {
    syn::Expr::Block(syn::ExprBlock {
        attrs: Vec::new(),
        label: None,
        block: block.clone(),
    })
}

// needed helper: methods `build` calls on `self`, i.e. its atomic delegates
fn self_calls(block: &syn::Block) -> Vec<String> {
    let mut names = Vec::new();
    let mut collector = SelfCalls(&mut names);
    collector.visit_block(block);
    names
}

// needed helper: records `self.<method>(..)` names
#[derive(Deref, DerefMut)]
struct SelfCalls<'a>(&'a mut Vec<String>);

impl<'ast> Visit<'ast> for SelfCalls<'_> {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if let syn::Expr::Path(receiver) = &*node.receiver
            && receiver.path.is_ident("self")
        {
            self.push(node.method.to_string());
        }
        syn::visit::visit_expr_method_call(self, node);
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::plugin_registrations;
    use lele_lint::Project;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.methods_parsed_files.insert(
            PathBuf::from("hud_plugin/build_plugin.rs"),
            syn::parse_str(
                "pub fn build_plugin(_p: &HudPlugin, app: &mut App) {
                     app.add_systems(Startup, hud::bevy_systems::spawn_hud);
                 }",
            )
            .unwrap(),
        );
        let file = syn::parse_str(
            "impl Plugin for HudPlugin {
                 fn build(&self, app: &mut App) {
                     app.add_systems(Update, tick_hud);
                     self.build_plugin(app);
                 }
             }",
        )
        .unwrap();
        let paths = plugin_registrations(&project, &file);
        assert!(paths.contains(&vec!["tick_hud".to_string()]));
        assert!(paths.contains(&vec![
            "hud".to_string(),
            "bevy_systems".to_string(),
            "spawn_hud".to_string(),
        ]));
    }
}
