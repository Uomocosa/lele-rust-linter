pub struct Query;

pub fn tick_enemies(_q: Query) {}

#[cfg(test)]
mod tests {
    use super::Query;
    use super::tick_enemies;

    #[test]
    fn tick_enemies_ui_png_preview() {
        tick_enemies(Query);
    }

    #[test]
    fn extra_regression() {
        tick_enemies(Query);
    }
}
