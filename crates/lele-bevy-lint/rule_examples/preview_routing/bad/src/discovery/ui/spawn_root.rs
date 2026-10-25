pub fn setup(s: &mut S) { s.spawn((Node,)); }

#[cfg(test)]
mod tests {
    #[test]
    fn spawn_root_ui_png_preview() {
        let shot = std::path::PathBuf::from("spawn_root.png");
        assert!(shot.exists());
    }
}
