pub fn setup(s: &mut S) { s.spawn((Node,)); }

#[cfg(test)]
mod tests {
    use lele_bevy_preview::run;

    #[test]
    fn room_row_ui_png_preview() {
        let _ = run(&scene(), &Config::default(), "x");
    }
}
