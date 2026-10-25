use derive_more::Deref;

#[derive(Deref)]
pub struct Rooms(pub Vec<String>);
