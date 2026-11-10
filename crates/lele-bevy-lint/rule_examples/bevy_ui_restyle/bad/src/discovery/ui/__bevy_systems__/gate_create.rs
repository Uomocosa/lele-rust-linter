pub fn build(app: &mut App) {
    app.add_systems(Update, gate_create);
}

pub fn gate_create(button: Query<&mut BackgroundColor>) {
    let _ = button;
}
