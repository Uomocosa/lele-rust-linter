pub(crate) fn preview_source(kind: &str, build_body: &str) -> String {
    format!(
        "#[cfg(test)]
         mod tests {{
             fn spawn_rows(mut commands: Commands) {{ commands.spawn((Node::default(),)); }}
             fn build(app: &mut App) {{ {build_body} }}
             #[test]
             fn rows_ui_png_preview() {{
                 let scene = Scene {{ kind: Kind::{kind}, build, states: vec![] }};
                 lele_bevy_preview::run(&scene, &c).expect(\"preview\");
             }}
         }}"
    )
}

#[cfg(test)]
mod tests {
    use super::preview_source;

    #[test]
    fn test_usage() {
        let source = preview_source("System", "app.add_systems(Startup, spawn_rows);");
        assert!(source.contains("Kind::System"));
        assert!(source.contains("app.add_systems(Startup, spawn_rows);"));
    }
}
