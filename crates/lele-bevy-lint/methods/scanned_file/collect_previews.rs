use syn::spanned::Spanned;
use syn::visit::Visit;

use crate::scan;

const PNG_SUFFIX: &str = "_ui_png_preview";
const MP4_SUFFIX: &str = "_ui_mp4_preview";
const SCENE_SUFFIX: &str = "_ui_scene_preview";

#[must_use]
pub fn collect_previews(file: &scan::ScannedFile) -> Vec<scan::Preview> {
    let mut previews = Vec::new();
    let mut finder = PreviewFinder {
        previews: &mut previews,
        is_run_imported: file.imports_harness_run(),
        module_fns: module_fns_of(&file.items),
    };
    finder.visit_file(file);
    previews
}

// needed helper: matches one preview fn into a Preview record, scoped to its module's fns
struct PreviewFinder<'a, 'ast> {
    previews: &'a mut Vec<scan::Preview>,
    is_run_imported: bool,
    module_fns: scan::ModuleFns<'ast>,
}

impl<'ast> Visit<'ast> for PreviewFinder<'_, 'ast> {
    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        let Some((_, items)) = &node.content else {
            return;
        };
        let outer = std::mem::replace(&mut self.module_fns, module_fns_of(items));
        syn::visit::visit_item_mod(self, node);
        self.module_fns = outer;
    }

    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        let name = node.sig.ident.to_string();
        if let Some(kind) = kind_of(&name) {
            self.previews.push(scan::Preview {
                name,
                kind,
                line: node.sig.fn_token.span().start().line,
                is_test: node.attrs.iter().any(|attr| attr.path().is_ident("test")),
                harness: scan::HarnessCall::detect(&node.block, self.is_run_imported),
                scene: scan::SceneUsage::detect(&node.block, &self.module_fns),
            });
        }
        syn::visit::visit_item_fn(self, node);
    }
}

// needed helper: preview kind from the fn-name suffix
fn kind_of(name: &str) -> Option<scan::PreviewKind> {
    if name.ends_with(PNG_SUFFIX) {
        Some(scan::PreviewKind::Png)
    } else if name.ends_with(MP4_SUFFIX) {
        Some(scan::PreviewKind::Mp4)
    } else if name.ends_with(SCENE_SUFFIX) {
        Some(scan::PreviewKind::Scene)
    } else {
        None
    }
}

// needed helper: the fns declared directly in one module, by name
fn module_fns_of(items: &[syn::Item]) -> scan::ModuleFns<'_> {
    items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Fn(func) => Some((func.sig.ident.to_string(), func)),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_previews;
    use crate::scan;

    #[test]
    fn test_usage() {
        let file = syn::parse_str(
            "pub fn spawn_root(mut c: Commands) { c.spawn((Node,)); }
             #[cfg(test)]
             mod tests {
                 use lele_bevy_preview::{run, scene::Scene};
                 fn build(app: &mut App) { app.add_systems(Startup, super::spawn_root); }
                 #[test]
                 fn spawn_root_ui_png_preview() {
                     let scene = Scene { build, states: vec![State { label, apply: rest }] };
                     run(&scene, &Config::default(), \"x\").expect(\"preview\");
                 }
             }",
        )
        .unwrap();
        let previews = collect_previews(&scan::ScannedFile(&file));
        assert_eq!(previews.len(), 1);
        assert!(previews[0].is_test);
        assert_eq!(previews[0].harness, scan::HarnessCall::Checked);
        assert!(
            previews[0]
                .scene
                .build_uses
                .contains(&"spawn_root".to_string())
        );
        assert_eq!(previews[0].scene.state_count, 1);
    }

    #[test]
    fn test_usage_without_harness_is_not_routed() {
        let file = syn::parse_str(
            "#[cfg(test)]
             mod tests {
                 fn wheel_ui_png_preview() {
                     let _ = run(&x, &y, \"z\");
                 }
             }",
        )
        .unwrap();
        let previews = collect_previews(&scan::ScannedFile(&file));
        assert_eq!(previews.len(), 1);
        assert!(!previews[0].is_test);
        assert_eq!(previews[0].harness, scan::HarnessCall::Missing);
    }
}
