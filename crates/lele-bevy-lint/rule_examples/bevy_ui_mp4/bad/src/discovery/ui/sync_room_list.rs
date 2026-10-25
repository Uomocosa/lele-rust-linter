pub fn setup(s: &mut S) { s.spawn((Node,)); }

pub fn tick(time: Res<Time>) {
    let _ = time.delta_secs();
}
