use std::path::PathBuf;

use lele_lint::Project;
use lele_snake_case::ToLeleSnakeCase;

use crate::scan;

pub fn plugin_registrations(file: &scan::ScannedFile, project: &Project) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    let module_fns = file.module_fns();
    for impl_block in file.items.iter().filter_map(|item| match item {
        syn::Item::Impl(impl_block) => Some(impl_block),
        _ => None,
    }) {
        let Some(plugin) = scan::PluginImpl(impl_block).plugin_type() else {
            continue;
        };
        let delegate_dir = PathBuf::from(plugin.to_snake_case());
        for method in impl_block.items.iter().filter_map(|item| match item {
            syn::ImplItem::Fn(method) if method.sig.ident == "build" => Some(method),
            _ => None,
        }) {
            let build_region = scan::ExprRegion {
                exprs: vec![scan::ScannedFile::block_expr(&method.block)],
                module_fns: &module_fns,
            };
            paths.extend(build_region.registered_paths());
            for delegate in scan::ScannedFile::self_calls(&method.block) {
                let rel_path = delegate_dir.join(format!("{delegate}.rs"));
                let Some(delegate_file) = project.methods_parsed_files.get(&rel_path) else {
                    continue;
                };
                let bodies: Vec<syn::Expr> = delegate_file
                    .items
                    .iter()
                    .filter_map(|item| match item {
                        syn::Item::Fn(func) => Some(scan::ScannedFile::block_expr(&func.block)),
                        _ => None,
                    })
                    .collect();
                let delegate_scanned = scan::ScannedFile(delegate_file);
                let delegate_fns = delegate_scanned.module_fns();
                let delegate_region = scan::ExprRegion {
                    exprs: bodies,
                    module_fns: &delegate_fns,
                };
                paths.extend(delegate_region.registered_paths());
            }
        }
    }
    paths
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::plugin_registrations;
    use crate::scan;
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
        let paths = plugin_registrations(&scan::ScannedFile(&file), &project);
        assert!(paths.contains(&vec!["tick_hud".to_string()]));
        assert!(paths.contains(&vec![
            "hud".to_string(),
            "bevy_systems".to_string(),
            "spawn_hud".to_string(),
        ]));
    }
}
