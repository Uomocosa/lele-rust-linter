pub struct Query;

pub fn tick_beehive(_q: Query) {}

pub fn register_beehive() {
    let app = App;
    app.add_systems(Update, tick_beehive);
}
