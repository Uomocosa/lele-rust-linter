use syn::visit::Visit;

use crate::scan;

#[must_use]
pub fn collect_previews(file: &scan::ScannedFile) -> Vec<scan::Preview> {
    let mut previews = Vec::new();
    let mut finder = scan::PreviewFinder {
        previews: &mut previews,
        is_run_imported: file.imports_harness_run(),
        module_fns: scan::ScannedFile::module_fns_of(&file.items),
    };
    finder.visit_file(file);
    previews
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
