pub struct Query;

pub fn tick_enemies(_q: Query) {}

pub fn register_enemies() {
    let app = App;
    app.add_systems(Update, tick_enemies);
}
