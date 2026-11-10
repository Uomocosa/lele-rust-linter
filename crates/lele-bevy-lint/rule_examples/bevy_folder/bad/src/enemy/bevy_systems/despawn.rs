pub struct Query;

pub fn despawn(_q: Query) {}

pub fn register_despawn() {
    let app = App;
    app.add_systems(Update, despawn);
}
